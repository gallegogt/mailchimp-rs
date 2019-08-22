//! Conversations
//!

use super::conversation_messages::{
    CollectionConversationMessages, ConversationMessage, MessagesBuider, MessagesFilter,
};
use std::rc::Rc;

use super::link::LinkType;
use crate::api::MailchimpApi;
use crate::internal::request::MailchimpResult;
use crate::iter::{BuildIter, MailchimpCollection, MalchimpIter, ResourceFilter};
use log::error;
use std::collections::HashMap;

///
/// Conversation tracking is a paid feature that lets you view subscribers’
/// replies to your campaigns in your Mailchimp account.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Conversation {
    /// A string that uniquely identifies this conversation.
    #[serde(default)]
    pub id: String,
    /// The total number of messages in this conversation.
    #[serde(default)]
    pub message_count: u64,
    /// The unique identifier of the campaign for this conversation.
    #[serde(default)]
    pub campaign_id: String,
    /// The unique identifier of the list for this conversation.
    #[serde(default)]
    pub list_id: String,
    /// The number of unread messages in this conversation.
    #[serde(default)]
    pub unread_messages: u64,
    /// A label representing the sender of this message.
    #[serde(default)]
    pub from_label: String,
    /// A label representing the email of the sender of this message.
    #[serde(default)]
    pub from_email: String,
    /// The subject of the message.
    #[serde(default)]
    pub subject: String,
    /// The most recent message in the conversation.
    #[serde(default)]
    pub last_message: ConversationMessage,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,

    /// Mailchimp API
    #[serde(skip)]
    _api: Rc<MailchimpApi>,
}

///
/// Param Message
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

impl Conversation {
    ///
    /// Post a new message to a conversation.
    ///
    pub fn create_message(&self, message: ParamMessage) -> MailchimpResult<ConversationMessage> {
        // POST /conversations/{conversation_id}/messages
        let mut endpoint = self.get_base_endpoint();
        endpoint.push_str("/messages");
        self._api
            .post::<ConversationMessage, ParamMessage>(&endpoint, message)
    }

    ///
    /// Get conversation messages
    ///
    pub fn get_conversation_messages(
        &self,
        filter: Option<MessagesFilter>,
    ) -> MalchimpIter<MessagesBuider> {
        // GET  /conversations/{conversation_id}/messages
        let mut endpoint = self.get_base_endpoint();
        endpoint.push_str("/messages");

        let mut filter_params = MessagesFilter::default();

        if let Some(f) = filter {
            filter_params = f;
        }

        match self
            ._api
            .get::<CollectionConversationMessages>(&endpoint, filter_params.build_payload())
        {
            Ok(collection) => MalchimpIter {
                builder: MessagesBuider {},
                data: collection.conversation_messages,
                cur_filters: filter_params.clone(),
                cur_it: 0,
                total_items: collection.total_items,
                api: self._api.clone(),
                endpoint: endpoint.to_string(),
            },
            Err(e) => {
                error!( target: "mailchimp",  "Get Activities: Response Error details: {:?}", e);
                MalchimpIter {
                    builder: MessagesBuider {},
                    data: Vec::new(),
                    cur_filters: filter_params.clone(),
                    cur_it: 0,
                    total_items: 0,
                    api: self._api.clone(),
                    endpoint: endpoint.to_string(),
                }
            }
        }
    }

    ///
    /// Get a specific conversation message
    ///
    pub fn get_conversation_message<'a>(
        &self,
        message_id: &'a str,
    ) -> MailchimpResult<ConversationMessage> {
        let mut endpoint = self.get_base_endpoint();
        endpoint.push_str("/messages/");
        endpoint.push_str(message_id);

        let mut payload = HashMap::new();
        payload.insert("message_id".to_string(), message_id.to_string());
        self._api.get::<ConversationMessage>(&endpoint, payload)
    }
    ///
    /// Update API
    ///
    pub fn set_api(&mut self, n_api: Rc<MailchimpApi>) {
        self._api = n_api;
    }

    fn get_base_endpoint(&self) -> String {
        let mut endpoint = "conversations/".to_string();
        endpoint.push_str(&self.id);
        endpoint
    }
}

///
/// Collection Conversations
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionConversations {
    /// Recent list activity.
    #[serde(default)]
    pub conversations: Vec<Conversation>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(default)]
    pub total_items: u64,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

impl MailchimpCollection<Conversation> for CollectionConversations {
    /// Total Items
    fn get_total_items(&self) -> u64 {
        self.total_items
    }

    /// Data
    fn get_values(&self) -> Vec<Conversation> {
        self.conversations.clone()
    }
}

impl Default for CollectionConversations {
    fn default() -> Self {
        CollectionConversations {
            conversations: Vec::new(),
            total_items: 0,
            _links: Vec::new(),
        }
    }
}

///
/// Conversations Filter
///
#[derive(Debug, Clone)]
pub struct ConversationsFilter {
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
    /// Whether the conversation has any unread messages. Posible value "true" or "false"
    pub has_unread_messages: Option<String>,
    /// The unique id for the list.
    pub list_id: Option<String>,
    /// The unique id for the campaign.
    pub campaign_id: Option<u64>,
}

impl Default for ConversationsFilter {
    fn default() -> Self {
        ConversationsFilter {
            fields: None,
            exclude_fields: None,
            count: Some(50),
            offset: Some(0),
            has_unread_messages: None,
            list_id: None,
            campaign_id: None,
        }
    }
}

impl ResourceFilter for ConversationsFilter {
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

        if self.has_unread_messages.is_some() {
            payload.insert(
                "has_unread_messages".to_string(),
                format!("{:}", self.has_unread_messages.as_ref().unwrap().clone()),
            );
        }
        if self.list_id.is_some() {
            payload.insert(
                "list_id".to_string(),
                format!("{:}", self.list_id.as_ref().unwrap().clone()),
            );
        }
        if self.campaign_id.is_some() {
            payload.insert(
                "campaign_id".to_string(),
                format!("{:}", self.campaign_id.as_ref().unwrap().clone()),
            );
        }
        payload
    }
}

///
/// Conversation Builder
///
#[derive(Debug)]
pub struct ConversationBuilder {}

impl BuildIter for ConversationBuilder {
    type Item = Conversation;
    type FilterItem = ConversationsFilter;
    type Collection = CollectionConversations;

    ///
    /// Return a new data updated
    ///
    fn update_item(&self, data: &Self::Item, api: Rc<MailchimpApi>) -> Self::Item {
        let mut in_data = data.clone();
        in_data.set_api(api);
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
