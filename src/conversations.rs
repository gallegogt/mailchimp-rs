use super::api::MailchimpApi;
use super::internal::request::MailchimpResult;
use super::types::{
    CollectionConversations, Conversation, ConversationBuilder, ConversationsFilter,
};
use crate::iter::{MalchimpIter, ResourceFilter};
use log::error;
use std::collections::HashMap;
use std::rc::Rc;

///
/// Conversations
///
/// Conversation tracking is a paid feature that lets you view subscribers’
/// replies to your campaigns in your Mailchimp account.
///
#[derive(Debug, Clone)]
pub struct Conversations {
    api: Rc<MailchimpApi>,
}

impl Conversations {
    ///
    /// Argumentos:
    ///     api: MailchimpApi
    ///
    pub fn new(api: MailchimpApi) -> Self {
        Conversations { api: Rc::new(api) }
    }

    ///
    /// Get a list of conversations
    ///
    pub fn get_conversations(
        &self,
        filter: Option<ConversationsFilter>,
    ) -> MalchimpIter<ConversationBuilder> {
        // GET /conversations
        let endpoint = "conversations";
        let mut filter_params = ConversationsFilter::default();

        if let Some(f) = filter {
            filter_params = f;
        }

        match self
            .api
            .get::<CollectionConversations>(&endpoint, filter_params.build_payload())
        {
            Ok(collection) => MalchimpIter {
                builder: ConversationBuilder {},
                data: collection.conversations,
                cur_filters: filter_params.clone(),
                cur_it: 0,
                total_items: collection.total_items,
                api: self.api.clone(),
                endpoint: endpoint.to_string(),
            },
            Err(e) => {
                error!( target: "mailchimp",  "Get Activities: Response Error details: {:?}", e);
                MalchimpIter {
                    builder: ConversationBuilder {},
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
    /// Get information about a conversation
    ///
    /// Get details about an individual conversation.
    ///
    pub fn get_conversation<'a>(&self, conversation_id: &'a str) -> MailchimpResult<Conversation> {
        let endpoint = format!("conversations/{}", conversation_id);
        let mut payload = HashMap::new();
        payload.insert("conversation_id".to_string(), conversation_id.to_string());
        self.api.get::<Conversation>(&endpoint, payload)
    }
}
