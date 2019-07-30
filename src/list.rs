//! Implement Mailchimp Lists Endpoint
//!
//! A Mailchimp list is a powerful and flexible tool that helps you manage your
//! contacts. Learn how to get started with lists in Mailchimp.
//!
//! ```
//!     use mailchimp::MailchimpApi;
//!     use mailchimp::{ListFilter, Lists, MailchimpApi};
//!     use std::collections::HashMap;
//!
//!     fn main() {
//!         let api = MailchimpApi::new("<API_KEY>");
//!
//!         // Create Instance of Lists
//!         let lists = Lists::new(api);
//!
//!         // Get information about all lists in the account.
//!         let lists_c = lists.iter(ListFilter::default());
//!         for w in lists_c {
//!             println!("\tID       {:?}",  w.id.unwrap());
//!             println!("\tName    {:?}", w.name);
//!             println!("\tStats   {:?}", w.stats);
//!         }
//!     }
//! ```
//!

use super::api::{MailchimpApi, MailchimpApiUpdate};
use super::internal::request::MailchimpResult;
use super::iter::{BuildIter, MalchimpIter, ResourceFilter};
use super::types::{ListParam, ListType, ListsType};
use log::error;
use std::collections::HashMap;

/// List Filter
#[derive(Debug, Clone)]
pub struct ListFilter {
    /// A comma-separated list of fields to return.
    /// Reference parameters of sub-objects with dot notation.
    pub fields: Option<String>,
    /// A comma-separated list of fields to exclude. Reference
    /// parameters of sub-objects with dot notation.
    pub exclude_fields: Option<String>,
    /// The number of records to return. Default value is 10.
    pub count: Option<u64>,
    /// The number of records from a collection to skip. Iterating over
    /// large collections with this parameter can be slow. Default value is 0..
    pub offset: Option<u64>,
    /// The campaign type.
    /// regular plaintext absplit rss variate
    pub campaign_type: Option<String>,
    /// The status of the campaign.
    pub status: Option<String>,
    /// Restrict the response to campaigns sent before the set time. We recommend
    /// ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    pub before_send_time: Option<String>,
    /// Restrict the response to campaigns sent after the set time. We recommend
    /// ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    pub since_send_time: Option<String>,
    /// Restrict the response to campaigns created before the set time. We recommend
    /// ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    pub before_create_time: Option<String>,
    /// Restrict the response to campaigns created after the set time. We recommend
    /// ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    pub since_create_time: Option<String>,
    /// Restrict results to lists that include a specific subscriber’s email address.
    pub email: Option<String>,
    /// Returns files sorted by the specified field.
    pub sort_field: Option<String>,
    /// Determines the order direction for sorted results.
    /// ASC DESC
    pub sort_dir: Option<String>,
}

impl Default for ListFilter {
    fn default() -> Self {
        ListFilter {
            fields: None,
            exclude_fields: None,
            count: Some(50),
            offset: Some(0),
            campaign_type: None,
            status: None,
            before_send_time: None,
            since_send_time: None,
            before_create_time: None,
            since_create_time: None,
            email: None,
            sort_field: None,
            sort_dir: None,
        }
    }
}

impl ResourceFilter for ListFilter {
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
        if self.campaign_type.is_some() {
            payload.insert(
                "campaign_type".to_string(),
                self.campaign_type.as_ref().unwrap().clone(),
            );
        }
        if self.status.is_some() {
            payload.insert("status".to_string(), self.status.as_ref().unwrap().clone());
        }
        if self.before_send_time.is_some() {
            payload.insert(
                "before_send_time".to_string(),
                self.before_send_time.as_ref().unwrap().clone(),
            );
        }
        if self.since_send_time.is_some() {
            payload.insert(
                "since_send_time".to_string(),
                self.since_send_time.as_ref().unwrap().clone(),
            );
        }
        if self.before_create_time.is_some() {
            payload.insert(
                "before_create_time".to_string(),
                self.before_create_time.as_ref().unwrap().clone(),
            );
        }

        if self.since_create_time.is_some() {
            payload.insert(
                "since_create_time".to_string(),
                self.since_create_time.as_ref().unwrap().clone(),
            );
        }

        if self.email.is_some() {
            payload.insert("email".to_string(), self.email.as_ref().unwrap().clone());
        }
        if self.sort_field.is_some() {
            payload.insert(
                "sort_field".to_string(),
                self.sort_field.as_ref().unwrap().clone(),
            );
        }
        if self.sort_field.is_some() {
            payload.insert(
                "sort_field".to_string(),
                self.sort_field.as_ref().unwrap().clone(),
            );
        }
        if self.sort_dir.is_some() {
            payload.insert(
                "sort_dir".to_string(),
                self.sort_dir.as_ref().unwrap().clone(),
            );
        }

        payload
    }
}

///
/// Implement Mailchimp Lists Endpoint
///
/// A Mailchimp list is a powerful and flexible tool that helps you manage your
/// contacts. Learn how to get started with lists in Mailchimp.
///
#[derive(Debug, Clone)]
pub struct Lists {
    api: MailchimpApi,
}

#[derive(Debug)]
pub struct ListsBuilder {}

impl BuildIter for ListsBuilder {
    type Item = ListType;
    type FilterItem = ListFilter;
    type Collection = ListsType;
    ///
    /// Crea un recurso a partir del dato pasado por parámetro
    ///
    fn update_item(&self, data: &Self::Item, api: &MailchimpApi) -> Self::Item {
        let mut in_data = data.clone();
        in_data.set_api(api);
        in_data
    }
    ///
    /// Actualiza el offset
    ///
    fn update_filter_offset(&self, filter: &Self::FilterItem) -> Self::FilterItem {
        let mut f = filter.clone();
        f.offset = Some(f.count.unwrap() + f.offset.unwrap());
        f
    }
}

impl Lists {
    ///
    /// Argumentos:
    ///     api: MailchimpApi
    ///
    pub fn new(api: MailchimpApi) -> Self {
        Lists { api: api }
    }

    ///
    /// Create a new list in your Mailchimp account.
    ///
    pub fn create_list(&self, param: ListParam) -> MailchimpResult<ListType> {
        self.api.post::<ListType, ListParam>("lists", param)
    }

    ///
    /// Get information about a specific list in your Mailchimp account.
    /// Results include list members who have signed up but haven’t confirmed
    /// their subscription yet and unsubscribed or cleaned.
    ///
    /// Argumentos:
    ///     list_id: The unique id for the list.
    ///     filters: Filtros que se requieran aplicar a la hora de obtener las automatizaciones
    ///         Estos filtros se deben pasar en forma de llave, valor donde las llaves puede ser
    ///         cualquiera de los siguientes:
    ///         fields: listado de campos deseados, separados por coma
    ///         exclude_fields: listado de campos excluidos, separados por coma
    ///
    pub fn get_list_info<'a>(
        &self,
        list_id: &'a str,
        filters: HashMap<String, String>,
    ) -> MailchimpResult<ListType> {
        let endpoint = String::from("lists/") + list_id;
        let response = self.api.get::<ListType>(endpoint.as_str(), filters);

        match response {
            Ok(data) => {
                let mut d = data;
                d.set_api(&self.api);
                Ok(d)
            }
            Err(e) => Err(e),
        }
    }

    ///
    /// Devuelve información de las listas creadas
    ///
    /// Argumentos:
    ///     filters: Filtros que se requieran aplicar a la hora de obtener las automatizaciones
    ///         Estos filtros se deben pasar en forma de llave, valor donde las llaves puede ser
    ///         cualquiera de los siguientes:
    ///         fields: listado de campos deseados, separados por coma
    ///         exclude_fields: listado de campos excluidos, separados por coma
    ///         count: Número de registros a devolver
    ///         offset: El número de registros de una colección a saltar
    ///         before_date_created: Restrict response to lists created before the set date.
    ///         since_date_created: Restrict results to lists created after the set date.
    ///         before_campaign_last_sent: Restrict results to lists created before the last campaign send date.
    ///         since_campaign_last_sent: Restrict results to lists created after the last campaign send date.
    ///         email: Restrict results to lists that include a specific subscriber’s email address.
    ///         sort_field: Returns files sorted by the specified field.
    ///         sort_dir: Determines the order direction for sorted results.
    ///
    pub fn get_campaigns_from_remote(&self, filters: Option<&ListFilter>) -> Option<ListsType> {
        let mut payload = HashMap::new();
        if filters.is_some() {
            payload = filters.as_ref().unwrap().build_payload();
        }
        let response = self.api.get::<ListsType>("lists", payload);
        match response {
            Ok(value) => Some(value),
            Err(e) => {
                error!( target: "mailchimp",  "Load Lists from remote: Response Error details: {:?}", e);
                None
            }
        }
    }

    ///
    /// Función para recorrer todas las campañas exitentes. A diferencia de la
    /// anterior esta función te devuelve un iterador
    ///
    pub fn iter(&self, filters: ListFilter) -> MalchimpIter<ListsBuilder> {
        if let Some(remote) = self.get_campaigns_from_remote(Some(&filters)) {
            return MalchimpIter {
                builder: ListsBuilder {},
                data: remote.lists,
                cur_filters: filters.clone(),
                cur_it: 0,
                total_items: remote.total_items,
                api: self.api.clone(),
                endpoint: "lists".to_string(),
            };
        }
        MalchimpIter {
            builder: ListsBuilder {},
            data: Vec::new(),
            cur_filters: filters.clone(),
            cur_it: 0,
            total_items: 0,
            api: self.api.clone(),
            endpoint: "lists".to_string(),
        }
    }
}
