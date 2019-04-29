extern crate log;
extern crate reqwest;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

// pub mod models;
mod api;
mod api_root;
mod authorized_apps;
mod automations;
mod campaigns;
mod conversations;
mod internal;
pub mod iter;
mod list;
pub mod types;
mod reports;

pub use crate::api::MailchimpApi;
pub use crate::api_root::ApiRoot;
pub use crate::authorized_apps::{AuthorizedApps, AuthorizedFilter};
pub use crate::automations::{Automations, AutomationsFilter};
pub use crate::campaigns::{CampaignFilter, Campaigns};
pub use crate::conversations::Conversations;
pub use crate::list::{ListFilter, Lists};
pub use crate::reports::Reports;
