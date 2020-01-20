//! Implement Authorized App Model Types

use super::LinkType;
use crate::api::{MailchimpApi, MailchimpApiUpdate};
use crate::iter::MailchimpCollection;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

///
/// Created Authorized App Type
///
/// Endpoint
///     POST /authorized-apps
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatedAuthorizedAppType {
    /// The access token issued by the auth server.
    #[serde(default)]
    pub access_token: String,
    /// Desc: The viewer token issued by the auth server.
    #[serde(default)]
    pub viewer_token: String,
    /// Desc: A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

///
/// Authorized App Type
///
/// Endpoint
///     GET /authorized-apps/{app_id}
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthorizedAppType {
    /// The ID for the application.
    #[serde(default)]
    pub id: u64,
    /// The name of the application.
    #[serde(default)]
    pub name: String,
    /// A short description of the application.
    #[serde(default)]
    pub description: String,
    /// An array of usernames for users who have linked the app.
    #[serde(default)]
    pub users: Vec<String>,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(skip)]
    pub _api: Rc<MailchimpApi>,
}

impl MailchimpApiUpdate for AuthorizedAppType {
    ///
    /// Update API
    ///
    fn set_api(&mut self, n_api: Rc<MailchimpApi>) {
        self._api = n_api
    }
}

/// List of AuthorizedAppType
pub type AuthorizedAppList = Vec<AuthorizedAppType>;

///
/// Authorized Apps Type
///
/// Endpoint
///     GET /authorized-apps
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthorizedAppsType {
    /// An array of objects, each representing an authorized application.
    #[serde(default)]
    pub apps: AuthorizedAppList,
    /// Desc: The total number of items matching the query regardless of pagination.
    #[serde(default)]
    pub total_items: u64,
    /// Desc: A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

impl MailchimpCollection<AuthorizedAppType> for AuthorizedAppsType {
    /// Total Items
    fn get_total_items(&self) -> u64 {
        self.total_items
    }
    /// Data
    fn get_values(&self) -> Vec<AuthorizedAppType> {
        self.apps.clone()
    }
}

impl Default for AuthorizedAppsType {
    fn default() -> Self {
        AuthorizedAppsType {
            apps: Vec::new(),
            total_items: 0,
            _links: Vec::new(),
        }
    }
}
