use super::empty::EmptyType;
use super::link::LinkType;
use super::list_batch_members::ListBatchErrors;
use super::list_members::ListMember;
use super::list_segment_members::{CollectionListSegmentMembers, ListSegmentMembersBuilder};
use super::list_segment_options::SegmentOptionsType;
use crate::api::MailchimpApi;
use crate::internal::error_type::MailchimpErrorType;
use crate::internal::request::MailchimpResult;
use crate::iter::{BuildIter, MailchimpCollection, MalchimpIter, ResourceFilter, SimpleFilter};
use log::error;
use std::collections::HashMap;

///
/// List Segment
///
///
/// Manage segments and tags for a specific Mailchimp list. A segment
/// is a section of your list that includes only those subscribers who
/// share specific common field information. Tags are labels you create
/// to help organize your contacts. Learn more about segments and tags.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListSegment {
    /// The unique id for the segment.
    #[serde(default)]
    pub id: u64,
    /// The name of the segment.
    #[serde(default)]
    pub name: String,
    /// The number of active subscribers currently included in the segment.
    #[serde(default)]
    pub member_count: u64,
    /// The type of segment. Static segments are now known as tags. Learn more about tags.
    /// Possible Values: saved - static - fuzzy
    #[serde(default, rename = "type")]
    pub s_type: String,
    /// The date and time the segment was created in ISO 8601 format.
    #[serde(default)]
    pub created_at: String,
    /// The date and time the segment was last updated in ISO 8601 format.
    #[serde(default)]
    pub updated_at: String,
    /// The conditions of the segment. Static segments (tags) and fuzzy segments don’t have conditions.
    #[serde(default)]
    pub options: SegmentOptionsType,
    /// The list id.
    #[serde(default)]
    pub list_id: String,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,

    /// Mailchimp API
    #[serde(skip)]
    _api: MailchimpApi,
    /// Endpoint
    #[serde(skip)]
    _endpoint: String,
}

///
/// Edit/Create List Segment
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModifierListSegmentParams {
    /// The name of the segment.
    #[serde(default)]
    pub name: String,
    ///  An array of emails to be used for a static segment. Any emails provided
    /// that are not present on the list will be ignored. Passing an empty array
    /// for an existing static segment will reset that segment and remove all members.
    /// This field cannot be provided with the options field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub static_segment: Option<Vec<String>>,
    /// The conditions of the segment. Static and fuzzy segments don’t have conditions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<SegmentOptionsType>,
}

///
/// Batch add/remove list members to static segment
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BatchListMembersStaticSegment {
    /// An array of objects, each representing a new member that
    /// was added to the static segment.
    #[serde(default)]
    pub members_added: Vec<ListMember>,
    /// An array of objects, each representing an existing list
    /// member that got deleted from the static segment.
    #[serde(default)]
    pub members_removed: Vec<ListMember>,
    /// An array of objects, each representing an array of email
    /// addresses that could not be added to the segment or removed and
    /// an error message providing more details.
    #[serde(default)]
    pub errors: Vec<ListBatchErrors>,
    /// The total number of items matching the query, irrespective of pagination.
    #[serde(default)]
    pub total_added: u64,
    /// The total number of items matching the query, irrespective of pagination.
    #[serde(default)]
    pub total_removed: u64,
    /// The total number of items matching the query, irrespective of pagination.
    #[serde(default)]
    pub error_count: u64,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

///
/// Response for endpoint GET /lists/{list_id}/segments
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionListSegment {
    /// An array of objects, each representing a list segment.
    #[serde(default)]
    pub segments: Vec<ListSegment>,
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

impl MailchimpCollection<ListSegment> for CollectionListSegment {
    /// Total Items
    fn get_total_items(&self) -> u64 {
        self.total_items
    }

    /// Data
    fn get_values(&self) -> Vec<ListSegment> {
        self.segments.clone()
    }
}

impl Default for CollectionListSegment {
    fn default() -> Self {
        CollectionListSegment {
            list_id: "".to_string(),
            segments: Vec::new(),
            total_items: 0,
            _links: Vec::new(),
        }
    }
}

/// ================================= ITER =====================

#[derive(Debug, Clone)]
pub struct ListSegmentFilter {
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
    ///
    /// Limit results based on segment type.
    ///
    pub s_type: Option<String>,
    ///
    /// Restrict results to segments created after the set time.
    ///  We recommend ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    ///
    pub since_created_at: Option<String>,
    ///
    /// Restrict results to segments created before the set time.
    /// We recommend ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    ///
    pub before_created_at: Option<String>,
    ///
    /// Restrict results to segments update after the set time.
    /// We recommend ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    ///
    pub since_updated_at: Option<String>,
    ///
    /// Restrict results to segments update before the set time.
    /// We recommend ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    ///
    pub before_updated_at: Option<String>,
}

impl Default for ListSegmentFilter {
    fn default() -> Self {
        ListSegmentFilter {
            fields: None,
            exclude_fields: None,
            count: Some(50),
            offset: Some(0),
            s_type: None,
            since_created_at: None,
            before_created_at: None,
            since_updated_at: None,
            before_updated_at: None,
        }
    }
}

impl ResourceFilter for ListSegmentFilter {
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
        if self.s_type.is_some() {
            payload.insert(
                "s_type".to_string(),
                format!("{:}", self.s_type.as_ref().unwrap().clone()),
            );
        }
        if self.since_created_at.is_some() {
            payload.insert(
                "since_created_at".to_string(),
                format!("{:}", self.since_created_at.as_ref().unwrap().clone()),
            );
        }
        if self.before_created_at.is_some() {
            payload.insert(
                "before_created_at".to_string(),
                format!("{:}", self.before_created_at.as_ref().unwrap().clone()),
            );
        }
        if self.since_updated_at.is_some() {
            payload.insert(
                "since_updated_at".to_string(),
                format!("{:}", self.since_updated_at.as_ref().unwrap().clone()),
            );
        }
        if self.before_updated_at.is_some() {
            payload.insert(
                "before_updated_at".to_string(),
                format!("{:}", self.before_updated_at.as_ref().unwrap().clone()),
            );
        }
        payload
    }
}

///
/// ListSegmentBuilder
///
#[derive(Debug)]
pub struct ListSegmentBuilder {
    pub endpoint: String,
}

impl BuildIter for ListSegmentBuilder {
    type Item = ListSegment;
    type FilterItem = ListSegmentFilter;
    type Collection = CollectionListSegment;

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

///
/// Implement
///
impl ListSegment {
    ///
    /// Update a specific segment in a list.
    ///
    pub fn update(&self, param: ModifierListSegmentParams) -> MailchimpResult<ListSegment> {
        // PATCH /lists/{list_id}/segments/{segment_id}
        let endpoint = self.get_base_endpoint();
        self._api
            .patch::<ListSegment, ModifierListSegmentParams>(&endpoint, param)
    }

    ///
    /// Batch add/remove list members to static segment
    ///
    /// Arguments:
    ///     members_to_add: An array of emails to be used for a static segment.
    ///         Any emails provided that are not present on the list will be ignored.
    ///     members_to_remove: An array of emails to be used for a static segment.
    ///         Any emails provided that are not present on the list will be ignored.
    ///
    pub fn create_bacth_to_static_segment(
        &self,
        members_to_add: Vec<String>,
        members_to_remove: Vec<String>,
    ) -> MailchimpResult<BatchListMembersStaticSegment> {
        // POST /lists/{list_id}/segments/{segment_id}
        let endpoint = self.get_base_endpoint();
        let mut payload = HashMap::new();
        payload.insert("members_to_add".to_string(), members_to_add);
        payload.insert("members_to_remove".to_string(), members_to_remove);
        self._api
            .post::<BatchListMembersStaticSegment, HashMap<String, Vec<String>>>(&endpoint, payload)
    }

    ///
    /// Delete a specific segment in a list.
    ///
    pub fn delete(&self) -> Option<MailchimpErrorType> {
        // DELETE /lists/{list_id}/segments/{segment_id}
        let endpoint = self.get_base_endpoint();
        match self._api.delete::<EmptyType>(&endpoint, HashMap::new()) {
            Ok(_) => None,
            Err(e) => Some(e),
        }
    }

    ///
    /// Get information about all members in a list segment
    ///
    pub fn get_members_informations(
        &self,
        filter: Option<SimpleFilter>,
    ) -> MalchimpIter<ListSegmentMembersBuilder> {
        // GET /lists/{list_id}/segments/{segment_id}/members
        let mut endpoint = self.get_base_endpoint();
        endpoint.push_str("/members");
        println!("{}", endpoint);
        let filter_params = if let Some(f) = filter {
            f
        } else {
            SimpleFilter::default()
        };

        match self
            ._api
            .get::<CollectionListSegmentMembers>(&endpoint, filter_params.build_payload())
        {
            Ok(collection) => MalchimpIter {
                builder: ListSegmentMembersBuilder {
                    endpoint: endpoint.clone(),
                },
                data: collection.members,
                cur_filters: filter_params.clone(),
                cur_it: 0,
                total_items: collection.total_items,
                api: self._api.clone(),
                endpoint: endpoint.clone(),
            },
            Err(e) => {
                error!( target: "mailchimp",  "Get Information_members: Response Error details: {:?}", e);
                MalchimpIter {
                    builder: ListSegmentMembersBuilder {
                        endpoint: endpoint.clone(),
                    },
                    data: Vec::new(),
                    cur_filters: filter_params.clone(),
                    cur_it: 0,
                    total_items: 0,
                    api: self._api.clone(),
                    endpoint: endpoint.clone(),
                }
            }
        }
    }

    ///
    /// Remove a member from the specified static segment
    ///
    /// Arguments:
    ///    subscriber_hash: The MD5 hash of the lowercase version of the list member’s email address.
    ///
    pub fn remove_member<'a>(&self, subscriber_hash: &'a str) -> Option<MailchimpErrorType> {
        // DELETE /lists/{list_id}/segments/{segment_id}/members/{subscriber_hash}
        let mut endpoint = self.get_base_endpoint();
        endpoint.push_str("/members/");
        endpoint.push_str(subscriber_hash);

        match self._api.delete::<EmptyType>(&endpoint, HashMap::new()) {
            Ok(_) => None,
            Err(e) => Some(e),
        }
    }

    ///
    /// Add a member to a static segment.
    ///
    /// Arguments:
    ///    email_address: Email address for a subscriber.
    ///
    pub fn add_member<'a>(&self, email_address: &'a str) -> MailchimpResult<ListMember> {
        // POST /lists/{list_id}/segments/{segment_id}/members
        let mut endpoint = self.get_base_endpoint();
        endpoint.push_str("/members");
        let mut payload = HashMap::new();
        payload.insert("email_address".to_string(), email_address.to_string());

        match self
            ._api
            .post::<ListMember, HashMap<String, String>>(&endpoint, payload)
        {
            Ok(data) => {
                let mut n_data = data.clone();
                n_data.set_api(&self._api);
                n_data.set_endpoint(&endpoint);
                Ok(data)
            }
            Err(e) => Err(e),
        }
    }

    ///
    /// Set API
    ///
    pub fn set_api(&mut self, n_api: &MailchimpApi) {
        self._api = n_api.clone();
    }

    /// Set Endpoint
    pub fn set_endpoint<'a>(&mut self, n_endpoint: &'a str) {
        self._endpoint = n_endpoint.to_string();
    }

    ///
    /// Return de base endpoint for the resource
    ///
    fn get_base_endpoint(&self) -> String {
        let mut endpoint = self._endpoint.clone();
        endpoint.push_str(format!("/{}", &self.id).as_str());
        endpoint
    }
}
