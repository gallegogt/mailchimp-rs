use super::api::MailchimpApi;
use super::internal::request::MailchimpResult;
use super::types::{
    CollectionLandingPage, LandingPageBuilder, LandingPage
};
use crate::iter::{MalchimpIter, SimpleFilter, ResourceFilter};
use log::error;
use std::collections::HashMap;
use std::rc::Rc;

///
/// Landing Pages
///
/// Manage your Landing Pages, including publishing and unpublishing.
///
#[derive(Debug, Clone)]
pub struct LandingPages {
    api: Rc<MailchimpApi>,
}

impl LandingPages {
    ///
    /// Argumentos:
    ///     api: MailchimpApi
    ///
    pub fn new(api: MailchimpApi) -> Self {
        LandingPages { api: Rc::new(api) }
    }

    ///
    /// Get a list of landing pages
    ///
    pub fn get_pages(
        &self,
        filter: Option<SimpleFilter>,
    ) -> MalchimpIter<LandingPageBuilder> {
        // GET /landing-pages
        let endpoint = "landing-pages";
        let mut filter_params = SimpleFilter::default();

        if let Some(f) = filter {
            filter_params = f;
        }

        match self
            .api
            .get::<CollectionLandingPage>(&endpoint, filter_params.build_payload())
        {
            Ok(collection) => MalchimpIter {
                builder: LandingPageBuilder {},
                data: collection.landing_pages,
                cur_filters: filter_params.clone(),
                cur_it: 0,
                total_items: collection.total_items,
                api: self.api.clone(),
                endpoint: endpoint.to_string(),
            },
            Err(e) => {
                error!( target: "mailchimp",  "Get Activities: Response Error details: {:?}", e);
                MalchimpIter {
                    builder: LandingPageBuilder {},
                    data: Vec::new(),
                    cur_filters: filter_params.clone(),
                    cur_it: 0,
                    total_items: 0,
                    api: self.api.clone(),
                    endpoint: endpoint.to_string(),
                }
            }
        }
    }

    ///
    /// Get information about a specific page
    ///
    pub fn get_landing_page<'a>(&self, page_id: &'a str) -> MailchimpResult<LandingPage> {
        let endpoint = format!("landing-pages/{}", page_id);
        let mut payload = HashMap::new();
        payload.insert("page_id".to_string(), page_id.to_string());
        self.api.get::<LandingPage>(&endpoint, payload)
    }
}
