//! Implement Automations Email Queue Types
//!

use super::link::LinkType;
use crate::api::MailchimpApi;
use crate::iter::{BuildIter, MailchimpCollection, ResourceFilter};
use std::collections::HashMap;

///
/// Workflow Email
///
/// Endpoint
///     GET /automations/{workflow_id}/emails/{workflow_email_id}/queue/{subscriber_hash}
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutomationEmailQueueType {
    /// The MD5 hash of the lowercase version of the list member’s email address.
    #[serde(default)]
    pub id: String,
    /// A string that uniquely identifies an Automation workflow.
    #[serde(default)]
    pub workflow_id: String,
    /// A string that uniquely identifies an email in an Automation workflow.
    #[serde(default)]
    pub email_id: String,
    /// A string that uniquely identifies a list.
    #[serde(default)]
    pub list_id: String,
    /// The status of the list used, namely if it’s deleted or disabled.
    #[serde(default)]
    pub list_is_active: bool,
    /// The list member’s email address.
    #[serde(default)]
    pub email_address: String,
    /// The date and time of the next send for the workflow email in ISO 8601 format.
    #[serde(default)]
    pub next_send: String,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

///
/// Collection Automation Email Queue
///
/// Endpoint
///     GET /automations/{workflow_id}/emails/{workflow_email_id}/queue
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionAutomationEmailQueue {
    /// A string that uniquely identifies an Automation workflow.
    #[serde(default)]
    pub workflow_id: String,
    /// A string that uniquely identifies an email in an Automation workflow.
    #[serde(default)]
    pub email_id: String,
    /// A string that uniquely identifies a list.
    #[serde(default)]
    pub queue: Vec<AutomationEmailQueueType>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(default)]
    pub total_items: u64,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

impl MailchimpCollection<AutomationEmailQueueType> for CollectionAutomationEmailQueue {
    /// Total Items
    fn get_total_items(&self) -> u64 {
        self.total_items
    }

    /// Data
    fn get_values(&self) -> Vec<AutomationEmailQueueType> {
        self.queue.clone()
    }
}

impl Default for CollectionAutomationEmailQueue {
    fn default() -> Self {
        CollectionAutomationEmailQueue {
            workflow_id: String::new(),
            email_id: String::new(),
            queue: Vec::new(),
            total_items: 0,
            _links: Vec::new(),
        }
    }
}

///
/// Automation Email Queue Filter
///
#[derive(Debug, Clone)]
pub struct AutomationEmailQueueFilter {
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

impl Default for AutomationEmailQueueFilter {
    fn default() -> Self {
        AutomationEmailQueueFilter {
            fields: None,
            exclude_fields: None,
            count: Some(50),
            offset: Some(0),
        }
    }
}

impl ResourceFilter for AutomationEmailQueueFilter {
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
/// Automation Email Queue Builder
///
#[derive(Debug)]
pub struct AutomationEmailQueueBuilder {}

impl BuildIter for AutomationEmailQueueBuilder {
    type Item = AutomationEmailQueueType;
    type FilterItem = AutomationEmailQueueFilter;
    type Collection = CollectionAutomationEmailQueue;

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
