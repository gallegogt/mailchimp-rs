use super::link::LinkType;
use crate::api::MailchimpApi;
use crate::iter::{BuildIter, MailchimpCollection, SimpleFilter};

// ============ List Clients ==============
///
/// Get information about the most popular email clients for subscribers in a specific Mailchimp list.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListClientType {
    /// The date for the activity summary.
    #[serde(default)]
    pub client: String,
    /// The total number of emails sent on the date for the activity summary
    #[serde(default)]
    pub members: u64,
}

/// GET /automations/{workflow_id}/emails/{workflow_email_id}/queue
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionListClients {
    /// Recent list activity.
    #[serde(default)]
    pub clients: Vec<ListClientType>,
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

impl MailchimpCollection<ListClientType> for CollectionListClients {
    /// Total Items
    fn get_total_items(&self) -> u64 {
        self.total_items
    }

    /// Data
    fn get_values(&self) -> Vec<ListClientType> {
        self.clients.clone()
    }
}

impl Default for CollectionListClients {
    fn default() -> Self {
        CollectionListClients {
            list_id: String::new(),
            clients: Vec::new(),
            total_items: 0,
            _links: Vec::new(),
        }
    }
}

/// ================================= ITER =====================

#[derive(Debug)]
pub struct ListClientsBuilder {}

impl BuildIter for ListClientsBuilder {
    type Item = ListClientType;
    type FilterItem = SimpleFilter;
    type Collection = CollectionListClients;

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
