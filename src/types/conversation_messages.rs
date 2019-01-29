///
/// Conversations
///
use super::link::LinkType;
use crate::api::MailchimpApi;
use crate::iter::{BuildIter, MailchimpCollection, ResourceFilter};
use std::collections::HashMap;

///
/// The most recent message in the conversation.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConversationMessage {
    /// A string that uniquely identifies this message
    #[serde(default)]
    pub id: String,
    /// A string that identifies this message’s conversation
    #[serde(default)]
    pub conversation_id: String,
    /// The unique identifier of the list for this conversation.
    #[serde(default)]
    pub list_id: String,
    /// A label representing the sender of this message.
    #[serde(default)]
    pub from_label: String,
    /// A label representing the email of the sender of this message.
    #[serde(default)]
    pub from_email: String,
    /// The subject of this message.
    #[serde(default)]
    pub subject: String,
    /// The plain-text content of the message.
    #[serde(default)]
    pub message: String,
    /// Whether this message has been marked as read.
    #[serde(default)]
    pub read: bool,
    /// The date and time the message was either sent or received in ISO 8601 format.
    #[serde(default)]
    pub timestamp: String,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

impl Default for ConversationMessage {
    fn default() -> Self {
        ConversationMessage {
            id: "".to_string(),
            conversation_id: "".to_string(),
            list_id: "".to_string(),
            from_label: "".to_string(),
            from_email: "".to_string(),
            subject: "".to_string(),
            message: "".to_string(),
            read: true,
            timestamp: "".to_string(),
            _links: Vec::new(),
        }
    }
}

///
/// ParamMessage
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParamMessage {
    /// A label representing the email of the sender of this message
    #[serde(default)]
    pub from_email: String,
    /// The subject of this message.
    #[serde(default)]
    pub subject: String,
    /// The plain-text content of the message.
    #[serde(default)]
    pub message: String,
    /// Whether this message has been marked as read.
    #[serde(default)]
    pub read: bool,
}

///
/// CollectionConversationMessages
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionConversationMessages {
    /// Recent list activity.
    #[serde(default)]
    pub conversation_messages: Vec<ConversationMessage>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(default)]
    pub total_items: u64,
    /// A string that identifies this conversation.
    #[serde(default)]
    pub conversation_id: String,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

impl MailchimpCollection<ConversationMessage> for CollectionConversationMessages {
    /// Total Items
    fn get_total_items(&self) -> u64 {
        self.total_items
    }

    /// Data
    fn get_values(&self) -> Vec<ConversationMessage> {
        self.conversation_messages.clone()
    }
}

impl Default for CollectionConversationMessages {
    fn default() -> Self {
        CollectionConversationMessages {
            conversation_id: "".to_string(),
            conversation_messages: Vec::new(),
            total_items: 0,
            _links: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MessagesFilter {
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
    /// Restrict the response to messages created before the set time. We recommend ISO 8601
    /// time format: 2015-10-21T15:41:36+00:00.
    pub before_timestamp: Option<String>,
    /// Restrict the response to messages created after the set time. We recommend ISO 8601
    /// time format: 2015-10-21T15:41:36+00:00.
    pub since_timestamp: Option<String>,
}

impl Default for MessagesFilter {
    fn default() -> Self {
        MessagesFilter {
            fields: None,
            exclude_fields: None,
            count: Some(50),
            offset: Some(0),
            before_timestamp: None,
            since_timestamp: None,
        }
    }
}

impl ResourceFilter for MessagesFilter {
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


        if self.before_timestamp.is_some() {
            payload.insert(
                "before_timestamp".to_string(),
                format!("{:}", self.before_timestamp.as_ref().unwrap().clone()),
            );
        }
        if self.since_timestamp.is_some() {
            payload.insert(
                "since_timestamp".to_string(),
                format!("{:}", self.since_timestamp.as_ref().unwrap().clone()),
            );
        }
        payload
    }
}

///
/// Conversation Builder
///
#[derive(Debug)]
pub struct MessagesBuider {}

impl BuildIter for MessagesBuider {
    type Item = ConversationMessage;
    type FilterItem = MessagesFilter;
    type Collection = CollectionConversationMessages;

    ///
    /// Return a new data updated
    ///
    fn update_item(&self, data: &Self::Item, _: &MailchimpApi) -> Self::Item {
        let in_data = data.clone();
        in_data
    }
    ///
    /// Update the offset for pagination
    ///
    fn update_filter_offset(&self, filter: &Self::FilterItem) -> Self::FilterItem {
        let mut f = filter.clone();
        f.offset = Some(f.count.unwrap() + f.offset.unwrap());
        f
    }
}
