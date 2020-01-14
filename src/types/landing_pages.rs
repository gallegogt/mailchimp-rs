use super::empty::EmptyType;
use super::link::LinkType;
use crate::api::MailchimpApi;
use crate::internal::error_type::MailchimpErrorType;
use crate::internal::request::MailchimpResult;
use crate::iter::{BuildIter, MailchimpCollection, SimpleFilter};
///
/// Landing Pages
///
/// Manage your Landing Pages, including publishing and unpublishing.
///
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

///
/// Manage your Landing Pages, including publishing and unpublishing.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LandingPage {
    /// Landing Page ID, A string that uniquely identifies this landing page.
    #[serde(default)]
    pub id: String,
    /// Landing Page Name, The name of this landing page.
    #[serde(default)]
    pub name: String,
    /// The title of this landing page seen in the browser’s title bar.
    #[serde(default)]
    pub title: String,
    /// The description of this landing page.
    #[serde(default)]
    pub description: String,
    /// The template_id of this landing page.
    #[serde(default)]
    pub template_id: u64,
    /// The status of this landing page.
    #[serde(default)]
    pub status: String,
    /// The list’s ID associated with this landing page.
    #[serde(default)]
    pub list_id: String,
    /// The ID of the store associated with this landing page.
    #[serde(default)]
    pub store_id: String,
    /// The url of the published landing page.
    #[serde(default)]
    pub url: String,
    /// The time this landing page was created.
    #[serde(default)]
    pub created_at: String,
    /// The time this landing page was published.
    #[serde(default)]
    pub published_at: String,
    /// The time this landing page was unpublished.
    #[serde(default)]
    pub unpublished_at: String,
    /// The time this landing page was updated at.
    #[serde(default)]
    pub updated_at: String,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,

    /// Mailchimp API
    #[serde(skip)]
    _api: Rc<MailchimpApi>,

    /// Endpoint
    #[serde(skip)]
    _endpoint: String,
}

///
/// Response for endpoint  GET /landing-pages
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionLandingPage {
    /// An array of objects, each representing a specific landing pages.
    #[serde(default)]
    pub landing_pages: Vec<LandingPage>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(default)]
    pub total_items: u64,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

impl MailchimpCollection<LandingPage> for CollectionLandingPage {
    /// Total Items
    fn get_total_items(&self) -> u64 {
        self.total_items
    }
    /// Data
    fn get_values(&self) -> Vec<LandingPage> {
        self.landing_pages.clone()
    }
}

impl Default for CollectionLandingPage {
    fn default() -> Self {
        CollectionLandingPage {
            landing_pages: Vec::new(),
            total_items: 0,
            _links: Vec::new(),
        }
    }
}

///
/// LandingPageBuilder
///
#[derive(Debug)]
pub struct LandingPageBuilder {
    // /// Resource Endpoint
// pub endpoint: String,
}

impl BuildIter for LandingPageBuilder {
    type Item = LandingPage;
    type FilterItem = SimpleFilter;
    type Collection = CollectionLandingPage;

    ///
    /// Crea un recurso a partir del dato pasado por parámetro
    ///
    fn update_item(&self, data: &Self::Item, api: Rc<MailchimpApi>) -> Self::Item {
        let mut in_data = data.clone();
        in_data.set_api(api);
        // in_data.set_endpoint(&self.endpoint);
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

impl LandingPage {
    ///
    /// Delete a webhook
    ///
    /// Delete a specific webhook in a list.
    ///
    pub fn delete(&self) -> Option<MailchimpErrorType> {
        // DELETE /landing-pages/{page_id}
        let endpoint = self.get_base_endpoint();
        match self._api.delete::<EmptyType>(&endpoint, HashMap::new()) {
            Ok(_) => None,
            Err(e) => Some(e),
        }
    }

    ///
    /// Set API
    ///
    pub fn set_api(&mut self, api: Rc<MailchimpApi>) {
        self._api = api;
    }
    ///
    /// Set Endpoint
    ///
    pub fn set_endpoint<'a>(&mut self, endpoint: &'a str) {
        self._endpoint = endpoint.to_string();
    }
    ///
    /// Get Base Endpoint
    ///
    fn get_base_endpoint(&self) -> String {
        let mut endpoint = self._endpoint.clone();
        endpoint.push_str(format!("/{}", self.id).as_str());
        endpoint
    }
}
