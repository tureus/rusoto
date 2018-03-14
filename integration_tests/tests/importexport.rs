#![cfg(feature = "importexport")]
extern crate env_logger;
extern crate rusoto_core;
extern crate rusoto_importexport;

use rusoto_importexport::{ImportExport, ImportExportClient, ListJobsInput};
use rusoto_core::Region;

#[test]
fn should_list_jobs() {
    let _ = env_logger::try_init();
    let client = ImportExportClient::simple(Region::UsEast1);
    let request = ListJobsInput::default();

    let result = client.list_jobs(&request).sync().unwrap();
    println!("{:#?}", result);
}
