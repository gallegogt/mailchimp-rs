//! Implement Mailchimp Authorized Apps Endpoint
//!
//! Manage registered, connected apps for your Mailchimp account with the Authorized Apps endpoints.
//!
//! ```
//!     use mailchimp::{AuthorizedApps, AuthorizedFilter, MailchimpApi};
//!     use std::collections::HashMap;
//!
//!     fn main() {
//!         let api = MailchimpApi::new("<API_KEY>");
//!
//!         // Create Instance
//!          let authorized_apps = AuthorizedApps::new(api);
//!
//!         // Get information about all authorized apps.
//!         for app in authorized_apps.iter(AuthorizedFilter::default()); {
//!             println!("ID   {:?}", app.id);
//!             println!("Name   {:?}", app.name);
//!             println!("Descriptions   {:?}", app.description);
//!             println!("Users   {:?}", app.users);
//!         }
//!     }
//! ```
//!

use super::api::{MailchimpApi, MailchimpApiUpdate};
use super::internal::request::MailchimpResult;
use super::iter::{BuildIter, MalchimpIter, ResourceFilter};
use crate::types::{AuthorizedAppType, AuthorizedAppsType, CreatedAuthorizedAppType};
use log::error;
use std::collections::HashMap;

/// Authorized Request Filter
#[derive(Debug, Clone)]
pub struct AuthorizedFilter {
    /// A comma-separated list of fields to return. Reference
    /// parameters of sub-objects with dot notation.
    pub fields: Option<String>,
    /// A comma-separated list of fields to exclude. Reference
    /// parameters of sub-objects with dot notation.
    pub exclude_fields: Option<String>,
    /// The number of records to return. Default value is 10.
    pub count: Option<u64>,
    /// The number of records from a collection to skip. Iterating over
    /// large collections with this parameter can be slow. Default value is 0..
    pub offset: Option<u64>,
}

impl Default for AuthorizedFilter {
    fn default() -> Self {
        AuthorizedFilter {
            fields: None,
            exclude_fields: None,
            count: Some(50),
            offset: Some(0),
        }
    }
}

impl ResourceFilter for AuthorizedFilter {
    fn build_payload(&self) -> HashMap<String, String> {
        let mut payload = HashMap::new();

        if self.fields.is_some() {
            payload.insert("fields".to_string(), self.fields.as_ref().unwrap().clone());
        }
        if self.exclude_fields.is_some() {
            payload.insert(
                "exclude_fields".to_string(),
                self.exclude_fields.as_ref().unwrap().clone(),
            );
        }
        if self.count.is_some() {
            payload.insert(
                "count".to_string(),
                format!("{:}", self.count.as_ref().unwrap().clone()),
            );
        }
        if self.offset.is_some() {
            payload.insert(
                "offset".to_string(),
                format!("{:}", self.offset.as_ref().unwrap().clone()),
            );
        }
        payload
    }
}

///
/// Implement Mailchimp Authorized Apps Endpoint
///
/// Manage registered, connected apps for your Mailchimp account with the Authorized Apps endpoints.
///
#[derive(Debug, Clone)]
pub struct AuthorizedApps {
    api: MailchimpApi,
}
#[derive(Debug)]
pub struct AuthorizedAppsBuilder {}

impl BuildIter for AuthorizedAppsBuilder {
    type Item = AuthorizedAppType;
    type FilterItem = AuthorizedFilter;
    type Collection = AuthorizedAppsType;

    ///
    /// Create new resource, with the api instance updated
    ///
    fn update_item(&self, data: &Self::Item, api: &MailchimpApi) -> Self::Item {
        let mut in_data = data.clone();
        in_data.set_api(api);
        in_data
    }
    ///
    /// Update Offset
    ///
    fn update_filter_offset(&self, filter: &Self::FilterItem) -> Self::FilterItem {
        let mut f = filter.clone();
        f.offset = Some(f.count.unwrap() + f.offset.unwrap());
        f
    }
}

impl AuthorizedApps {
    ///
    /// Arguments:
    ///     api: MailchimpApi
    ///
    pub fn new(api: MailchimpApi) -> Self {
        AuthorizedApps { api: api }
    }

    ///
    /// Get a list of an account’s registered, connected applications.
    ///
    ///
    /// Arguments:
    ///     filters: Filters
    ///         fields: A comma-separated list of fields to return. Reference parameters of
    ///                 sub-objects with dot notation.
    ///         exclude_fields: A comma-separated list of fields to exclude.
    ///                Reference parameters of sub-objects with dot notation.
    ///         count: The number of records to return.
    ///         offset: The number of records from a collection to skip. Iterating over large
    ///             collections with this parameter can be slow. Default value is 0.
    ///
    pub fn get_authorized_apps_from_remote(
        &self,
        filters: Option<&AuthorizedFilter>,
    ) -> Option<AuthorizedAppsType> {
        let mut payload = HashMap::new();
        if filters.is_some() {
            payload = filters.as_ref().unwrap().build_payload();
        }
        let response = self
            .api
            .get::<AuthorizedAppsType>("authorized-apps", payload);
        match response {
            Ok(value) => Some(value),
            Err(e) => {
                error!( target: "mailchimp",  "Load Authorized Apps from remote: Response Error details: {:?}", e);
                None
            }
        }
    }

    ///
    /// Retrieve OAuth2-based credentials to associate API calls with your application.
    ///
    /// Arguments:
    ///     client_id: Id o nombre de usuario único para la autorización
    ///     client_secret: Contraseña del cliente para la autorización
    ///
    pub fn link_authorized_apps<'a>(
        &self,
        client_id: &'a str,
        client_secret: &'a str,
    ) -> MailchimpResult<CreatedAuthorizedAppType> {
        let mut payload = HashMap::new();
        payload.insert("client_id".to_string(), client_id.to_string());
        payload.insert("client_secret".to_string(), client_secret.to_string());

        let resp = self
            .api
            .post::<CreatedAuthorizedAppType, HashMap<String, String>>("authorized-apps", payload);

        match resp {
            Ok(value) => Ok(value.clone()),
            Err(e) => Err(e),
        }
    }

    ///
    ///
    /// Get information about a specific authorized app
    ///
    /// Arguments:
    ///     app_id: identificador de la aplicación autorizada
    ///     filters: Filtros que requieras aplicar a la hora de obtener las aplicaciones
    ///         fields: A comma-separated list of fields to return. Reference
    ///             parameters of sub-objects with dot notation.
    ///         exclude_fields: A comma-separated list of fields to exclude.
    ///             Reference parameters of sub-objects with dot notation.
    ///
    pub fn get_authorized_app_info<'a>(
        &self,
        app_id: &'a str,
        filters: HashMap<String, String>,
    ) -> MailchimpResult<AuthorizedAppType> {
        let endpoint = String::from("authorized-apps/") + app_id;
        let resp = self
            .api
            .get::<AuthorizedAppType>(endpoint.as_str(), filters);
        match resp {
            Ok(value) => Ok(value.clone()),
            Err(e) => Err(e),
        }
    }

    ///
    /// Returns a iterator to access all applications
    ///
    pub fn iter(&self, filters: AuthorizedFilter) -> MalchimpIter<AuthorizedAppsBuilder> {
        if let Some(remote) = self.get_authorized_apps_from_remote(Some(&filters)) {
            return MalchimpIter {
                builder: AuthorizedAppsBuilder {},
                data: remote.apps,
                cur_filters: filters.clone(),
                cur_it: 0,
                total_items: remote.total_items,
                api: self.api.clone(),
                endpoint: "authorized-apps".to_string(),
            };
        }

        MalchimpIter {
            builder: AuthorizedAppsBuilder {},
            data: Vec::new(),
            cur_filters: filters.clone(),
            cur_it: 0,
            total_items: 0,
            api: self.api.clone(),
            endpoint: "authorized-apps".to_string(),
        }
    }
}
