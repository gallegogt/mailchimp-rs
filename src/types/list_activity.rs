use super::link::LinkType;
use crate::api::MailchimpApi;
use crate::iter::{BuildIter, MailchimpCollection, SimpleFilter};

// ============ List Activity ==============
///
///  Get up to the previous 180 days of daily detailed aggregated activity stats
/// for a list, not including Automation activity.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListActivityType {
    /// The date for the activity summary.
    #[serde(default)]
    pub day: String,
    /// The total number of emails sent on the date for the activity summary
    #[serde(default)]
    pub emails_sent: u64,
    /// The number of unique opens
    #[serde(default)]
    pub unique_opens: u64,
    /// The number of clicks
    #[serde(default)]
    pub recipient_clicks: u64,
    /// The number of hard bounces
    #[serde(default)]
    pub hard_bounce: u64,
    /// The number of soft bounces
    #[serde(default)]
    pub soft_bounce: u64,
    /// The number of subscribes
    #[serde(default)]
    pub subs: u64,
    /// The number of unsubscribes
    #[serde(default)]
    pub unsubs: u64,
    /// The number of subscribers who may have been added outside of the double
    /// opt-in process, such as imports or API activity.
    #[serde(default)]
    pub other_adds: u64,
    /// The number of subscribers who may have been removed outside of unsubscribing
    /// or reporting an email as spam (for example, deleted subscribers).
    #[serde(default)]
    pub other_removes: u64,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

/// GET /automations/{workflow_id}/emails/{workflow_email_id}/queue
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionListActivity {
    /// Recent list activity.
    #[serde(default)]
    pub activity: Vec<ListActivityType>,
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

impl MailchimpCollection<ListActivityType> for CollectionListActivity {
    /// Total Items
    fn get_total_items(&self) -> u64 {
        self.total_items
    }

    /// Data
    fn get_values(&self) -> Vec<ListActivityType> {
        self.activity.clone()
    }
}

impl Default for CollectionListActivity {
    fn default() -> Self {
        CollectionListActivity {
            list_id: String::new(),
            activity: Vec::new(),
            total_items: 0,
            _links: Vec::new(),
        }
    }
}

/// ================================= ITER =====================

#[derive(Debug)]
pub struct ListActivityBuilder {}

impl BuildIter for ListActivityBuilder {
    type Item = ListActivityType;
    type FilterItem = SimpleFilter;
    type Collection = CollectionListActivity;

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
