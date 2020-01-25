use super::empty::EmptyType;
use super::link::LinkType;
use super::list_member_activity::{CollectionListMemberActivity, ListMemberActivityBuilder};
use super::list_member_goals::{CollectionListMemberGoal, ListMemberGoalBuilder};
use super::list_member_notes::{CollectionListMemberNote, ListMemberNote, ListMemberNoteBuilder};
use super::list_member_tags::{
    CollectionListMemberTag, ListMemberTagBuilder, ListMemberTagParam, ListMemberTagType,
};
use crate::api::MailchimpApi;
use crate::internal::error_type::MailchimpErrorType;
use crate::internal::request::MailchimpResult;
use crate::iter::{BuildIter, MailchimpCollection, MalchimpIter, ResourceFilter, SimpleFilter};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

use log::error;

///
/// The most recent Note added about this member.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListNote {
    /// The note id.
    #[serde(default)]
    pub note_id: u64,
    /// The date and time the note was created in ISO 8601 format.
    #[serde(default)]
    pub created_at: String,
    /// The author of the note.
    #[serde(default)]
    pub created_by: String,
    /// The content of the note.
    #[serde(default)]
    pub note: String,
}

impl Default for ListNote {
    fn default() -> Self {
        ListNote {
            note_id: 0,
            created_at: "".to_string(),
            created_by: "".to_string(),
            note: "".to_string(),
        }
    }
}

///
/// The marketing permissions for the subscriber.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListMarketingPermision {
    /// The id for the marketing permission on the list
    #[serde(default)]
    pub marketing_permission_id: String,
    /// The text of the marketing permission.
    #[serde(default)]
    pub text: String,
    /// If the subscriber has opted-in to the marketing permission.
    #[serde(default)]
    pub enabled: bool,
}

impl Default for ListMarketingPermision {
    fn default() -> Self {
        ListMarketingPermision {
            marketing_permission_id: "".to_string(),
            text: "".to_string(),
            enabled: false,
        }
    }
}

///
/// Returns up to 50 tags applied to this member. To retrieve all tags see `get_tags`.
///
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct MemberTag {
    id: u64,
    name: String,
}

///
/// Open and click rates for this subscriber.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriberStats {
    /// A subscriber’s average open rate.
    #[serde(default)]
    pub avg_open_rate: f32,
    /// A subscriber’s average clickthrough rate.
    #[serde(default)]
    pub avg_click_rate: f32,
}

impl Default for SubscriberStats {
    fn default() -> Self {
        SubscriberStats {
            avg_open_rate: 0.0,
            avg_click_rate: 0.0,
        }
    }
}

///
/// Subscriber location information.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriberLocation {
    /// The location latitude.
    #[serde(default)]
    pub latitude: f32,
    /// The location longitude.
    #[serde(default)]
    pub longitude: f32,
    /// The time difference in hours from GMT.
    #[serde(default)]
    pub gmtoff: i64,
    /// The offset for timezones where daylight saving time is observed.
    #[serde(default)]
    pub dstoff: i64,
    /// The unique code for the location country.
    #[serde(default)]
    pub country_code: String,
    /// The timezone for the location.
    #[serde(default)]
    pub timezone: String,
}

impl Default for SubscriberLocation {
    fn default() -> Self {
        SubscriberLocation {
            latitude: 0.0,
            longitude: 0.0,
            gmtoff: 0,
            dstoff: 0,
            country_code: "".to_string(),
            timezone: "".to_string(),
        }
    }
}

// ============ List Locations ==============
///
/// Get information about a specific list member, including a currently subscribed,
/// unsubscribed, or bounced member.
///
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListMember {
    /// The MD5 hash of the lowercase version of the list member’s email address.
    #[serde(default)]
    pub id: String,
    /// Email address for a subscriber.
    #[serde(default)]
    pub email_address: String,
    /// An identifier for the address across all of Mailchimp.
    #[serde(default)]
    pub unique_email_id: String,
    /// Type of email this member asked to get (‘html’ or ‘text’).
    #[serde(default)]
    pub email_type: String,
    /// Subscriber’s current status.
    #[serde(default)]
    pub status: String,
    /// A subscriber’s reason for unsubscribing.
    #[serde(default)]
    pub unsubscribe_reason: String,
    /// An individual merge var and value for a member.
    #[serde(default)]
    pub merge_fields: HashMap<String, String>,
    /// The key of this object’s properties is the ID of the interest in question.
    #[serde(default)]
    pub interests: HashMap<String, String>,
    /// Open and click rates for this subscriber.
    #[serde(default)]
    pub stats: SubscriberStats,
    /// The date and time the subscriber signed up for the list in ISO 8601 format.
    #[serde(default)]
    pub timestamp_signup: String,
    /// The IP address the subscriber used to confirm their opt-in status.
    #[serde(default)]
    pub ip_opt: String,
    /// The date and time the subscribe confirmed their opt-in status in ISO 8601 format.
    #[serde(default)]
    pub timestamp_opt: String,
    /// Star rating for this member, between 1 and 5.
    #[serde(default)]
    pub member_rating: u64,
    /// The date and time the member’s info was last changed in ISO 8601 format.
    #[serde(default)]
    pub last_changed: String,
    /// If set/detected, the subscriber’s language.
    #[serde(default)]
    pub language: String,
    /// VIP status for subscriber.
    #[serde(default)]
    pub vip: bool,
    /// The list member’s email client.
    #[serde(default)]
    pub email_client: String,
    /// Subscriber location information.
    #[serde(default)]
    pub location: SubscriberLocation,
    /// The marketing permissions for the subscriber.
    #[serde(default)]
    pub marketing_permissions: Vec<ListMarketingPermision>,
    /// The most recent Note added about this member.
    #[serde(default)]
    pub last_note: ListNote,
    /// The number of tags applied to this member.
    #[serde(default)]
    pub tags_count: u64,
    /// The tags applied to this member.
    #[serde(default)]
    pub tags: Vec<MemberTag>,
    /// IP address the subscriber signed up from.
    #[serde(default)]
    pub ip_signup: String,
    /// The list id.
    #[serde(default)]
    pub list_id: String,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,

    /// Mailchimp API
    #[serde(skip)]
    _api: Rc<MailchimpApi>,
    /// Endpoint Base for the instance
    #[serde(skip)]
    _endpoint: String,
}

impl ListMember {
    // ============== Actions ==============
    ///
    /// Permanently delete a list member
    ///
    /// Delete all personally identifiable information related to a list member, and
    /// remove them from a list. This will make it impossible to re-import the list member.
    ///
    ///
    pub fn permanently_delete(&self) -> Option<MailchimpErrorType> {
        // POST /lists/{list_id}/members/{subscriber_hash}/actions/delete-permanent
        let mut b_endpoint = self.build_list_endpoint();
        b_endpoint.push_str("/actions/delete-permanent");
        match self
            ._api
            .post::<EmptyType, HashMap<String, String>>(b_endpoint.as_str(), HashMap::new())
        {
            Ok(_) => None,
            Err(e) => Some(e),
        }
    }

    ///
    /// Delete a member from a list/segment
    ///
    pub fn archive(&self) -> Option<MailchimpErrorType> {
        // DELETE /lists/{list_id}/members/{subscriber_hash}
        let b_endpoint = self.build_list_endpoint();
        match self
            ._api
            .delete::<EmptyType>(b_endpoint.as_str(), HashMap::new())
        {
            Ok(_) => None,
            Err(e) => Some(e),
        }
    }

    ///
    /// Delete a member from a list/segment
    ///
    pub fn delete(&self) -> Option<MailchimpErrorType> {
        // DELETE /lists/{list_id}/members/{subscriber_hash}
        let b_endpoint = self._endpoint.clone();
        match self
            ._api
            .delete::<EmptyType>(b_endpoint.as_str(), HashMap::new())
        {
            Ok(_) => None,
            Err(e) => Some(e),
        }
    }

    /**
     * Update API
     */
    pub fn set_api(&mut self, n_api: Rc<MailchimpApi>) {
        self._api = n_api
    }

    ///
    /// Set new value for endpoint
    ///
    /// Argumentos:
    ///     n_endpoint: Nuevo Endpoint
    ///
    pub fn set_endpoint<'a>(&mut self, n_endpoint: &'a str) {
        self._endpoint = n_endpoint.to_string();
    }

    ///
    /// Get resource endpoint
    ///
    ///
    pub fn get_base_endpoint(&self) -> String {
        let mut endpoint = self._endpoint.clone() + "/";
        endpoint.push_str(&self.id);
        endpoint
    }

    fn build_list_endpoint(&self) -> String {
        format!("/list/{}/members/{}", self.list_id, self.id)
    }

    ///
    /// Get details about subscribers’ recent activity.
    ///
    pub fn get_activity(&self) -> MalchimpIter<ListMemberActivityBuilder> {
        // GET /lists/{list_id}/members/{subscriber_hash}/activity
        let mut endpoint = self.get_base_endpoint();
        endpoint.push_str("/activity");
        let filter_params = SimpleFilter::default();

        match self
            ._api
            .get::<CollectionListMemberActivity>(&endpoint, filter_params.build_payload())
        {
            Ok(collection) => MalchimpIter {
                builder: ListMemberActivityBuilder {},
                data: collection.activity,
                cur_filters: filter_params.clone(),
                cur_it: 0,
                total_items: collection.total_items,
                api: self._api.clone(),
                endpoint: endpoint.clone(),
            },
            Err(e) => {
                error!( target: "mailchimp",  "Get List Members: Response Error details: {:?}", e);
                MalchimpIter {
                    builder: ListMemberActivityBuilder {},
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
    /// Get information about recent goal events for a specific list member.
    ///
    /// Arguments:
    ///     subscriber_hash: The MD5 hash of the lowercase version of the list member’s email address.
    ///
    pub fn get_goals(&self) -> MalchimpIter<ListMemberGoalBuilder> {
        // GET  /lists/{list_id}/members/{subscriber_hash}/goals
        let mut endpoint = self.get_base_endpoint().to_string() + "/";
        endpoint.push_str(&self.id);
        endpoint.push_str("/goals");
        let filter_params = SimpleFilter::default();

        match self
            ._api
            .get::<CollectionListMemberGoal>(&endpoint, filter_params.build_payload())
        {
            Ok(collection) => MalchimpIter {
                builder: ListMemberGoalBuilder {},
                data: collection.goals,
                cur_filters: filter_params.clone(),
                cur_it: 0,
                total_items: collection.total_items,
                api: self._api.clone(),
                endpoint: endpoint.clone(),
            },
            Err(e) => {
                error!( target: "mailchimp",  "Get List Members: Response Error details: {:?}", e);
                MalchimpIter {
                    builder: ListMemberGoalBuilder {},
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
    /// Get the tags on a list member.
    ///
    pub fn get_tags(&self) -> MalchimpIter<ListMemberTagBuilder> {
        // GET /lists/{list_id}/members/{subscriber_hash}/tags
        let mut endpoint = self.get_base_endpoint();
        endpoint.push_str("/tags");
        let filter_params = SimpleFilter::default();

        match self
            ._api
            .get::<CollectionListMemberTag>(&endpoint, filter_params.build_payload())
        {
            Ok(collection) => MalchimpIter {
                builder: ListMemberTagBuilder {},
                data: collection.tags,
                cur_filters: filter_params.clone(),
                cur_it: 0,
                total_items: collection.total_items,
                api: self._api.clone(),
                endpoint: endpoint.clone(),
            },
            Err(e) => {
                error!( target: "mailchimp",  "Get List Members: Response Error details: {:?}", e);
                MalchimpIter {
                    builder: ListMemberTagBuilder {},
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
    /// Post the tags on a list member.
    ///
    /// Add or remove tags from a list member. If a tag that does not exist is passed in
    /// and set as ‘active’, a new tag will be created.
    ///
    pub fn post_tag(&self, tags: Vec<ListMemberTagType>) -> Option<MailchimpErrorType> {
        // POST /lists/{list_id}/members/{subscriber_hash}/tags
        let mut endpoint = self.get_base_endpoint();
        endpoint.push_str("/tags");
        let param = ListMemberTagParam { tags: tags };

        match self
            ._api
            .post::<EmptyType, ListMemberTagParam>(&endpoint, param)
        {
            Ok(_) => None,
            Err(e) => Some(e),
        }
    }

    ///
    /// Get recent notes for a specific list member
    ///
    /// Arguments:
    ///     filters
    ///
    pub fn get_notes(&self, filters: Option<SimpleFilter>) -> MalchimpIter<ListMemberNoteBuilder> {
        // GET /lists/{list_id}/members/{subscriber_hash}/notes
        let mut endpoint = self.get_base_endpoint();
        endpoint.push_str("/notes");

        let filter_params = if let Some(f) = filters {
            f
        } else {
            SimpleFilter::default()
        };

        match self
            ._api
            .get::<CollectionListMemberNote>(&endpoint, filter_params.build_payload())
        {
            Ok(collection) => MalchimpIter {
                builder: ListMemberNoteBuilder {
                    endpoint: endpoint.clone(),
                },
                data: collection.notes,
                cur_filters: filter_params.clone(),
                cur_it: 0,
                total_items: collection.total_items,
                api: self._api.clone(),
                endpoint: endpoint.clone(),
            },
            Err(e) => {
                error!( target: "mailchimp",  "Get List Members: Response Error details: {:?}", e);
                MalchimpIter {
                    builder: ListMemberNoteBuilder {
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
    /// Get details about subscribers’ recent activity.
    ///
    /// Argument:
    ///     note_id: The id for the note.
    ///
    pub fn get_specific_note<'a>(&self, note_id: &'a str) -> MailchimpResult<ListMemberNote> {
        // GET /lists/{list_id}/members/{subscriber_hash}/notes/{note_id}
        let mut endpoint = self.get_base_endpoint();
        endpoint.push_str("/notes/");
        endpoint.push_str(note_id);

        self._api.get::<ListMemberNote>(&endpoint, HashMap::new())
    }

    ///
    /// Add a new note for a specific subscriber.
    ///
    /// Argument:
    ///     note: The content of the note. Note length is limited to 1,000 characters.
    ///
    pub fn create_note<'a>(&self, note: &'a str) -> MailchimpResult<ListMemberNote> {
        // POST /lists/{list_id}/members/{subscriber_hash}/notes
        let mut endpoint = self.get_base_endpoint();
        endpoint.push_str("/notes");
        let mut payload = HashMap::new();
        payload.insert("note".to_string(), note.to_string());
        self._api
            .post::<ListMemberNote, HashMap<String, String>>(&endpoint, payload)
    }
}

///
/// Get information about members in a specific Mailchimp list.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionListMembers {
    /// An array of objects, each representing a specific list member.
    #[serde(default)]
    pub members: Vec<ListMember>,
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

impl MailchimpCollection<ListMember> for CollectionListMembers {
    /// Total Items
    fn get_total_items(&self) -> u64 {
        self.total_items
    }

    /// Data
    fn get_values(&self) -> Vec<ListMember> {
        self.members.clone()
    }
}

impl Default for CollectionListMembers {
    fn default() -> Self {
        CollectionListMembers {
            list_id: String::new(),
            members: Vec::new(),
            total_items: 0,
            _links: Vec::new(),
        }
    }
}

///
/// List Members Filter
///
#[derive(Debug, Clone)]
pub struct ListMembersFilter {
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
    /// The email type.
    pub email_type: Option<String>,
    /// The subscriber’s status.
    pub status: Option<String>,
    /// Restrict results to subscribers who opted-in after the set timeframe.
    /// We recommend ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    pub since_timestamp_opt: Option<String>,
    /// Restrict results to subscribers who opted-in before the set timeframe.
    /// We recommend ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    pub before_timestamp_opt: Option<String>,
    /// Restrict results to subscribers whose information changed after the set timeframe.
    /// We recommend ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    pub since_last_changed: Option<String>,
    /// Restrict results to subscribers whose information changed before the set timeframe.
    /// We recommend ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    pub before_last_changed: Option<String>,
    /// A unique identifier for the email address across all Mailchimp lists.
    /// This parameter can be found in any links with Ecommerce Tracking enabled.
    pub unique_email_id: Option<String>,
    /// A filter to return only the list’s VIP members. Passing true will restrict results
    /// to VIP list members, passing false will return all list members.
    pub vip_only: Option<String>,
    /// The unique id for the interest category.
    pub interest_category_id: Option<String>,
    /// Used to filter list members by interests. Must be accompanied by interest_category_id
    /// and interest_match. The value must be a comma separated list of interest ids present for
    /// any supplied interest categories.
    pub interest_ids: Option<String>,
    /// Used to filter list members by interests. Must be accompanied by interest_category_id and
    /// interest_ids. “any” will match a member with any of the interest supplied, “all” will only
    /// match members with every interest supplied, and “none” will match members without any of the
    /// interest supplied.
    pub interest_match: Option<String>,
    /// Returns files sorted by the specified field. [timestamp_opt, timestamp_signup]
    pub sort_field: Option<String>,
    /// Determines the order direction for sorted results.
    pub sort_dir: Option<String>,
    /// Filter subscribers by those subscribed/unsubscribed/pending/cleaned since
    /// last email campaign send. Member status is required to use this filter.
    pub since_last_campaign: Option<bool>,
    /// Filter subscribers by those unsubscribed since a specific date. Using any status
    /// other than unsubscribed with this filter will result in an error.
    pub unsubscribed_since: Option<String>,
}

impl Default for ListMembersFilter {
    fn default() -> Self {
        ListMembersFilter {
            fields: None,
            exclude_fields: None,
            count: Some(50),
            offset: Some(0),
            email_type: None,
            status: None,
            since_timestamp_opt: None,
            before_timestamp_opt: None,
            since_last_changed: None,
            before_last_changed: None,
            unique_email_id: None,
            vip_only: None,
            interest_category_id: None,
            interest_ids: None,
            interest_match: None,
            sort_field: None,
            sort_dir: None,
            since_last_campaign: None,
            unsubscribed_since: None,
        }
    }
}

impl ResourceFilter for ListMembersFilter {
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
        if self.email_type.is_some() {
            payload.insert(
                "email_type".to_string(),
                format!("{:}", self.email_type.as_ref().unwrap().clone()),
            );
        }
        if self.status.is_some() {
            payload.insert(
                "status".to_string(),
                format!("{:}", self.status.as_ref().unwrap().clone()),
            );
        }
        if self.since_timestamp_opt.is_some() {
            payload.insert(
                "since_timestamp_opt".to_string(),
                format!("{:}", self.since_timestamp_opt.as_ref().unwrap().clone()),
            );
        }
        if self.before_timestamp_opt.is_some() {
            payload.insert(
                "before_timestamp_opt".to_string(),
                format!("{:}", self.before_timestamp_opt.as_ref().unwrap().clone()),
            );
        }
        if self.since_last_changed.is_some() {
            payload.insert(
                "since_last_changed".to_string(),
                format!("{:}", self.since_last_changed.as_ref().unwrap().clone()),
            );
        }
        if self.before_last_changed.is_some() {
            payload.insert(
                "before_last_changed".to_string(),
                format!("{:}", self.before_last_changed.as_ref().unwrap().clone()),
            );
        }
        if self.unique_email_id.is_some() {
            payload.insert(
                "unique_email_id".to_string(),
                format!("{:}", self.unique_email_id.as_ref().unwrap().clone()),
            );
        }
        if self.vip_only.is_some() {
            payload.insert(
                "vip_only".to_string(),
                format!("{:}", self.vip_only.as_ref().unwrap().clone()),
            );
        }
        if self.interest_category_id.is_some() {
            payload.insert(
                "interest_category_id".to_string(),
                format!("{:}", self.interest_category_id.as_ref().unwrap().clone()),
            );
        }
        if self.interest_ids.is_some() {
            payload.insert(
                "interest_ids".to_string(),
                format!("{:}", self.interest_ids.as_ref().unwrap().clone()),
            );
        }
        if self.interest_match.is_some() {
            payload.insert(
                "interest_match".to_string(),
                format!("{:}", self.interest_match.as_ref().unwrap().clone()),
            );
        }
        if self.sort_field.is_some() {
            payload.insert(
                "sort_field".to_string(),
                format!("{:}", self.sort_field.as_ref().unwrap().clone()),
            );
        }
        if self.sort_dir.is_some() {
            payload.insert(
                "sort_dir".to_string(),
                format!("{:}", self.sort_dir.as_ref().unwrap().clone()),
            );
        }
        if self.since_last_campaign.is_some() {
            payload.insert(
                "since_last_campaign".to_string(),
                format!("{:}", self.since_last_campaign.as_ref().unwrap().clone()),
            );
        }
        if self.unsubscribed_since.is_some() {
            payload.insert(
                "unsubscribed_since".to_string(),
                format!("{:}", self.unsubscribed_since.as_ref().unwrap().clone()),
            );
        }
        payload
    }
}

///
/// List Members Builder
///
#[derive(Debug)]
pub struct ListMembersBuilder {
    /// Resource Endpoint
    pub endpoint: String,
}

impl BuildIter for ListMembersBuilder {
    type Item = ListMember;
    type FilterItem = ListMembersFilter;
    type Collection = CollectionListMembers;

    ///
    /// Crea un recurso a partir del dato pasado por parámetro
    ///
    fn update_item(&self, data: &Self::Item, api: Rc<MailchimpApi>) -> Self::Item {
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
/// ListMemberParams
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListMemberParams {
    /// Email address for a subscriber.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// Subscriber’s status. This value is required only if the email address is not
    /// already present on the list.
    /// Possible Values:
    ///     * subscribed
    ///     * unsubscribed
    ///     * cleaned
    ///     * pending
    ///     * transactional
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_if_new: Option<String>,
    /// Type of email this member asked to get (‘html’ or ‘text’).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_type: Option<String>,
    /// Subscriber’s current status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// An individual merge var and value for a member.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_fields: Option<HashMap<String, String>>,
    /// The key of this object’s properties is the ID of the interest in question.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interests: Option<HashMap<String, String>>,
    /// If set/detected, the subscriber’s language.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// VIP status for subscriber.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vip: Option<bool>,
    /// location
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<SubscriberLocation>,
    /// The marketing permissions for the subscriber.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub marketing_permissions: Option<Vec<ListMarketingPermision>>,
    /// The number of tags applied to this member.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags_count: Option<u64>,
    /// The tags applied to this member.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// IP address the subscriber signed up from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_signup: Option<String>,
    /// The date and time the subscriber signed up for the list in ISO 8601 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp_signup: Option<String>,
    /// The IP address the subscriber used to confirm their opt-in status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_opt: Option<String>,
    /// The date and time the subscribe confirmed their opt-in status in ISO 8601 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp_opt: Option<String>,
}
