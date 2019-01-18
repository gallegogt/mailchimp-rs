use super::link::LinkType;
use super::list_members::ListMember;
use crate::api::MailchimpApi;
use crate::iter::{BuildIter, MailchimpCollection, SimpleFilter};

///
/// Response for endpoint GET /lists/{list_id}/segments/{segment_id}/members
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionListSegmentMembers {
    /// An array of objects, each representing a specific list member.
    #[serde(default)]
    pub members: Vec<ListMember>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(default)]
    pub total_items: u64,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

impl MailchimpCollection<ListMember> for CollectionListSegmentMembers {
    /// Total Items
    fn get_total_items(&self) -> u64 {
        self.total_items
    }

    /// Data
    fn get_values(&self) -> Vec<ListMember> {
        self.members.clone()
    }
}

impl Default for CollectionListSegmentMembers {
    fn default() -> Self {
        CollectionListSegmentMembers {
            members: Vec::new(),
            total_items: 0,
            _links: Vec::new(),
        }
    }
}

/// ================================= ITER =====================

///
/// ListSegmentMembersBuilder
///
#[derive(Debug)]
pub struct ListSegmentMembersBuilder {
    pub endpoint: String,
}

impl BuildIter for ListSegmentMembersBuilder {
    type Item = ListMember;
    type FilterItem = SimpleFilter;
    type Collection = CollectionListSegmentMembers;

    ///
    /// Crea un recurso a partir del dato pasado por parámetro
    ///
    fn update_item(&self, data: &Self::Item, api: &MailchimpApi) -> Self::Item {
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
