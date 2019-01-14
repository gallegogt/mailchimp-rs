use super::link::LinkType;
use crate::api::MailchimpApi;
use crate::iter::{BuildIter, MailchimpCollection, SimpleFilter};

///
/// List Member Activity
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListMemberActivity {
    /// The type of action recorded for the subscriber.
    #[serde(default)]
    pub action: String,
    /// The date and time recorded for the action.
    #[serde(default)]
    pub timestamp: String,
    /// For clicks, the URL the subscriber clicked on.
    #[serde(default)]
    pub url: String,
    /// The type of campaign that was sent.
    #[serde(default, rename="type")]
    pub activity_type: String,
    /// The web-based ID for the campaign.
    #[serde(default)]
    pub campaign_id: String,
    /// If set, the campaign’s title.
    #[serde(default)]
    pub title: String,
    /// The ID of the parent campaign.
    #[serde(default)]
    pub parent_campaign: String,
}

///
/// Response for endpoint  GET /lists/{list_id}/members/{subscriber_hash}/activity
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionListMemberActivity {
    /// An array of objects, each representing a member event.
    #[serde(default)]
    pub activity: Vec<ListMemberActivity>,
    /// The unique id for the list.
    #[serde(default)]
    pub list_id: String,
    /// The MD5 hash of the lowercase version of the list member’s email address.
    #[serde(default)]
    pub email_id: String,
    /// The total number of items matching the query regardless of pagination.
    #[serde(default)]
    pub total_items: u64,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

impl MailchimpCollection<ListMemberActivity> for CollectionListMemberActivity {
    /// Total Items
    fn get_total_items(&self) -> u64 {
        self.total_items
    }

    /// Data
    fn get_values(&self) -> Vec<ListMemberActivity> {
        self.activity.clone()
    }
}

impl Default for CollectionListMemberActivity {
    fn default() -> Self {
        CollectionListMemberActivity {
            list_id: "".to_string(),
            email_id: "".to_string(),
            activity: Vec::new(),
            total_items: 0,
            _links: Vec::new(),
        }
    }
}

/// ================================= ITER =====================

///
/// ListMemberActivityBuilder
///
#[derive(Debug)]
pub struct ListMemberActivityBuilder {}

impl BuildIter for ListMemberActivityBuilder {
    type Item = ListMemberActivity;
    type FilterItem = SimpleFilter;
    type Collection = CollectionListMemberActivity;

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
