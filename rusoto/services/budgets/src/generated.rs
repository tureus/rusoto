// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

#[allow(warnings)]
use hyper::Client;
use hyper::status::StatusCode;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::region;

use std::fmt;
use std::error::Error;
use std::io;
use std::io::Read;
use rusoto_core::request::HttpDispatchError;
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::signature::SignedRequest;
use serde_json::Value as SerdeJsonValue;
use serde_json::from_str;
/// AWS Budget model
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Budget {
    #[serde(rename = "BudgetLimit")] pub budget_limit: Spend,
    #[serde(rename = "BudgetName")] pub budget_name: String,
    #[serde(rename = "BudgetType")] pub budget_type: String,
    #[serde(rename = "CalculatedSpend")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculated_spend: Option<CalculatedSpend>,
    #[serde(rename = "CostFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_filters: Option<::std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "CostTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_types: Option<CostTypes>,
    #[serde(rename = "TimePeriod")] pub time_period: TimePeriod,
    #[serde(rename = "TimeUnit")] pub time_unit: String,
}

/// A structure that holds the actual and forecasted spend for a budget.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CalculatedSpend {
    #[serde(rename = "ActualSpend")] pub actual_spend: Spend,
    #[serde(rename = "ForecastedSpend")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecasted_spend: Option<Spend>,
}

/// This includes the options for getting the cost of a budget.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CostTypes {
    /// A boolean value whether to include credits in the cost budget.
    #[serde(rename = "IncludeCredit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_credit: Option<bool>,
    /// A boolean value whether to include other subscription costs in the cost budget.
    #[serde(rename = "IncludeOtherSubscription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_other_subscription: Option<bool>,
    /// A boolean value whether to include recurring costs in the cost budget.
    #[serde(rename = "IncludeRecurring")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_recurring: Option<bool>,
    /// A boolean value whether to include refunds in the cost budget.
    #[serde(rename = "IncludeRefund")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_refund: Option<bool>,
    /// A boolean value whether to include subscriptions in the cost budget.
    #[serde(rename = "IncludeSubscription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subscription: Option<bool>,
    /// A boolean value whether to include support costs in the cost budget.
    #[serde(rename = "IncludeSupport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_support: Option<bool>,
    /// A boolean value whether to include tax in the cost budget.
    #[serde(rename = "IncludeTax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_tax: Option<bool>,
    /// A boolean value whether to include upfront costs in the cost budget.
    #[serde(rename = "IncludeUpfront")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_upfront: Option<bool>,
    /// A boolean value whether to use blended costs in the cost budget.
    #[serde(rename = "UseBlended")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_blended: Option<bool>,
}

/// Request of CreateBudget
#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateBudgetRequest {
    #[serde(rename = "AccountId")] pub account_id: String,
    #[serde(rename = "Budget")] pub budget: Budget,
    #[serde(rename = "NotificationsWithSubscribers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications_with_subscribers: Option<Vec<NotificationWithSubscribers>>,
}

/// Response of CreateBudget
#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateBudgetResponse;

/// Request of CreateNotification
#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateNotificationRequest {
    #[serde(rename = "AccountId")] pub account_id: String,
    #[serde(rename = "BudgetName")] pub budget_name: String,
    #[serde(rename = "Notification")] pub notification: Notification,
    #[serde(rename = "Subscribers")] pub subscribers: Vec<Subscriber>,
}

/// Response of CreateNotification
#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateNotificationResponse;

/// Request of CreateSubscriber
#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateSubscriberRequest {
    #[serde(rename = "AccountId")] pub account_id: String,
    #[serde(rename = "BudgetName")] pub budget_name: String,
    #[serde(rename = "Notification")] pub notification: Notification,
    #[serde(rename = "Subscriber")] pub subscriber: Subscriber,
}

/// Response of CreateSubscriber
#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateSubscriberResponse;

/// Request of DeleteBudget
#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteBudgetRequest {
    #[serde(rename = "AccountId")] pub account_id: String,
    #[serde(rename = "BudgetName")] pub budget_name: String,
}

/// Response of DeleteBudget
#[derive(Default, Debug, Clone, Deserialize)]
pub struct DeleteBudgetResponse;

/// Request of DeleteNotification
#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteNotificationRequest {
    #[serde(rename = "AccountId")] pub account_id: String,
    #[serde(rename = "BudgetName")] pub budget_name: String,
    #[serde(rename = "Notification")] pub notification: Notification,
}

/// Response of DeleteNotification
#[derive(Default, Debug, Clone, Deserialize)]
pub struct DeleteNotificationResponse;

/// Request of DeleteSubscriber
#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteSubscriberRequest {
    #[serde(rename = "AccountId")] pub account_id: String,
    #[serde(rename = "BudgetName")] pub budget_name: String,
    #[serde(rename = "Notification")] pub notification: Notification,
    #[serde(rename = "Subscriber")] pub subscriber: Subscriber,
}

/// Response of DeleteSubscriber
#[derive(Default, Debug, Clone, Deserialize)]
pub struct DeleteSubscriberResponse;

/// Request of DescribeBudget
#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeBudgetRequest {
    #[serde(rename = "AccountId")] pub account_id: String,
    #[serde(rename = "BudgetName")] pub budget_name: String,
}

/// Response of DescribeBudget
#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeBudgetResponse {
    #[serde(rename = "Budget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget: Option<Budget>,
}

/// Request of DescribeBudgets
#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeBudgetsRequest {
    #[serde(rename = "AccountId")] pub account_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// Response of DescribeBudgets
#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeBudgetsResponse {
    #[serde(rename = "Budgets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budgets: Option<Vec<Budget>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// Request of DescribeNotificationsForBudget
#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeNotificationsForBudgetRequest {
    #[serde(rename = "AccountId")] pub account_id: String,
    #[serde(rename = "BudgetName")] pub budget_name: String,
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// Response of GetNotificationsForBudget
#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeNotificationsForBudgetResponse {
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Notifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications: Option<Vec<Notification>>,
}

/// Request of DescribeSubscribersForNotification
#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeSubscribersForNotificationRequest {
    #[serde(rename = "AccountId")] pub account_id: String,
    #[serde(rename = "BudgetName")] pub budget_name: String,
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Notification")] pub notification: Notification,
}

/// Response of DescribeSubscribersForNotification
#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeSubscribersForNotificationResponse {
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Subscribers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<Subscriber>>,
}

/// Notification model. Each budget may contain multiple notifications with different settings.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    #[serde(rename = "ComparisonOperator")] pub comparison_operator: String,
    #[serde(rename = "NotificationType")] pub notification_type: String,
    #[serde(rename = "Threshold")] pub threshold: f64,
    #[serde(rename = "ThresholdType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_type: Option<String>,
}

/// A structure to relate notification and a list of subscribers who belong to the notification.
#[derive(Default, Debug, Clone, Serialize)]
pub struct NotificationWithSubscribers {
    #[serde(rename = "Notification")] pub notification: Notification,
    #[serde(rename = "Subscribers")] pub subscribers: Vec<Subscriber>,
}

/// A structure that represents either a cost spend or usage spend. Contains an amount and a unit.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Spend {
    #[serde(rename = "Amount")] pub amount: String,
    #[serde(rename = "Unit")] pub unit: String,
}

/// Subscriber model. Each notification may contain multiple subscribers with different addresses.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Subscriber {
    #[serde(rename = "Address")] pub address: String,
    #[serde(rename = "SubscriptionType")] pub subscription_type: String,
}

/// A time period indicating the start date and end date of a budget.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TimePeriod {
    #[serde(rename = "End")] pub end: f64,
    #[serde(rename = "Start")] pub start: f64,
}

/// Request of UpdateBudget
#[derive(Default, Debug, Clone, Serialize)]
pub struct UpdateBudgetRequest {
    #[serde(rename = "AccountId")] pub account_id: String,
    #[serde(rename = "NewBudget")] pub new_budget: Budget,
}

/// Response of UpdateBudget
#[derive(Default, Debug, Clone, Deserialize)]
pub struct UpdateBudgetResponse;

/// Request of UpdateNotification
#[derive(Default, Debug, Clone, Serialize)]
pub struct UpdateNotificationRequest {
    #[serde(rename = "AccountId")] pub account_id: String,
    #[serde(rename = "BudgetName")] pub budget_name: String,
    #[serde(rename = "NewNotification")] pub new_notification: Notification,
    #[serde(rename = "OldNotification")] pub old_notification: Notification,
}

/// Response of UpdateNotification
#[derive(Default, Debug, Clone, Deserialize)]
pub struct UpdateNotificationResponse;

/// Request of UpdateSubscriber
#[derive(Default, Debug, Clone, Serialize)]
pub struct UpdateSubscriberRequest {
    #[serde(rename = "AccountId")] pub account_id: String,
    #[serde(rename = "BudgetName")] pub budget_name: String,
    #[serde(rename = "NewSubscriber")] pub new_subscriber: Subscriber,
    #[serde(rename = "Notification")] pub notification: Notification,
    #[serde(rename = "OldSubscriber")] pub old_subscriber: Subscriber,
}

/// Response of UpdateSubscriber
#[derive(Default, Debug, Clone, Deserialize)]
pub struct UpdateSubscriberResponse;

/// Errors returned by CreateBudget
#[derive(Debug, PartialEq)]
pub enum CreateBudgetError {
    ///The exception is thrown when customer tries to create a record (e.g. budget), but the number this record already exceeds the limitation.
    CreationLimitExceeded(String),
    ///The exception is thrown when customer tries to create a record (e.g. budget) that already exists.
    DuplicateRecord(String),
    ///This exception is thrown on an unknown internal failure.
    InternalError(String),
    ///This exception is thrown if any request is given an invalid parameter. E.g., if a required Date field is null.
    InvalidParameter(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateBudgetError {
    pub fn from_body(body: &str) -> CreateBudgetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CreationLimitExceededException" => {
                        CreateBudgetError::CreationLimitExceeded(String::from(error_message))
                    }
                    "DuplicateRecordException" => {
                        CreateBudgetError::DuplicateRecord(String::from(error_message))
                    }
                    "InternalErrorException" => {
                        CreateBudgetError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        CreateBudgetError::InvalidParameter(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateBudgetError::Validation(error_message.to_string())
                    }
                    _ => CreateBudgetError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateBudgetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateBudgetError {
    fn from(err: serde_json::error::Error) -> CreateBudgetError {
        CreateBudgetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateBudgetError {
    fn from(err: CredentialsError) -> CreateBudgetError {
        CreateBudgetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateBudgetError {
    fn from(err: HttpDispatchError) -> CreateBudgetError {
        CreateBudgetError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateBudgetError {
    fn from(err: io::Error) -> CreateBudgetError {
        CreateBudgetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateBudgetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateBudgetError {
    fn description(&self) -> &str {
        match *self {
            CreateBudgetError::CreationLimitExceeded(ref cause) => cause,
            CreateBudgetError::DuplicateRecord(ref cause) => cause,
            CreateBudgetError::InternalError(ref cause) => cause,
            CreateBudgetError::InvalidParameter(ref cause) => cause,
            CreateBudgetError::Validation(ref cause) => cause,
            CreateBudgetError::Credentials(ref err) => err.description(),
            CreateBudgetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateBudgetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateNotification
#[derive(Debug, PartialEq)]
pub enum CreateNotificationError {
    ///The exception is thrown when customer tries to create a record (e.g. budget), but the number this record already exceeds the limitation.
    CreationLimitExceeded(String),
    ///The exception is thrown when customer tries to create a record (e.g. budget) that already exists.
    DuplicateRecord(String),
    ///This exception is thrown on an unknown internal failure.
    InternalError(String),
    ///This exception is thrown if any request is given an invalid parameter. E.g., if a required Date field is null.
    InvalidParameter(String),
    ///This exception is thrown if a requested entity is not found. E.g., if a budget id doesn't exist for an account ID.
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateNotificationError {
    pub fn from_body(body: &str) -> CreateNotificationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CreationLimitExceededException" => {
                        CreateNotificationError::CreationLimitExceeded(String::from(error_message))
                    }
                    "DuplicateRecordException" => {
                        CreateNotificationError::DuplicateRecord(String::from(error_message))
                    }
                    "InternalErrorException" => {
                        CreateNotificationError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        CreateNotificationError::InvalidParameter(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateNotificationError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateNotificationError::Validation(error_message.to_string())
                    }
                    _ => CreateNotificationError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateNotificationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateNotificationError {
    fn from(err: serde_json::error::Error) -> CreateNotificationError {
        CreateNotificationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateNotificationError {
    fn from(err: CredentialsError) -> CreateNotificationError {
        CreateNotificationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateNotificationError {
    fn from(err: HttpDispatchError) -> CreateNotificationError {
        CreateNotificationError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateNotificationError {
    fn from(err: io::Error) -> CreateNotificationError {
        CreateNotificationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateNotificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateNotificationError {
    fn description(&self) -> &str {
        match *self {
            CreateNotificationError::CreationLimitExceeded(ref cause) => cause,
            CreateNotificationError::DuplicateRecord(ref cause) => cause,
            CreateNotificationError::InternalError(ref cause) => cause,
            CreateNotificationError::InvalidParameter(ref cause) => cause,
            CreateNotificationError::NotFound(ref cause) => cause,
            CreateNotificationError::Validation(ref cause) => cause,
            CreateNotificationError::Credentials(ref err) => err.description(),
            CreateNotificationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateNotificationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateSubscriber
#[derive(Debug, PartialEq)]
pub enum CreateSubscriberError {
    ///The exception is thrown when customer tries to create a record (e.g. budget), but the number this record already exceeds the limitation.
    CreationLimitExceeded(String),
    ///The exception is thrown when customer tries to create a record (e.g. budget) that already exists.
    DuplicateRecord(String),
    ///This exception is thrown on an unknown internal failure.
    InternalError(String),
    ///This exception is thrown if any request is given an invalid parameter. E.g., if a required Date field is null.
    InvalidParameter(String),
    ///This exception is thrown if a requested entity is not found. E.g., if a budget id doesn't exist for an account ID.
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateSubscriberError {
    pub fn from_body(body: &str) -> CreateSubscriberError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CreationLimitExceededException" => {
                        CreateSubscriberError::CreationLimitExceeded(String::from(error_message))
                    }
                    "DuplicateRecordException" => {
                        CreateSubscriberError::DuplicateRecord(String::from(error_message))
                    }
                    "InternalErrorException" => {
                        CreateSubscriberError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        CreateSubscriberError::InvalidParameter(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateSubscriberError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateSubscriberError::Validation(error_message.to_string())
                    }
                    _ => CreateSubscriberError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateSubscriberError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateSubscriberError {
    fn from(err: serde_json::error::Error) -> CreateSubscriberError {
        CreateSubscriberError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateSubscriberError {
    fn from(err: CredentialsError) -> CreateSubscriberError {
        CreateSubscriberError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateSubscriberError {
    fn from(err: HttpDispatchError) -> CreateSubscriberError {
        CreateSubscriberError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateSubscriberError {
    fn from(err: io::Error) -> CreateSubscriberError {
        CreateSubscriberError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateSubscriberError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateSubscriberError {
    fn description(&self) -> &str {
        match *self {
            CreateSubscriberError::CreationLimitExceeded(ref cause) => cause,
            CreateSubscriberError::DuplicateRecord(ref cause) => cause,
            CreateSubscriberError::InternalError(ref cause) => cause,
            CreateSubscriberError::InvalidParameter(ref cause) => cause,
            CreateSubscriberError::NotFound(ref cause) => cause,
            CreateSubscriberError::Validation(ref cause) => cause,
            CreateSubscriberError::Credentials(ref err) => err.description(),
            CreateSubscriberError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateSubscriberError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBudget
#[derive(Debug, PartialEq)]
pub enum DeleteBudgetError {
    ///This exception is thrown on an unknown internal failure.
    InternalError(String),
    ///This exception is thrown if any request is given an invalid parameter. E.g., if a required Date field is null.
    InvalidParameter(String),
    ///This exception is thrown if a requested entity is not found. E.g., if a budget id doesn't exist for an account ID.
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteBudgetError {
    pub fn from_body(body: &str) -> DeleteBudgetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        DeleteBudgetError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DeleteBudgetError::InvalidParameter(String::from(error_message))
                    }
                    "NotFoundException" => DeleteBudgetError::NotFound(String::from(error_message)),
                    "ValidationException" => {
                        DeleteBudgetError::Validation(error_message.to_string())
                    }
                    _ => DeleteBudgetError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteBudgetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteBudgetError {
    fn from(err: serde_json::error::Error) -> DeleteBudgetError {
        DeleteBudgetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteBudgetError {
    fn from(err: CredentialsError) -> DeleteBudgetError {
        DeleteBudgetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBudgetError {
    fn from(err: HttpDispatchError) -> DeleteBudgetError {
        DeleteBudgetError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBudgetError {
    fn from(err: io::Error) -> DeleteBudgetError {
        DeleteBudgetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBudgetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBudgetError {
    fn description(&self) -> &str {
        match *self {
            DeleteBudgetError::InternalError(ref cause) => cause,
            DeleteBudgetError::InvalidParameter(ref cause) => cause,
            DeleteBudgetError::NotFound(ref cause) => cause,
            DeleteBudgetError::Validation(ref cause) => cause,
            DeleteBudgetError::Credentials(ref err) => err.description(),
            DeleteBudgetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteBudgetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteNotification
#[derive(Debug, PartialEq)]
pub enum DeleteNotificationError {
    ///This exception is thrown on an unknown internal failure.
    InternalError(String),
    ///This exception is thrown if any request is given an invalid parameter. E.g., if a required Date field is null.
    InvalidParameter(String),
    ///This exception is thrown if a requested entity is not found. E.g., if a budget id doesn't exist for an account ID.
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteNotificationError {
    pub fn from_body(body: &str) -> DeleteNotificationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        DeleteNotificationError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DeleteNotificationError::InvalidParameter(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteNotificationError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteNotificationError::Validation(error_message.to_string())
                    }
                    _ => DeleteNotificationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteNotificationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteNotificationError {
    fn from(err: serde_json::error::Error) -> DeleteNotificationError {
        DeleteNotificationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteNotificationError {
    fn from(err: CredentialsError) -> DeleteNotificationError {
        DeleteNotificationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteNotificationError {
    fn from(err: HttpDispatchError) -> DeleteNotificationError {
        DeleteNotificationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteNotificationError {
    fn from(err: io::Error) -> DeleteNotificationError {
        DeleteNotificationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteNotificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteNotificationError {
    fn description(&self) -> &str {
        match *self {
            DeleteNotificationError::InternalError(ref cause) => cause,
            DeleteNotificationError::InvalidParameter(ref cause) => cause,
            DeleteNotificationError::NotFound(ref cause) => cause,
            DeleteNotificationError::Validation(ref cause) => cause,
            DeleteNotificationError::Credentials(ref err) => err.description(),
            DeleteNotificationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteNotificationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteSubscriber
#[derive(Debug, PartialEq)]
pub enum DeleteSubscriberError {
    ///This exception is thrown on an unknown internal failure.
    InternalError(String),
    ///This exception is thrown if any request is given an invalid parameter. E.g., if a required Date field is null.
    InvalidParameter(String),
    ///This exception is thrown if a requested entity is not found. E.g., if a budget id doesn't exist for an account ID.
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteSubscriberError {
    pub fn from_body(body: &str) -> DeleteSubscriberError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        DeleteSubscriberError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DeleteSubscriberError::InvalidParameter(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteSubscriberError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteSubscriberError::Validation(error_message.to_string())
                    }
                    _ => DeleteSubscriberError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteSubscriberError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteSubscriberError {
    fn from(err: serde_json::error::Error) -> DeleteSubscriberError {
        DeleteSubscriberError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteSubscriberError {
    fn from(err: CredentialsError) -> DeleteSubscriberError {
        DeleteSubscriberError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteSubscriberError {
    fn from(err: HttpDispatchError) -> DeleteSubscriberError {
        DeleteSubscriberError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteSubscriberError {
    fn from(err: io::Error) -> DeleteSubscriberError {
        DeleteSubscriberError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteSubscriberError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSubscriberError {
    fn description(&self) -> &str {
        match *self {
            DeleteSubscriberError::InternalError(ref cause) => cause,
            DeleteSubscriberError::InvalidParameter(ref cause) => cause,
            DeleteSubscriberError::NotFound(ref cause) => cause,
            DeleteSubscriberError::Validation(ref cause) => cause,
            DeleteSubscriberError::Credentials(ref err) => err.description(),
            DeleteSubscriberError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteSubscriberError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeBudget
#[derive(Debug, PartialEq)]
pub enum DescribeBudgetError {
    ///This exception is thrown on an unknown internal failure.
    InternalError(String),
    ///This exception is thrown if any request is given an invalid parameter. E.g., if a required Date field is null.
    InvalidParameter(String),
    ///This exception is thrown if a requested entity is not found. E.g., if a budget id doesn't exist for an account ID.
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeBudgetError {
    pub fn from_body(body: &str) -> DescribeBudgetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        DescribeBudgetError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DescribeBudgetError::InvalidParameter(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DescribeBudgetError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeBudgetError::Validation(error_message.to_string())
                    }
                    _ => DescribeBudgetError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeBudgetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeBudgetError {
    fn from(err: serde_json::error::Error) -> DescribeBudgetError {
        DescribeBudgetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeBudgetError {
    fn from(err: CredentialsError) -> DescribeBudgetError {
        DescribeBudgetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeBudgetError {
    fn from(err: HttpDispatchError) -> DescribeBudgetError {
        DescribeBudgetError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeBudgetError {
    fn from(err: io::Error) -> DescribeBudgetError {
        DescribeBudgetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeBudgetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeBudgetError {
    fn description(&self) -> &str {
        match *self {
            DescribeBudgetError::InternalError(ref cause) => cause,
            DescribeBudgetError::InvalidParameter(ref cause) => cause,
            DescribeBudgetError::NotFound(ref cause) => cause,
            DescribeBudgetError::Validation(ref cause) => cause,
            DescribeBudgetError::Credentials(ref err) => err.description(),
            DescribeBudgetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeBudgetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeBudgets
#[derive(Debug, PartialEq)]
pub enum DescribeBudgetsError {
    ///This exception is thrown if the paging token is expired - past its TTL
    ExpiredNextToken(String),
    ///This exception is thrown on an unknown internal failure.
    InternalError(String),
    ///This exception is thrown if paging token signature didn't match the token, or the paging token isn't for this request
    InvalidNextToken(String),
    ///This exception is thrown if any request is given an invalid parameter. E.g., if a required Date field is null.
    InvalidParameter(String),
    ///This exception is thrown if a requested entity is not found. E.g., if a budget id doesn't exist for an account ID.
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeBudgetsError {
    pub fn from_body(body: &str) -> DescribeBudgetsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ExpiredNextTokenException" => {
                        DescribeBudgetsError::ExpiredNextToken(String::from(error_message))
                    }
                    "InternalErrorException" => {
                        DescribeBudgetsError::InternalError(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        DescribeBudgetsError::InvalidNextToken(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DescribeBudgetsError::InvalidParameter(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DescribeBudgetsError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeBudgetsError::Validation(error_message.to_string())
                    }
                    _ => DescribeBudgetsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeBudgetsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeBudgetsError {
    fn from(err: serde_json::error::Error) -> DescribeBudgetsError {
        DescribeBudgetsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeBudgetsError {
    fn from(err: CredentialsError) -> DescribeBudgetsError {
        DescribeBudgetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeBudgetsError {
    fn from(err: HttpDispatchError) -> DescribeBudgetsError {
        DescribeBudgetsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeBudgetsError {
    fn from(err: io::Error) -> DescribeBudgetsError {
        DescribeBudgetsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeBudgetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeBudgetsError {
    fn description(&self) -> &str {
        match *self {
            DescribeBudgetsError::ExpiredNextToken(ref cause) => cause,
            DescribeBudgetsError::InternalError(ref cause) => cause,
            DescribeBudgetsError::InvalidNextToken(ref cause) => cause,
            DescribeBudgetsError::InvalidParameter(ref cause) => cause,
            DescribeBudgetsError::NotFound(ref cause) => cause,
            DescribeBudgetsError::Validation(ref cause) => cause,
            DescribeBudgetsError::Credentials(ref err) => err.description(),
            DescribeBudgetsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeBudgetsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeNotificationsForBudget
#[derive(Debug, PartialEq)]
pub enum DescribeNotificationsForBudgetError {
    ///This exception is thrown if the paging token is expired - past its TTL
    ExpiredNextToken(String),
    ///This exception is thrown on an unknown internal failure.
    InternalError(String),
    ///This exception is thrown if paging token signature didn't match the token, or the paging token isn't for this request
    InvalidNextToken(String),
    ///This exception is thrown if any request is given an invalid parameter. E.g., if a required Date field is null.
    InvalidParameter(String),
    ///This exception is thrown if a requested entity is not found. E.g., if a budget id doesn't exist for an account ID.
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeNotificationsForBudgetError {
    pub fn from_body(body: &str) -> DescribeNotificationsForBudgetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ExpiredNextTokenException" => {
                        DescribeNotificationsForBudgetError::ExpiredNextToken(String::from(
                            error_message,
                        ))
                    }
                    "InternalErrorException" => {
                        DescribeNotificationsForBudgetError::InternalError(String::from(
                            error_message,
                        ))
                    }
                    "InvalidNextTokenException" => {
                        DescribeNotificationsForBudgetError::InvalidNextToken(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterException" => {
                        DescribeNotificationsForBudgetError::InvalidParameter(String::from(
                            error_message,
                        ))
                    }
                    "NotFoundException" => {
                        DescribeNotificationsForBudgetError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeNotificationsForBudgetError::Validation(error_message.to_string())
                    }
                    _ => DescribeNotificationsForBudgetError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeNotificationsForBudgetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeNotificationsForBudgetError {
    fn from(err: serde_json::error::Error) -> DescribeNotificationsForBudgetError {
        DescribeNotificationsForBudgetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeNotificationsForBudgetError {
    fn from(err: CredentialsError) -> DescribeNotificationsForBudgetError {
        DescribeNotificationsForBudgetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeNotificationsForBudgetError {
    fn from(err: HttpDispatchError) -> DescribeNotificationsForBudgetError {
        DescribeNotificationsForBudgetError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeNotificationsForBudgetError {
    fn from(err: io::Error) -> DescribeNotificationsForBudgetError {
        DescribeNotificationsForBudgetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeNotificationsForBudgetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeNotificationsForBudgetError {
    fn description(&self) -> &str {
        match *self {
            DescribeNotificationsForBudgetError::ExpiredNextToken(ref cause) => cause,
            DescribeNotificationsForBudgetError::InternalError(ref cause) => cause,
            DescribeNotificationsForBudgetError::InvalidNextToken(ref cause) => cause,
            DescribeNotificationsForBudgetError::InvalidParameter(ref cause) => cause,
            DescribeNotificationsForBudgetError::NotFound(ref cause) => cause,
            DescribeNotificationsForBudgetError::Validation(ref cause) => cause,
            DescribeNotificationsForBudgetError::Credentials(ref err) => err.description(),
            DescribeNotificationsForBudgetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeNotificationsForBudgetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeSubscribersForNotification
#[derive(Debug, PartialEq)]
pub enum DescribeSubscribersForNotificationError {
    ///This exception is thrown if the paging token is expired - past its TTL
    ExpiredNextToken(String),
    ///This exception is thrown on an unknown internal failure.
    InternalError(String),
    ///This exception is thrown if paging token signature didn't match the token, or the paging token isn't for this request
    InvalidNextToken(String),
    ///This exception is thrown if any request is given an invalid parameter. E.g., if a required Date field is null.
    InvalidParameter(String),
    ///This exception is thrown if a requested entity is not found. E.g., if a budget id doesn't exist for an account ID.
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeSubscribersForNotificationError {
    pub fn from_body(body: &str) -> DescribeSubscribersForNotificationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ExpiredNextTokenException" => {
                        DescribeSubscribersForNotificationError::ExpiredNextToken(String::from(
                            error_message,
                        ))
                    }
                    "InternalErrorException" => {
                        DescribeSubscribersForNotificationError::InternalError(String::from(
                            error_message,
                        ))
                    }
                    "InvalidNextTokenException" => {
                        DescribeSubscribersForNotificationError::InvalidNextToken(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterException" => {
                        DescribeSubscribersForNotificationError::InvalidParameter(String::from(
                            error_message,
                        ))
                    }
                    "NotFoundException" => DescribeSubscribersForNotificationError::NotFound(
                        String::from(error_message),
                    ),
                    "ValidationException" => DescribeSubscribersForNotificationError::Validation(
                        error_message.to_string(),
                    ),
                    _ => DescribeSubscribersForNotificationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeSubscribersForNotificationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeSubscribersForNotificationError {
    fn from(err: serde_json::error::Error) -> DescribeSubscribersForNotificationError {
        DescribeSubscribersForNotificationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeSubscribersForNotificationError {
    fn from(err: CredentialsError) -> DescribeSubscribersForNotificationError {
        DescribeSubscribersForNotificationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeSubscribersForNotificationError {
    fn from(err: HttpDispatchError) -> DescribeSubscribersForNotificationError {
        DescribeSubscribersForNotificationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeSubscribersForNotificationError {
    fn from(err: io::Error) -> DescribeSubscribersForNotificationError {
        DescribeSubscribersForNotificationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeSubscribersForNotificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeSubscribersForNotificationError {
    fn description(&self) -> &str {
        match *self {
            DescribeSubscribersForNotificationError::ExpiredNextToken(ref cause) => cause,
            DescribeSubscribersForNotificationError::InternalError(ref cause) => cause,
            DescribeSubscribersForNotificationError::InvalidNextToken(ref cause) => cause,
            DescribeSubscribersForNotificationError::InvalidParameter(ref cause) => cause,
            DescribeSubscribersForNotificationError::NotFound(ref cause) => cause,
            DescribeSubscribersForNotificationError::Validation(ref cause) => cause,
            DescribeSubscribersForNotificationError::Credentials(ref err) => err.description(),
            DescribeSubscribersForNotificationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeSubscribersForNotificationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateBudget
#[derive(Debug, PartialEq)]
pub enum UpdateBudgetError {
    ///This exception is thrown on an unknown internal failure.
    InternalError(String),
    ///This exception is thrown if any request is given an invalid parameter. E.g., if a required Date field is null.
    InvalidParameter(String),
    ///This exception is thrown if a requested entity is not found. E.g., if a budget id doesn't exist for an account ID.
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateBudgetError {
    pub fn from_body(body: &str) -> UpdateBudgetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        UpdateBudgetError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        UpdateBudgetError::InvalidParameter(String::from(error_message))
                    }
                    "NotFoundException" => UpdateBudgetError::NotFound(String::from(error_message)),
                    "ValidationException" => {
                        UpdateBudgetError::Validation(error_message.to_string())
                    }
                    _ => UpdateBudgetError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateBudgetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateBudgetError {
    fn from(err: serde_json::error::Error) -> UpdateBudgetError {
        UpdateBudgetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateBudgetError {
    fn from(err: CredentialsError) -> UpdateBudgetError {
        UpdateBudgetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateBudgetError {
    fn from(err: HttpDispatchError) -> UpdateBudgetError {
        UpdateBudgetError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateBudgetError {
    fn from(err: io::Error) -> UpdateBudgetError {
        UpdateBudgetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateBudgetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateBudgetError {
    fn description(&self) -> &str {
        match *self {
            UpdateBudgetError::InternalError(ref cause) => cause,
            UpdateBudgetError::InvalidParameter(ref cause) => cause,
            UpdateBudgetError::NotFound(ref cause) => cause,
            UpdateBudgetError::Validation(ref cause) => cause,
            UpdateBudgetError::Credentials(ref err) => err.description(),
            UpdateBudgetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateBudgetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateNotification
#[derive(Debug, PartialEq)]
pub enum UpdateNotificationError {
    ///The exception is thrown when customer tries to create a record (e.g. budget) that already exists.
    DuplicateRecord(String),
    ///This exception is thrown on an unknown internal failure.
    InternalError(String),
    ///This exception is thrown if any request is given an invalid parameter. E.g., if a required Date field is null.
    InvalidParameter(String),
    ///This exception is thrown if a requested entity is not found. E.g., if a budget id doesn't exist for an account ID.
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateNotificationError {
    pub fn from_body(body: &str) -> UpdateNotificationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DuplicateRecordException" => {
                        UpdateNotificationError::DuplicateRecord(String::from(error_message))
                    }
                    "InternalErrorException" => {
                        UpdateNotificationError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        UpdateNotificationError::InvalidParameter(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateNotificationError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateNotificationError::Validation(error_message.to_string())
                    }
                    _ => UpdateNotificationError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateNotificationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateNotificationError {
    fn from(err: serde_json::error::Error) -> UpdateNotificationError {
        UpdateNotificationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateNotificationError {
    fn from(err: CredentialsError) -> UpdateNotificationError {
        UpdateNotificationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateNotificationError {
    fn from(err: HttpDispatchError) -> UpdateNotificationError {
        UpdateNotificationError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateNotificationError {
    fn from(err: io::Error) -> UpdateNotificationError {
        UpdateNotificationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateNotificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateNotificationError {
    fn description(&self) -> &str {
        match *self {
            UpdateNotificationError::DuplicateRecord(ref cause) => cause,
            UpdateNotificationError::InternalError(ref cause) => cause,
            UpdateNotificationError::InvalidParameter(ref cause) => cause,
            UpdateNotificationError::NotFound(ref cause) => cause,
            UpdateNotificationError::Validation(ref cause) => cause,
            UpdateNotificationError::Credentials(ref err) => err.description(),
            UpdateNotificationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateNotificationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateSubscriber
#[derive(Debug, PartialEq)]
pub enum UpdateSubscriberError {
    ///The exception is thrown when customer tries to create a record (e.g. budget) that already exists.
    DuplicateRecord(String),
    ///This exception is thrown on an unknown internal failure.
    InternalError(String),
    ///This exception is thrown if any request is given an invalid parameter. E.g., if a required Date field is null.
    InvalidParameter(String),
    ///This exception is thrown if a requested entity is not found. E.g., if a budget id doesn't exist for an account ID.
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateSubscriberError {
    pub fn from_body(body: &str) -> UpdateSubscriberError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DuplicateRecordException" => {
                        UpdateSubscriberError::DuplicateRecord(String::from(error_message))
                    }
                    "InternalErrorException" => {
                        UpdateSubscriberError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        UpdateSubscriberError::InvalidParameter(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateSubscriberError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateSubscriberError::Validation(error_message.to_string())
                    }
                    _ => UpdateSubscriberError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateSubscriberError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateSubscriberError {
    fn from(err: serde_json::error::Error) -> UpdateSubscriberError {
        UpdateSubscriberError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateSubscriberError {
    fn from(err: CredentialsError) -> UpdateSubscriberError {
        UpdateSubscriberError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateSubscriberError {
    fn from(err: HttpDispatchError) -> UpdateSubscriberError {
        UpdateSubscriberError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateSubscriberError {
    fn from(err: io::Error) -> UpdateSubscriberError {
        UpdateSubscriberError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateSubscriberError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateSubscriberError {
    fn description(&self) -> &str {
        match *self {
            UpdateSubscriberError::DuplicateRecord(ref cause) => cause,
            UpdateSubscriberError::InternalError(ref cause) => cause,
            UpdateSubscriberError::InvalidParameter(ref cause) => cause,
            UpdateSubscriberError::NotFound(ref cause) => cause,
            UpdateSubscriberError::Validation(ref cause) => cause,
            UpdateSubscriberError::Credentials(ref err) => err.description(),
            UpdateSubscriberError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateSubscriberError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWSBudgets API. AWSBudgets clients implement this trait.
pub trait Budgets {
    #[doc = "Create a new budget"]
    fn create_budget(
        &self,
        input: &CreateBudgetRequest,
    ) -> Result<CreateBudgetResponse, CreateBudgetError>;

    #[doc = "Create a new Notification with subscribers for a budget"]
    fn create_notification(
        &self,
        input: &CreateNotificationRequest,
    ) -> Result<CreateNotificationResponse, CreateNotificationError>;

    #[doc = "Create a new Subscriber for a notification"]
    fn create_subscriber(
        &self,
        input: &CreateSubscriberRequest,
    ) -> Result<CreateSubscriberResponse, CreateSubscriberError>;

    #[doc = "Delete a budget and related notifications"]
    fn delete_budget(
        &self,
        input: &DeleteBudgetRequest,
    ) -> Result<DeleteBudgetResponse, DeleteBudgetError>;

    #[doc = "Delete a notification and related subscribers"]
    fn delete_notification(
        &self,
        input: &DeleteNotificationRequest,
    ) -> Result<DeleteNotificationResponse, DeleteNotificationError>;

    #[doc = "Delete a Subscriber for a notification"]
    fn delete_subscriber(
        &self,
        input: &DeleteSubscriberRequest,
    ) -> Result<DeleteSubscriberResponse, DeleteSubscriberError>;

    #[doc = "Get a single budget"]
    fn describe_budget(
        &self,
        input: &DescribeBudgetRequest,
    ) -> Result<DescribeBudgetResponse, DescribeBudgetError>;

    #[doc = "Get all budgets for an account"]
    fn describe_budgets(
        &self,
        input: &DescribeBudgetsRequest,
    ) -> Result<DescribeBudgetsResponse, DescribeBudgetsError>;

    #[doc = "Get notifications of a budget"]
    fn describe_notifications_for_budget(
        &self,
        input: &DescribeNotificationsForBudgetRequest,
    ) -> Result<DescribeNotificationsForBudgetResponse, DescribeNotificationsForBudgetError>;

    #[doc = "Get subscribers of a notification"]
    fn describe_subscribers_for_notification(
        &self,
        input: &DescribeSubscribersForNotificationRequest,
    ) -> Result<DescribeSubscribersForNotificationResponse, DescribeSubscribersForNotificationError>;

    #[doc = "Update the information of a budget already created"]
    fn update_budget(
        &self,
        input: &UpdateBudgetRequest,
    ) -> Result<UpdateBudgetResponse, UpdateBudgetError>;

    #[doc = "Update the information about a notification already created"]
    fn update_notification(
        &self,
        input: &UpdateNotificationRequest,
    ) -> Result<UpdateNotificationResponse, UpdateNotificationError>;

    #[doc = "Update a subscriber"]
    fn update_subscriber(
        &self,
        input: &UpdateSubscriberRequest,
    ) -> Result<UpdateSubscriberResponse, UpdateSubscriberError>;
}
/// A client for the AWSBudgets API.
pub struct BudgetsClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> BudgetsClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        BudgetsClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> Budgets for BudgetsClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    #[doc = "Create a new budget"]
    fn create_budget(
        &self,
        input: &CreateBudgetRequest,
    ) -> Result<CreateBudgetResponse, CreateBudgetError> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.CreateBudget");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateBudgetResponse>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateBudgetError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Create a new Notification with subscribers for a budget"]
    fn create_notification(
        &self,
        input: &CreateNotificationRequest,
    ) -> Result<CreateNotificationResponse, CreateNotificationError> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.CreateNotification");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateNotificationResponse>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateNotificationError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Create a new Subscriber for a notification"]
    fn create_subscriber(
        &self,
        input: &CreateSubscriberRequest,
    ) -> Result<CreateSubscriberResponse, CreateSubscriberError> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.CreateSubscriber");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateSubscriberResponse>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateSubscriberError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Delete a budget and related notifications"]
    fn delete_budget(
        &self,
        input: &DeleteBudgetRequest,
    ) -> Result<DeleteBudgetResponse, DeleteBudgetError> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.DeleteBudget");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteBudgetResponse>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteBudgetError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Delete a notification and related subscribers"]
    fn delete_notification(
        &self,
        input: &DeleteNotificationRequest,
    ) -> Result<DeleteNotificationResponse, DeleteNotificationError> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.DeleteNotification");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteNotificationResponse>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteNotificationError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Delete a Subscriber for a notification"]
    fn delete_subscriber(
        &self,
        input: &DeleteSubscriberRequest,
    ) -> Result<DeleteSubscriberResponse, DeleteSubscriberError> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.DeleteSubscriber");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteSubscriberResponse>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteSubscriberError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Get a single budget"]
    fn describe_budget(
        &self,
        input: &DescribeBudgetRequest,
    ) -> Result<DescribeBudgetResponse, DescribeBudgetError> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.DescribeBudget");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeBudgetResponse>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeBudgetError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Get all budgets for an account"]
    fn describe_budgets(
        &self,
        input: &DescribeBudgetsRequest,
    ) -> Result<DescribeBudgetsResponse, DescribeBudgetsError> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.DescribeBudgets");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeBudgetsResponse>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeBudgetsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Get notifications of a budget"]
    fn describe_notifications_for_budget(
        &self,
        input: &DescribeNotificationsForBudgetRequest,
    ) -> Result<DescribeNotificationsForBudgetResponse, DescribeNotificationsForBudgetError> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSBudgetServiceGateway.DescribeNotificationsForBudget",
        );
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(
                    serde_json::from_str::<DescribeNotificationsForBudgetResponse>(
                        String::from_utf8_lossy(&body).as_ref(),
                    ).unwrap(),
                )
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeNotificationsForBudgetError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Get subscribers of a notification"]
    fn describe_subscribers_for_notification(
        &self,
        input: &DescribeSubscribersForNotificationRequest,
    ) -> Result<DescribeSubscribersForNotificationResponse, DescribeSubscribersForNotificationError>
    {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSBudgetServiceGateway.DescribeSubscribersForNotification",
        );
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(
                    serde_json::from_str::<DescribeSubscribersForNotificationResponse>(
                        String::from_utf8_lossy(&body).as_ref(),
                    ).unwrap(),
                )
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeSubscribersForNotificationError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Update the information of a budget already created"]
    fn update_budget(
        &self,
        input: &UpdateBudgetRequest,
    ) -> Result<UpdateBudgetResponse, UpdateBudgetError> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.UpdateBudget");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateBudgetResponse>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateBudgetError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Update the information about a notification already created"]
    fn update_notification(
        &self,
        input: &UpdateNotificationRequest,
    ) -> Result<UpdateNotificationResponse, UpdateNotificationError> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.UpdateNotification");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateNotificationResponse>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateNotificationError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "Update a subscriber"]
    fn update_subscriber(
        &self,
        input: &UpdateSubscriberRequest,
    ) -> Result<UpdateSubscriberResponse, UpdateSubscriberError> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.UpdateSubscriber");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateSubscriberResponse>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateSubscriberError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
