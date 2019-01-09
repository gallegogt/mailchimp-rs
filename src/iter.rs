use crate::api::MailchimpApi;
use log::error;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

// ===================== FILTER ==========
pub trait ResourceFilter {
    fn build_payload(&self) -> HashMap<String, String>;
}

#[derive(Debug, Clone)]
pub struct SimpleFilter {
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

impl Default for SimpleFilter {
    fn default() -> Self {
        SimpleFilter {
            fields: None,
            exclude_fields: None,
            count: Some(50),
            offset: Some(0),
        }
    }
}

impl ResourceFilter for SimpleFilter {
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

// ===================== Collection ==========
pub trait MailchimpCollection<T> {
    /// Total Items
    fn get_total_items(&self) -> u64;

    /// Data
    fn get_values(&self) -> Vec<T>;
}

// ===================== BuildIter ==========
pub trait BuildIter {
    type FilterItem;
    type Item;
    type Collection;

    ///
    /// Crea un recurso a partir del dato pasado por parámetro
    ///
    fn update_item(&self, data: &Self::Item, api: &MailchimpApi) -> Self::Item;
    ///
    /// Actualiza el offset
    ///
    fn update_filter_offset(&self, filter: &Self::FilterItem) -> Self::FilterItem;
}

// ========================== MalchimpIter ==========================
///
/// MalchimpIter
///
#[derive(Debug, Clone)]
pub struct MalchimpIter<B>
where
    B: BuildIter,
    B::FilterItem: ResourceFilter,
{
    pub builder: B,
    pub data: Vec<B::Item>,
    pub cur_filters: B::FilterItem,
    pub cur_it: u64,
    pub total_items: u64,
    // Mailchimp API
    pub api: MailchimpApi,
    // Endpoint
    pub endpoint: String,
}

impl<B> Iterator for MalchimpIter<B>
where
    B: BuildIter,
    B::FilterItem: ResourceFilter,
    B::Collection: MailchimpCollection<B::Item> + DeserializeOwned + Default,
{
    type Item = B::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut data_len = self.data.len();

        if let Some(_) = data_len.checked_sub(2) {
            data_len = data_len - 2;
        } else {
            data_len = 0;
        }

        if self.cur_it < self.total_items && ((self.cur_it as usize) == data_len) {
            let new_filter = self.builder.update_filter_offset(&self.cur_filters);
            let cl = self.get_collection(&new_filter);
            self.cur_filters = new_filter;

            for r in cl.get_values() {
                self.data.push(r);
            }
        }

        if (self.cur_it as usize) < self.data.len() {
            let data = &self.data[self.cur_it as usize];
            self.cur_it += 1;
            return Some(self.builder.update_item(data, &self.api));
        }

        None
    }
}

impl<B> MalchimpIter<B>
where
    B: BuildIter,
    B::FilterItem: ResourceFilter,
    B::Collection: MailchimpCollection<B::Item> + DeserializeOwned + Default,
{
    ///
    /// get_collection
    ///
    pub fn get_collection(&self, filters: &B::FilterItem) -> B::Collection {
        let payload = filters.build_payload();
        let response = self.api.get::<B::Collection>(&self.endpoint, payload);
        match response {
            Ok(value) => value,
            Err(e) => {
                error!( target: "mailchimp",  "MailchimpEndpointIter from remote: Response Error details: {:?}", e);
                B::Collection::default()
            }
        }
    }
}
