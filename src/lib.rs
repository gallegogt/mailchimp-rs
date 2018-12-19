extern crate reqwest;
extern crate log;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

// pub mod models;
mod internal;
mod api;
mod client;

pub use crate::api::{MailchimpApi, RequestMethod};
pub use crate::internal::types::*;
pub use crate::client::MailchimpClient;