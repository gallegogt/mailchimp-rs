use super::link::LinkType;
use crate::api::MailchimpApi;
use crate::iter::{BuildIter, MailchimpCollection, SimpleFilter};

// ============ Automation Subscribers ==============
// GET /automations/{workflow_id}/removed-subscribers
// Subscriber who was removed from an Automation workflow
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutomationSubscriberType {
    /// The MD5 hash of the lowercase version of the list member’s email address.
    #[serde(default)]
    pub id: String,
    /// A string that uniquely identifies an Automation workflow.
    #[serde(default)]
    pub workflow_id: String,
    /// A string that uniquely identifies a list.
    #[serde(default)]
    pub list_id: String,
    /// The list member’s email address.
    #[serde(default)]
    pub email_address: String,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

// GET /automations/{workflow_id}/removed-subscribers
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionAutomationSubscriber {
    /// A string that uniquely identifies an Automation workflow.
    #[serde(default)]
    pub workflow_id: String,
    /// An array of objects, each representing a subscriber who was removed from an Automation workflow.
    #[serde(default)]
    pub subscribers: Vec<AutomationSubscriberType>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(default)]
    pub total_items: u64,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

impl MailchimpCollection<AutomationSubscriberType> for CollectionAutomationSubscriber {
    /// Total Items
    fn get_total_items(&self) -> u64 {
        self.total_items
    }

    /// Data
    fn get_values(&self) -> Vec<AutomationSubscriberType> {
        self.subscribers.clone()
    }
}

impl Default for CollectionAutomationSubscriber {
    fn default() -> Self {
        CollectionAutomationSubscriber {
            workflow_id: String::new(),
            subscribers: Vec::new(),
            total_items: 0,
            _links: Vec::new(),
        }
    }
}

/// ================================= ITER =====================

#[derive(Debug)]
pub struct AutomationSubscriberBuilder {}

impl BuildIter for AutomationSubscriberBuilder {
    type Item = AutomationSubscriberType;
    type FilterItem = SimpleFilter;
    type Collection = CollectionAutomationSubscriber;

    ///
    /// Crea un recurso a partir del dato pasado por parámetro
    ///
    fn update_item(&self, data: &Self::Item, _: &MailchimpApi) -> Self::Item {
        let in_data = data.clone();
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
