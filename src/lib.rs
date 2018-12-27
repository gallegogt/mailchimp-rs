extern crate reqwest;
extern crate log;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

// pub mod models;
mod internal;
mod api;
mod campaigns;
mod automations;
mod api_root;
mod list;
mod authorized_apps;
pub mod resources;
pub mod iter;

pub use crate::api::MailchimpApi;
pub use crate::internal::types::*;
pub use crate::campaigns::{CampaignFilter, Campaigns};
pub use crate::automations::{Automations, AutomationsFilter};
pub use crate::api_root::ApiRoot;
pub use crate::list::{ListFilter, Lists};
pub use crate::authorized_apps::{AuthorizedApps, AuthorizedFilter};