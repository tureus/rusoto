#![doc(html_logo_url = "https://raw.githubusercontent.com/rusoto/rusoto/master/assets/logo-square.png")]
#![cfg_attr(feature = "nightly-testing", feature(plugin))]
#![cfg_attr(feature = "nightly-testing", plugin(clippy))]
#![cfg_attr(not(feature = "unstable"), deny(warnings))]
#![deny(missing_docs)]

//! Types for loading and managing AWS access credentials for API requests.

extern crate chrono;
#[macro_use]
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate regex;
extern crate serde_json;
extern crate tokio_core;

pub use environment::EnvironmentProvider;
pub use container::ContainerProvider;
pub use static_provider::StaticProvider;
pub use instance_metadata::InstanceMetadataProvider;
pub use profile::ProfileProvider;

mod request;
mod container;
mod environment;
mod static_provider;
mod instance_metadata;
mod profile;
pub(crate) mod test_utils;
pub mod claims;

use std::env::var as env_var;
use std::fmt;
use std::error::Error;
use std::io::Error as IoError;
use std::ops::Deref;
use std::sync::Mutex;
use std::time::Duration;
use std::cell::RefCell;
use std::collections::BTreeMap;

use chrono::{Duration as ChronoDuration, Utc, DateTime, ParseError};
use futures::{Async, Future, Poll};
use futures::future::{Either, Shared, SharedItem, err};
use hyper::{Error as HyperError};
use hyper::error::UriError;
use serde_json::{from_str as json_from_str, Value};
use tokio_core::reactor::Handle;

/// AWS API access credentials, including access key, secret key, token (for IAM profiles),
/// expiration timestamp, and claims from federated login.
#[derive(Clone)]
pub struct AwsCredentials {
    key: String,
    secret: String,
    token: Option<String>,
    expires_at: Option<DateTime<Utc>>,
    claims: BTreeMap<String, String>,
}

impl AwsCredentials {
    /// Create a new `AwsCredentials` from a key ID, secret key, optional access token, and expiry
    /// time.
    pub fn new<K, S>(
        key: K,
        secret: S,
        token: Option<String>,
        expires_at: Option<DateTime<Utc>>,
    ) -> AwsCredentials
    where
        K: Into<String>,
        S: Into<String>,
    {
        AwsCredentials {
            key: key.into(),
            secret: secret.into(),
            token: token,
            expires_at: expires_at,
            claims: BTreeMap::new(),
        }
    }

    /// Get a reference to the access key ID.
    pub fn aws_access_key_id(&self) -> &str {
        &self.key
    }

    /// Get a reference to the secret access key.
    pub fn aws_secret_access_key(&self) -> &str {
        &self.secret
    }

    /// Get a reference to the expiry time.
    pub fn expires_at(&self) -> &Option<DateTime<Utc>> {
        &self.expires_at
    }

    /// Get a reference to the access token.
    pub fn token(&self) -> &Option<String> {
        &self.token
    }

    /// Determine whether or not the credentials are expired.
    fn credentials_are_expired(&self) -> bool {
        match self.expires_at {
            Some(ref e) =>
                // This is a rough hack to hopefully avoid someone requesting creds then sitting on them
                // before issuing the request:
               *e < Utc::now() + ChronoDuration::seconds(20),
            None => false,
        }
    }

    /// Get the token claims
    pub fn claims(&self) -> &BTreeMap<String, String> {
        &self.claims
    }

    /// Get the mutable token claims
    pub fn claims_mut(&mut self) -> &mut BTreeMap<String, String> {
        &mut self.claims
    }
}

impl fmt::Debug for AwsCredentials {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AwsCredentials")
            .field("key", &self.key)
            .field("secret", &"**********")
            .field("token", &self.token.as_ref().map(|_| "**********"))
            .field("expires_at", &self.expires_at)
            .field("claims", &self.claims)
            .finish()
    }
}

/// Represents an Error that has occured during the fetching Credentials Phase.
///
/// This generally is an error message from one of our underlying libraries, however
/// we wrap it up with this type so we can export one single error type.
#[derive(Debug, PartialEq)]
pub struct CredentialsError {
    /// The underlying error message for the credentials error.
    pub message: String,
}

impl CredentialsError {
    /// Creates a new Credentials Error.
    ///
    /// * `message` - The Error message for this CredentialsError.
    pub fn new<S>(message: S) -> CredentialsError
    where
        S: Into<String>,
    {
        CredentialsError { message: message.into() }
    }
}

impl fmt::Display for CredentialsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for CredentialsError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl From<ParseError> for CredentialsError {
    fn from(err: ParseError) -> CredentialsError {
        CredentialsError::new(err.description())
    }
}

impl From<IoError> for CredentialsError {
    fn from(err: IoError) -> CredentialsError {
        CredentialsError::new(err.description())
    }
}

impl From<UriError> for CredentialsError {
    fn from(err: UriError) -> CredentialsError {
        CredentialsError::new(err.description())
    }
}

impl From<HyperError> for CredentialsError {
    fn from(err: HyperError) -> CredentialsError {
        CredentialsError::new(
            format!("Couldn't connect to credentials provider: {}", err),
        )
    }
}

/// A trait for types that produce `AwsCredentials`.
pub trait ProvideAwsCredentials {
    /// The future response value.
    type Future: Future<Item=AwsCredentials, Error=CredentialsError> + 'static;

    /// Produce a new `AwsCredentials` future.
    fn credentials(&self) -> Self::Future;
}

/// Wrapper for `ProvideAwsCredentials` that caches the credentials returned by the
/// wrapped provider.  Each time the credentials are accessed, they are checked to see if
/// they have expired, in which case they are retrieved from the wrapped provider again.
///
/// In order to access the wrapped provider, for instance to set a timeout, the `get_ref`
/// and `get_mut` methods can be used.
#[derive(Debug)]
pub struct BaseAutoRefreshingProvider<P: ProvideAwsCredentials + 'static, T> {
    credentials_provider: P,
    shared_future: T,
}

impl<P: ProvideAwsCredentials + 'static, T> BaseAutoRefreshingProvider<P, T> {
    /// Get a shared reference to the wrapped provider.
    pub fn get_ref(&self) -> &P {
        &self.credentials_provider
    }

    /// Get a mutable reference to the wrapped provider.
    ///
    /// This can be used to call `set_timeout` on the wrapped
    /// provider.
    pub fn get_mut(&mut self) -> &mut P {
        &mut self.credentials_provider
    }
}

enum AutoRefreshingFutureInner<P: ProvideAwsCredentials + 'static> {
    Cached(SharedItem<AwsCredentials>),
    NotCached(Shared<P::Future>)
}

impl<P: ProvideAwsCredentials + 'static> AutoRefreshingFutureInner<P> {
    fn from_shared_future(future: &mut Shared<P::Future>, provider: &P) -> Self {
        match future.peek() {
            // no result from the future yet, let's keep using it
            None => AutoRefreshingFutureInner::NotCached(future.clone()),
            // successful result from the future, use it if not expired
            Some(Ok(ref creds)) if !creds.credentials_are_expired() =>
                AutoRefreshingFutureInner::Cached(creds.clone()),
            Some(_) => {
                // else launch a new future
                *future = provider.credentials().shared();
                AutoRefreshingFutureInner::NotCached(future.clone())
            }
        }
    }
}

impl<P: ProvideAwsCredentials + 'static> Clone for AutoRefreshingFutureInner<P> {
    fn clone(&self) -> Self {
        match *self {
            AutoRefreshingFutureInner::Cached(ref shared_item) =>
                AutoRefreshingFutureInner::Cached(shared_item.clone()),
            AutoRefreshingFutureInner::NotCached(ref shared_future) =>
                AutoRefreshingFutureInner::NotCached(shared_future.clone())
        }
    }
}

/// Future returned from `AutoRefreshingProvider`.
pub struct AutoRefreshingProviderFuture<P: ProvideAwsCredentials + 'static> {
    inner: AutoRefreshingFutureInner<P>
}

impl<P: ProvideAwsCredentials + 'static> Future for AutoRefreshingProviderFuture<P> {
    type Item = AwsCredentials;
    type Error = CredentialsError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.inner {
            AutoRefreshingFutureInner::Cached(ref creds) => Ok(Async::Ready(creds.deref().clone())),
            AutoRefreshingFutureInner::NotCached(ref mut future) => {
                match future.poll() {
                    Err(err) => Err(CredentialsError { message: err.message.to_owned() }),
                    Ok(Async::NotReady) => Ok(Async::NotReady),
                    Ok(Async::Ready(item)) => Ok(Async::Ready(item.deref().clone()))
                }
            }
        }
    }
}

/// Threadsafe `AutoRefreshingProvider` that locks cached credentials with a `Mutex`
pub type AutoRefreshingProviderSync<P: ProvideAwsCredentials + 'static> =
    BaseAutoRefreshingProvider<P, Mutex<Shared<P::Future>>>;

impl<P: ProvideAwsCredentials + 'static> AutoRefreshingProviderSync<P> {
    /// Grab a RefreshingProvider that locks it's credentials with a Mutex so it's thread safe.
    pub fn with_mutex(provider: P) -> Result<AutoRefreshingProviderSync<P>, CredentialsError> {
        let future = provider.credentials();
        Ok(BaseAutoRefreshingProvider {
            credentials_provider: provider,
            shared_future: Mutex::new(future.shared()),
        })
    }
}

impl<P: ProvideAwsCredentials + 'static> ProvideAwsCredentials for AutoRefreshingProviderSync<P> {
    type Future = AutoRefreshingProviderFuture<P>;

    fn credentials(&self) -> Self::Future {
        let mut shared_future = self.shared_future.lock().expect(
            "Failed to lock the cached credentials Mutex",
        );
        AutoRefreshingProviderFuture {
            inner: AutoRefreshingFutureInner::from_shared_future(&mut shared_future, &self.credentials_provider)
        }
    }
}

/// `!Sync` `AutoRefreshingProvider` that caches credentials in a `RefCell`
pub type AutoRefreshingProvider<P: ProvideAwsCredentials + 'static> =
    BaseAutoRefreshingProvider<P, RefCell<Shared<P::Future>>>;

impl<P: ProvideAwsCredentials + 'static> AutoRefreshingProvider<P> {
    /// Grab a provider that locks it's credentials with a RefCell. If you're looking for
    /// Thread Safety, take a look at AutoRefreshingProviderSync.
    pub fn with_refcell(provider: P) -> Result<AutoRefreshingProvider<P>, CredentialsError> {
        let future = provider.credentials();
        Ok(BaseAutoRefreshingProvider {
            credentials_provider: provider,
            shared_future: RefCell::new(future.shared()),
        })
    }
}

impl<P: ProvideAwsCredentials + 'static> ProvideAwsCredentials for AutoRefreshingProvider<P> {
    type Future = AutoRefreshingProviderFuture<P>;

    fn credentials(&self) -> Self::Future {
        let mut shared_future = self.shared_future.borrow_mut();
        AutoRefreshingProviderFuture {
            inner: AutoRefreshingFutureInner::from_shared_future(&mut shared_future, &self.credentials_provider)
        }
    }
}

/// The credentials provider you probably want to use if you don't require Sync for your AWS services.
/// Wraps a `ChainProvider` in an `AutoRefreshingProvider` that uses a `RefCell` to cache credentials
///
/// The underlying `ChainProvider` checks multiple sources for credentials, and the `AutoRefreshingProvider`
/// refreshes the credentials automatically when they expire.  The `RefCell` allows this caching to happen
/// without the overhead of a `Mutex`, but is `!Sync`.
///
/// For a `Sync` implementation of the same, see `DefaultCredentialsProviderSync`
pub type DefaultCredentialsProvider = AutoRefreshingProvider<ChainProvider>;

impl DefaultCredentialsProvider {
    /// Creates a new DefaultCredentials Provider. If you're looking for
    /// Thread Safety look at DefaultCredentialsProviderSync.
    pub fn new(handle: &Handle) -> Result<DefaultCredentialsProvider, CredentialsError> {
        AutoRefreshingProvider::with_refcell(ChainProvider::new(handle))
    }
}

/// The credentials provider you probably want to use if you do require Sync for your AWS services.
/// Wraps a `ChainProvider` in an `AutoRefreshingProvider` that uses a `Mutex` to lock credentials in a
/// threadsafe manner.
///
/// The underlying `ChainProvider` checks multiple sources for credentials, and the `AutoRefreshingProvider`
/// refreshes the credentials automatically when they expire.  The `Mutex` allows this caching to happen
/// in a Sync manner, incurring the overhead of a Mutex when credentials expire and need to be refreshed.
///
/// For a `!Sync` implementation of the same, see `DefaultCredentialsProvider`
pub type DefaultCredentialsProviderSync = AutoRefreshingProviderSync<ChainProvider>;

impl DefaultCredentialsProviderSync {
    /// Creates a new Thread Safe Default Credentials Provider.
    pub fn new(handle: &Handle) -> Result<DefaultCredentialsProviderSync, CredentialsError> {
        AutoRefreshingProviderSync::with_mutex(ChainProvider::new(handle))
    }
}

/// Provides AWS credentials from multiple possible sources using a priority order.
///
/// The following sources are checked in order for credentials when calling `credentials`:
///
/// 1. Environment variables: `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY`
/// 2. AWS credentials file. Usually located at `~/.aws/credentials`.
/// 3. IAM instance profile. Will only work if running on an EC2 instance with an instance profile/role.
///
/// If the sources are exhausted without finding credentials, an error is returned.
///
/// The provider has a default timeout of 30 seconds. While it should work well for most setups,
/// you can change the timeout using the `set_timeout` method.
///
/// # Example
///
/// ```rust
/// extern crate rusoto_credential;
/// extern crate tokio_core;
///
/// use std::time::Duration;
///
/// use rusoto_credential::ChainProvider;
/// use tokio_core::reactor::Core;
///
/// fn main() {
///   let core = Core::new().unwrap();
///
///   let mut provider = ChainProvider::new(&core.handle());
///   // you can overwrite the default timeout like this:
///   provider.set_timeout(Duration::from_secs(60));
///
///   // ...
/// }
/// ```
#[derive(Debug, Clone)]
pub struct ChainProvider {
    instance_metadata_provider: InstanceMetadataProvider,
    container_provider: ContainerProvider,
    profile_provider: Option<ProfileProvider>,
}

impl ChainProvider {
    /// Set the timeout on the provider to the specified duration.
    pub fn set_timeout(&mut self, duration: Duration) {
        self.instance_metadata_provider.set_timeout(duration);
        self.container_provider.set_timeout(duration);
    }
}

/// Future returned from `ChainProvider`.
pub struct ChainProviderFuture {
    inner: Box<Future<Item=AwsCredentials, Error=CredentialsError>>
}

impl Future for ChainProviderFuture {
    type Item = AwsCredentials;
    type Error = CredentialsError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}

impl ProvideAwsCredentials for ChainProvider {
    type Future = ChainProviderFuture;

    fn credentials(&self) -> Self::Future {
        let profile_provider = self.profile_provider.clone();
        let instance_metadata_provider = self.instance_metadata_provider.clone();
        let container_provider = self.container_provider.clone();
        let future = EnvironmentProvider.credentials()
            .or_else(move |_| match profile_provider {
                Some(ref provider) => Either::A(provider.credentials()),
                None => Either::B(err(CredentialsError::new(""))),
            })
            .or_else(move |_| container_provider.credentials())
            .or_else(move |_| instance_metadata_provider.credentials())
            .or_else(|_| {
                Err(CredentialsError::new(
                    "Couldn't find AWS credentials in environment, credentials file, or IAM role.",
                ))
            });
        ChainProviderFuture {
            inner: Box::new(future)
        }
    }
}

impl ChainProvider {
    /// Create a new `ChainProvider` using a `ProfileProvider` with the default settings.
    pub fn new(handle: &Handle) -> ChainProvider {
        ChainProvider {
            profile_provider: ProfileProvider::new().ok(),
            instance_metadata_provider: InstanceMetadataProvider::new(handle),
            container_provider: ContainerProvider::new(handle)
        }
    }

    /// Create a new `ChainProvider` using the provided `ProfileProvider`.
    pub fn with_profile_provider(handle: &Handle, profile_provider: ProfileProvider) -> ChainProvider {
        ChainProvider {
            profile_provider: Some(profile_provider),
            instance_metadata_provider: InstanceMetadataProvider::new(handle),
            container_provider: ContainerProvider::new(handle)
        }
    }
}

/// This is a helper function as Option<T>::filter is not yet stable (see issue #45860).
/// <https://github.com/rust-lang/rfcs/issues/2036> also affects the implementation of this.
fn non_empty_env_var(name: &str) -> Option<String> {
    match env_var(name) {
        Ok(value) => if value.is_empty() { None } else { Some(value) },
        Err(_) => None
    }
}

/// Reduces Boilerplate on getting json values. Wraps `serde_json::Value.get(key)`.
fn extract_string_value_from_json(
    json_object: &Value,
    key: &str,
) -> Result<String, CredentialsError> {
    match json_object.get(key) {
        Some(v) => Ok(
            v.as_str()
                .expect(&format!("{} value was not a string", key))
                .to_owned(),
        ),
        None => Err(CredentialsError::new(
            format!("Couldn't find {} in response.", key),
        )),
    }
}

/// Parses the response from an AWS Metadata Service, either from an IAM Role, or a Container.
fn parse_credentials_from_aws_service(response: &str) -> Result<AwsCredentials, CredentialsError> {
    let json_object: Value = match json_from_str(response) {
        Ok(v) => v,
        Err(_) => {
            return Err(CredentialsError::new(
                "Couldn't parse credentials response body.",
            ))
        }
    };

    let access_key_id = try!(extract_string_value_from_json(&json_object, "AccessKeyId"));
    let secret_access_key = try!(extract_string_value_from_json(
        &json_object,
        "SecretAccessKey",
    ));
    let token = try!(extract_string_value_from_json(&json_object, "Token"));
    let expiration = try!(extract_string_value_from_json(&json_object, "Expiration"));

    let expiration = Some(try!(expiration.parse()));

    Ok(AwsCredentials::new(
        access_key_id,
        secret_access_key,
        Some(token),
        expiration,
    ))
}

#[cfg(test)]
#[macro_use] extern crate lazy_static;

#[cfg(test)]
#[macro_use] extern crate quickcheck;

#[cfg(test)]
mod tests {
    use std::io::Read;
    use std::fs::{self, File};
    use std::path::Path;

    use futures::Future;
    use test_utils::{is_secret_hidden_behind_asterisks, SECRET};

    use super::*;

    #[test]
    fn profile_provider_finds_right_credentials_in_file() {
        let profile_provider = ProfileProvider::with_configuration(
            "tests/sample-data/multiple_profile_credentials",
            "foo",
        );

        let credentials = profile_provider.credentials().wait().expect(
            "Failed to get credentials from profile provider using tests/sample-data/multiple_profile_credentials",
        );

        assert_eq!(credentials.aws_access_key_id(), "foo_access_key");
        assert_eq!(credentials.aws_secret_access_key(), "foo_secret_key");
    }

    #[test]
    fn parse_iam_task_credentials_sample_response() {
        fn read_file_to_string(file_path: &Path) -> String {
            match fs::metadata(file_path) {
                Ok(metadata) => {
                    if !metadata.is_file() {
                        panic!("Couldn't open file");
                    }
                }
                Err(_) => panic!("Couldn't stat file"),
            };

            let mut file = File::open(file_path).unwrap();
            let mut result = String::new();
            file.read_to_string(&mut result).ok();

            result
        }

        let response = read_file_to_string(Path::new(
            "tests/sample-data/iam_task_credentials_sample_response",
        ));

        let credentials = parse_credentials_from_aws_service(&response);

        assert!(credentials.is_ok());
        let credentials = credentials.unwrap();

        assert_eq!(credentials.aws_access_key_id(), "AKIAIOSFODNN7EXAMPLE");
        assert_eq!(
            credentials.aws_secret_access_key(),
            "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"
        );
    }

    #[test]
    quickcheck! {
        fn test_aws_credentials_secrets_not_in_debug(
            key: String,
            valid_for: Option<i64>,
            token: Option<()>
        ) -> bool {
            let creds = AwsCredentials::new(
                key,
                SECRET.to_owned(),
                token.map(|_| test_utils::SECRET.to_owned()),
                valid_for.map(|v| Utc::now() + ChronoDuration::seconds(v)),
            );
            is_secret_hidden_behind_asterisks(&creds)
        }
    }
}
