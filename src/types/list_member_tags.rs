use super::link::LinkType;
use crate::api::MailchimpApi;
use crate::iter::{BuildIter, MailchimpCollection, SimpleFilter};

///
/// Response for endpoint  GET /lists/{list_id}/members/{subscriber_hash}/tags
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionListMemberTag {
    /// An array of objects, each representing a member event.
    #[serde(default)]
    pub tags: Vec<String>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(default)]
    pub total_items: u64,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

impl MailchimpCollection<String> for CollectionListMemberTag {
    /// Total Items
    fn get_total_items(&self) -> u64 {
        self.total_items
    }

    /// Data
    fn get_values(&self) -> Vec<String> {
        self.tags.clone()
    }
}

impl Default for CollectionListMemberTag {
    fn default() -> Self {
        CollectionListMemberTag {
            tags: Vec::new(),
            total_items: 0,
            _links: Vec::new(),
        }
    }
}

/// ================================= ITER =====================

///
/// ListMemberTagBuilder
///
#[derive(Debug)]
pub struct ListMemberTagBuilder {}

impl BuildIter for ListMemberTagBuilder {
    type Item = String;
    type FilterItem = SimpleFilter;
    type Collection = CollectionListMemberTag;

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


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListMemberTagType {
    /// The name of the tag.
    #[serde(default)]
    pub name: String,
    /// The status for the tag on the member, pass in active to add a tag or inactive to remove it.
    /// inactive - active
    #[serde(default)]
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListMemberTagParam {
    /// A list of tags assigned to the list member.
    #[serde(default)]
    pub tags: Vec<ListMemberTagType>,
}
