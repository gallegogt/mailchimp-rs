use crate::api::MailchimpApi;
use log::error;
use serde::de::DeserializeOwned;
use std::cell::RefCell;
use std::collections::HashMap;

pub trait ResourceFilter {
    fn build_payload(&self) -> HashMap<String, String>;
}

pub trait MailchimpCollection<T> {
    /// Total Items
    fn get_total_items(&self) -> u32;

    /// Data
    fn get_values(&self) -> Vec<T>;
}

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
    pub data: RefCell<Vec<B::Item>>,
    pub cur_filters: B::FilterItem,
    pub cur_it: usize,
    pub total_items: u32,
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
        let mut in_data = self.data.borrow_mut();
        if self.cur_it < (self.total_items as usize) && (self.cur_it == in_data.len() - 2) {
            let new_filter = self.builder.update_filter_offset(&self.cur_filters);
            let cl = self.get_collection(&new_filter);
            self.cur_filters = new_filter;

            for r in cl.get_values() {
                in_data.push(r);
            }
        }

        if self.cur_it < in_data.len() {
            let data = &in_data[self.cur_it];
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
    pub fn get_collection(&self, filters: &B::FilterItem) -> B::Collection
    {
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