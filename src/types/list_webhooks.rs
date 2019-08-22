///
/// Webhooks
///
/// Manage webhooks for a specific Mailchimp list. Learn more about webhooks in Mailchimp.
///
use super::empty::EmptyType;
use super::link::LinkType;
use crate::api::MailchimpApi;
use crate::internal::error_type::MailchimpErrorType;
use crate::internal::request::MailchimpResult;
use crate::iter::{BuildIter, MailchimpCollection, SimpleFilter};
use std::collections::HashMap;
use std::rc::Rc;

///
/// The events that can trigger the webhook and whether they are enabled.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebhookEvent {
    /// Whether the webhook is triggered when a list subscriber is added.
    #[serde(default)]
    subscribe: bool,
    /// Whether the webhook is triggered when a list member unsubscribes.
    #[serde(default)]
    unsubscribe: bool,
    /// Whether the webhook is triggered when a subscriber’s profile is updated.
    #[serde(default)]
    profile: bool,
    /// Whether the webhook is triggered when a subscriber’s email address is cleaned from the list.
    #[serde(default)]
    cleaned: bool,
    /// Whether the webhook is triggered when a subscriber’s email address is changed.
    #[serde(default)]
    upemail: bool,
    /// Whether the webhook is triggered when a campaign is sent or cancelled.
    #[serde(default)]
    campaign: bool,
}

impl Default for WebhookEvent {
    fn default() -> Self {
        WebhookEvent {
            subscribe: false,
            unsubscribe: false,
            profile: false,
            cleaned: false,
            upemail: false,
            campaign: false,
        }
    }
}
///
/// The possible sources of any events that can trigger the webhook and whether they are enabled.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebhookSource {
    /// Whether the webhook is triggered by subscriber-initiated actions.
    #[serde(default)]
    user: bool,
    /// Whether the webhook is triggered by admin-initiated actions in the web interface.
    #[serde(default)]
    admin: bool,
    /// Whether the webhook is triggered by actions initiated via the API.
    #[serde(default)]
    api: bool,
}

impl Default for WebhookSource {
    fn default() -> Self {
        WebhookSource {
            user: false,
            admin: false,
            api: false,
        }
    }
}

///
/// Webhooks
///
/// Manage webhooks for a specific Mailchimp list. Learn more about webhooks in Mailchimp.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListWebhooks {
    /// An string that uniquely identifies this webhook.
    #[serde(default)]
    pub id: String,
    /// A valid URL for the Webhook.
    #[serde(default)]
    pub url: String,
    /// The events that can trigger the webhook and whether they are enabled.
    #[serde(default)]
    pub events: WebhookEvent,
    /// The possible sources of any events that can trigger the webhook and whether they are enabled.
    #[serde(default)]
    pub sources: WebhookSource,
    /// The unique id for the list.
    #[serde(default)]
    pub list_id: String,
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
/// Merge Fields Param
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListWebhooksParam {
    /// A valid URL for the Webhook.
    #[serde(default)]
    pub url: String,
    /// The events that can trigger the webhook and whether they are enabled.
    #[serde(default)]
    pub events: WebhookEvent,
    /// The possible sources of any events that can trigger the webhook and whether they are enabled.
    #[serde(default)]
    pub sources: WebhookSource,
}

///
/// Response for endpoint  GET /lists/{list_id}/members/{subscriber_hash}/activity
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionListWebhooks {
    /// An array of objects, each representing a specific list member.
    #[serde(default)]
    pub webhooks: Vec<ListWebhooks>,
    /// The unique id for the list.
    #[serde(default)]
    pub list_id: String,
    /// The total number of items matching the query regardless of pagination.
    #[serde(default)]
    pub total_items: u64,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

impl MailchimpCollection<ListWebhooks> for CollectionListWebhooks {
    /// Total Items
    fn get_total_items(&self) -> u64 {
        self.total_items
    }
    /// Data
    fn get_values(&self) -> Vec<ListWebhooks> {
        self.webhooks.clone()
    }
}

impl Default for CollectionListWebhooks {
    fn default() -> Self {
        CollectionListWebhooks {
            list_id: "".to_string(),
            webhooks: Vec::new(),
            total_items: 0,
            _links: Vec::new(),
        }
    }
}

///
/// ListWebhooksBuilder
///
#[derive(Debug)]
pub struct ListWebhooksBuilder {
    /// Resource Endpoint
    pub endpoint: String,
}

impl BuildIter for ListWebhooksBuilder {
    type Item = ListWebhooks;
    type FilterItem = SimpleFilter;
    type Collection = CollectionListWebhooks;

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

impl ListWebhooks {
    ///
    /// Update a webhook
    ///
    pub fn update(&self, param: ListWebhooksParam) -> MailchimpResult<ListWebhooks> {
        // PATCH /lists/{list_id}/webhooks/{webhook_id}
        let endpoint = self.get_base_endpoint();
        self._api
            .patch::<ListWebhooks, ListWebhooksParam>(&endpoint, param)
    }

    ///
    /// Delete a webhook
    ///
    /// Delete a specific webhook in a list.
    ///
    pub fn delete(&self) -> Option<MailchimpErrorType> {
        // DELETE /lists/{list_id}/webhooks/{webhook_id}
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
