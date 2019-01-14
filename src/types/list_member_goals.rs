use super::link::LinkType;
use crate::api::MailchimpApi;
use crate::iter::{BuildIter, MailchimpCollection, SimpleFilter};

///
/// List Member Goal
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListMemberGoal {
    /// The id for a Goal event.
    #[serde(default)]
    pub goal_id: u64,
    /// The name/type of Goal event triggered.
    #[serde(default)]
    pub event: String,
    /// The date and time the user last triggered the Goal event in ISO 8601 format.
    #[serde(default)]
    pub last_visited_at: String,
    /// Any extra data passed with the Goal event.
    #[serde(default)]
    pub data: String,
}

///
/// Response for endpoint  GET /lists/{list_id}/members/{subscriber_hash}/goals
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionListMemberGoal {
    /// The last 50 Goal events triggered by a member.
    #[serde(default)]
    pub goals: Vec<ListMemberGoal>,
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

impl MailchimpCollection<ListMemberGoal> for CollectionListMemberGoal {
    /// Total Items
    fn get_total_items(&self) -> u64 {
        self.total_items
    }

    /// Data
    fn get_values(&self) -> Vec<ListMemberGoal> {
        self.goals.clone()
    }
}

impl Default for CollectionListMemberGoal {
    fn default() -> Self {
        CollectionListMemberGoal {
            list_id: "".to_string(),
            email_id: "".to_string(),
            goals: Vec::new(),
            total_items: 0,
            _links: Vec::new(),
        }
    }
}

/// ================================= ITER =====================

///
/// ListMemberGoalBuilder
///
#[derive(Debug)]
pub struct ListMemberGoalBuilder {}

impl BuildIter for ListMemberGoalBuilder {
    type Item = ListMemberGoal;
    type FilterItem = SimpleFilter;
    type Collection = CollectionListMemberGoal;

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
