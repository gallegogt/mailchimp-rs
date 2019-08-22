//! Implement Feedback Types
//!

use super::empty::EmptyType;
use super::link::LinkType;
use crate::api::MailchimpApi;
use crate::internal::request::MailchimpResult;
use crate::iter::{BuildIter, MailchimpCollection, SimpleFilter};
use std::collections::HashMap;
use std::rc::Rc;

///
/// Campaign Feedback Type
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CampaignFeedbackType {
    /// The individual id for the feedback item.
    #[serde(default)]
    pub feedback_id: u64,
    /// If a reply, the id of the parent feedback item.
    #[serde(default)]
    pub parent_id: u64,
    /// The block id for the editable block that the feedback addresses.
    #[serde(default)]
    pub block_id: u64,
    /// The content of the feedback.
    #[serde(default)]
    pub message: String,
    /// The status of feedback.
    #[serde(default)]
    pub is_complete: bool,
    /// The login name of the user who created the feedback.
    #[serde(default)]
    pub created_by: String,
    /// The date and time the feedback item was created in ISO 8601 format.
    #[serde(default)]
    pub created_at: String,
    /// The date and time the feedback was last updated in ISO 8601 format.
    #[serde(default)]
    pub updated_at: String,
    /// The source of the feedback. api email smsw eb ios android
    #[serde(default)]
    pub source: String,
    /// The unique id for the campaign.
    #[serde(default)]
    pub campaign_id: String,
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
/// Collection Campaign Feedback
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionCampaignFeedback {
    /// A collection of feedback items for a campaign.
    #[serde(default)]
    pub feedback: Vec<CampaignFeedbackType>,
    /// The unique id for the campaign.
    #[serde(default)]
    pub campaign_id: String,
    /// The total number of items matching the query regardless of pagination.
    #[serde(default)]
    pub total_items: u64,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

impl MailchimpCollection<CampaignFeedbackType> for CollectionCampaignFeedback {
    /// Total Items
    fn get_total_items(&self) -> u64 {
        self.total_items
    }

    /// Data
    fn get_values(&self) -> Vec<CampaignFeedbackType> {
        self.feedback.clone()
    }
}

impl Default for CollectionCampaignFeedback {
    fn default() -> Self {
        CollectionCampaignFeedback {
            feedback: Vec::new(),
            campaign_id: String::new(),
            total_items: 0,
            _links: Vec::new(),
        }
    }
}

///
/// Campaign Feedback Builder
///
#[derive(Debug)]
pub struct CampaignFeedbackBuilder {
    /// Endpoint
    pub endpoint: String,
}

impl BuildIter for CampaignFeedbackBuilder {
    type Item = CampaignFeedbackType;
    type FilterItem = SimpleFilter;
    type Collection = CollectionCampaignFeedback;

    ///
    /// Crea un recurso a partir del dato pasado por parámetro
    ///
    fn update_item(&self, data: &Self::Item, api: Rc<MailchimpApi>) -> Self::Item {
        let mut in_data = data.clone();
        in_data.set_api(api);
        in_data.set_endpoint(&self.endpoint);
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

///
/// Update Feedback Param
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateFeedbackParam {
    /// The block id for the editable block that the feedback addresses.
    #[serde(default)]
    pub block_id: Vec<CampaignFeedbackType>,
    /// The content of the feedback.
    #[serde(default)]
    pub message: String,
    /// The status of feedback.
    #[serde(default)]
    pub is_complete: String,
}

impl CampaignFeedbackType {
    ///
    /// Remove a specific feedback message for a campaign.
    ///
    pub fn delete(&self) -> MailchimpResult<EmptyType> {
        let endpoint = &self.get_base_endpoint();
        self._api.delete::<EmptyType>(&endpoint, HashMap::new())
    }
    ///
    /// Update a specific feedback message for a campaign.
    ///
    pub fn update(&self, param: UpdateFeedbackParam) -> MailchimpResult<CampaignFeedbackType> {
        // PATCH /campaigns/{campaign_id}/feedback/{feedback_id}
        let endpoint = self.get_base_endpoint();
        self._api
            .patch::<CampaignFeedbackType, UpdateFeedbackParam>(&endpoint, param)
    }

    ///
    /// Mailchimp API
    ///
    pub fn set_api(&mut self, api: Rc<MailchimpApi>) {
        self._api = api
    }

    ///
    /// Endpoint
    ///
    pub fn set_endpoint<'a>(&mut self, endpoint: &'a str) {
        self._endpoint = endpoint.to_string()
    }

    ///
    /// Base Endpoint
    ///
    fn get_base_endpoint(&self) -> String {
        format!("{:?}/{:?}", self._endpoint, self.feedback_id)
    }
}
