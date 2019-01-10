use super::contact::ContactType;
use super::link::LinkType;
use super::list_activity::{CollectionListActivity, ListActivityBuilder};
use super::list_clients::{CollectionListClients, ListClientsBuilder};
use super::list_locations::{CollectionListLocations, ListLocationsBuilder};
use super::list_abuse_report::{CollectionListAbuseReport, ListAbuseReportBuilder, ListAbuseReportType};
use crate::api::{MailchimpApi, MailchimpApiUpdate};
use crate::iter::MailchimpCollection;
use crate::iter::{MalchimpIter, ResourceFilter, SimpleFilter};
use crate::internal::request::MailchimpResult;
use std::collections::HashMap;
use log::error;

// ============ Campaign Defaults	 ==============
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CampaignDefaultsType {
    /// The default from name for campaigns sent to this list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_name: Option<String>,
    /// The default from email for campaigns sent to this list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_email: Option<String>,
    /// The default subject line for campaigns sent to this list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// The default language for this lists’s forms.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl Default for CampaignDefaultsType {
    fn default() -> Self {
        CampaignDefaultsType {
            from_name: None,
            from_email: None,
            subject: None,
            language: None,
        }
    }
}

// ============ Statistics ==============
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatisticsType {
    /// The number of active members in the list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_count: Option<u64>,
    /// The number of members who have unsubscribed from the list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unsubscribe_count: Option<u64>,
    /// The number of members cleaned from the list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cleaned_count: Option<u64>,
    /// The number of active members in the list since the last campaign was sent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_count_since_send: Option<u64>,
    /// The number of members who have unsubscribed since the last campaign was sent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unsubscribe_count_since_send: Option<u64>,
    /// The number of members cleaned from the list since the last campaign was sent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cleaned_count_since_send: Option<u64>,
    /// The number of campaigns in any status that use this list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub campaign_count: Option<u64>,
    /// The date and time the last campaign was sent to this list in ISO 8601 format.
    ///  This is updated when a campaign is sent to 10 or more recipients.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub campaign_last_sent: Option<String>,
    /// The number of merge vars for this list (not EMAIL, which is required).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_field_count: Option<u64>,
    /// The average number of subscriptions per month for the list (not returned
    /// if we haven’t calculated it yet).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_sub_rate: Option<f32>,
    /// The average number of unsubscriptions per month for the list (not returned
    /// if we haven’t calculated it yet).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_unsub_rate: Option<f32>,
    /// The target number of subscriptions per month for the list to keep
    /// it growing (not returned if we haven’t calculated it yet).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_sub_rate: Option<f32>,
    /// The average open rate (a percentage represented as a number between
    /// 0 and 100) per campaign for the list (not returned if we haven’t calculated it yet).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_rate: Option<f32>,
    /// The average click rate (a percentage represented as a number between 0 and 100)
    /// per campaign for the list (not returned if we haven’t calculated it yet).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub click_rate: Option<f32>,
    /// The date and time of the last time someone subscribed to this list in ISO 8601 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_sub_date: Option<String>,
    /// The date and time of the last time someone unsubscribed from this list in ISO 8601 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_unsub_date: Option<String>,
}

impl Default for StatisticsType {
    fn default() -> Self {
        StatisticsType {
            member_count: None,
            unsubscribe_count: None,
            cleaned_count: None,
            member_count_since_send: None,
            unsubscribe_count_since_send: None,
            cleaned_count_since_send: None,
            campaign_count: None,
            campaign_last_sent: None,
            merge_field_count: None,
            avg_sub_rate: None,
            avg_unsub_rate: None,
            target_sub_rate: None,
            open_rate: None,
            click_rate: None,
            last_sub_date: None,
            last_unsub_date: None,
        }
    }
}

// ============ List ==============
// GET /lists/{list_id}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListType {
    /// A string that uniquely identifies this list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The ID used in the Mailchimp web application. View this list in
    /// your Mailchimp account at https://{dc}.admin.mailchimp.com/lists/members/?id={web_id}.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_id: Option<u64>,
    /// The name of the list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Contact information displayed in campaign footers to comply with international spam laws.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact: Option<ContactType>,
    /// The permission reminder for the list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission_reminder: Option<String>,
    /// Whether campaigns for this list use the Archive Bar in archives by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_archive_bar: Option<bool>,
    /// Default values for campaigns created for this list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub campaign_defaults: Option<CampaignDefaultsType>,
    /// The email address to send subscribe notifications to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notify_on_subscribe: Option<String>,
    /// The email address to send unsubscribe notifications to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notify_on_unsubscribe: Option<String>,
    /// The date and time that this list was created in ISO 8601 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// An auto-generated activity score for the list (0-5).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_rating: Option<u64>,
    /// Whether the list supports multiple formats for emails. When set to true,
    /// subscribers can choose whether they want to receive HTML or plain-text
    /// emails. When set to false, subscribers will receive HTML emails,
    ///  with a plain-text alternative backup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_type_option: Option<bool>,
    /// Our EepURL shortened version of this list’s subscribe form.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscribe_url_short: Option<String>,
    /// The full version of this list’s subscribe form (host will vary).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscribe_url_long: Option<String>,
    /// The list’s Email Beamer address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub beamer_address: Option<String>,
    /// Whether this list is public or private.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    /// Whether or not to require the subscriber to confirm subscription via email.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub double_optin: Option<bool>,
    /// Whether or not this list has a welcome automation connected. Welcome
    /// Automations: welcomeSeries, singleWelcome, emailFollowup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_welcome: Option<bool>,
    /// Whether or not the list has marketing permissions (eg. GDPR) enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub marketing_permissions: Option<bool>,
    /// Any list-specific modules installed for this list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modules: Option<Vec<String>>,
    /// Stats for the list. Many of these are cached for at least five minutes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<StatisticsType>,
    /// Desc: A list of link types and descriptions for the API schema documents.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _links: Option<Vec<LinkType>>,

    #[serde(skip)]
    _api: MailchimpApi,
}

///
/// MailchimpApiUpdate Impl
///
impl MailchimpApiUpdate for ListType {
    /**
     * Update API
     */
    fn set_api(&mut self, n_api: &MailchimpApi) {
        self._api = n_api.clone()
    }
}

// GET /lists
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListsType {
    /// An array of objects, each representing an email in an Automation workflow.
    #[serde(default)]
    pub lists: Vec<ListType>,
    /// Desc: The total number of items matching the query regardless of pagination.
    #[serde(default)]
    pub total_items: u64,
    /// Desc: A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

impl MailchimpCollection<ListType> for ListsType {
    /// Total Items
    fn get_total_items(&self) -> u64 {
        self.total_items
    }
    /// Data
    fn get_values(&self) -> Vec<ListType> {
        self.lists.clone()
    }
}

impl Default for ListsType {
    fn default() -> Self {
        ListsType {
            lists: Vec::new(),
            total_items: 0,
            _links: Vec::new(),
        }
    }
}

impl ListType {
    ///
    /// Get up to the previous 180 days of daily detailed aggregated activity
    /// stats for a list, not including Automation activity.
    ///
    /// Arguments:
    ///     fields: A comma-separated list of fields to return. Reference
    ///         parameters of sub-objects with dot notation.
    ///     exclude_fields: A comma-separated list of fields to exclude. Reference
    ///         parameters of sub-objects with dot notation.
    ///
    pub fn get_activity(
        &self,
        fields: Option<String>,
        exclude_fields: Option<String>,
    ) -> MalchimpIter<ListActivityBuilder> {
        let endpoint = self.get_base_endpoint() + "/activity";
        let mut filter_params = SimpleFilter::default();

        if let Some(f) = fields {
            filter_params.fields = Some(f);
        }

        if let Some(ex) = exclude_fields {
            filter_params.exclude_fields = Some(ex);
        }

        match self
            ._api
            .get::<CollectionListActivity>(&endpoint, filter_params.build_payload())
        {
            Ok(collection) => MalchimpIter {
                builder: ListActivityBuilder {},
                data: collection.activity,
                cur_filters: filter_params.clone(),
                cur_it: 0,
                total_items: collection.total_items,
                api: self._api.clone(),
                endpoint: endpoint.clone(),
            },
            Err(e) => {
                error!( target: "mailchimp",  "Get Activities: Response Error details: {:?}", e);
                MalchimpIter {
                    builder: ListActivityBuilder {},
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
    /// Get a list of the top email clients based on user-agent strings.
    ///
    /// Arguments:
    ///     fields: A comma-separated list of fields to return. Reference
    ///         parameters of sub-objects with dot notation.
    ///     exclude_fields: A comma-separated list of fields to exclude. Reference
    ///         parameters of sub-objects with dot notation.
    ///
    pub fn get_clients(
        &self,
        fields: Option<String>,
        exclude_fields: Option<String>,
    ) -> MalchimpIter<ListClientsBuilder> {
        // GET /lists/{list_id}/clients
        let endpoint = self.get_base_endpoint() + "/clients";
        let mut filter_params = SimpleFilter::default();

        if let Some(f) = fields {
            filter_params.fields = Some(f);
        }

        if let Some(ex) = exclude_fields {
            filter_params.exclude_fields = Some(ex);
        }

        match self
            ._api
            .get::<CollectionListClients>(&endpoint, filter_params.build_payload())
        {
            Ok(collection) => MalchimpIter {
                builder: ListClientsBuilder {},
                data: collection.clients,
                cur_filters: filter_params.clone(),
                cur_it: 0,
                total_items: collection.total_items,
                api: self._api.clone(),
                endpoint: endpoint.clone(),
            },
            Err(e) => {
                error!( target: "mailchimp",  "Get Clients: Response Error details: {:?}", e);
                MalchimpIter {
                    builder: ListClientsBuilder {},
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
    /// Get the locations (countries) that the list’s subscribers have been tagged to
    /// based on geocoding their IP address.
    ///
    /// Arguments:
    ///     fields: A comma-separated list of fields to return. Reference
    ///         parameters of sub-objects with dot notation.
    ///     exclude_fields: A comma-separated list of fields to exclude. Reference
    ///         parameters of sub-objects with dot notation.
    ///
    pub fn get_locations(
        &self,
        fields: Option<String>,
        exclude_fields: Option<String>,
    ) -> MalchimpIter<ListLocationsBuilder> {
        // GET /lists/{list_id}/locations
        let endpoint = self.get_base_endpoint() + "/locations";
        let mut filter_params = SimpleFilter::default();

        if let Some(f) = fields {
            filter_params.fields = Some(f);
        }

        if let Some(ex) = exclude_fields {
            filter_params.exclude_fields = Some(ex);
        }

        match self
            ._api
            .get::<CollectionListLocations>(&endpoint, filter_params.build_payload())
        {
            Ok(collection) => MalchimpIter {
                builder: ListLocationsBuilder {},
                data: collection.locations,
                cur_filters: filter_params.clone(),
                cur_it: 0,
                total_items: collection.total_items,
                api: self._api.clone(),
                endpoint: endpoint.clone(),
            },
            Err(e) => {
                error!( target: "mailchimp",  "Get Locations: Response Error details: {:?}", e);
                MalchimpIter {
                    builder: ListLocationsBuilder {},
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
    /// Get all abuse reports for a specific list.
    ///
    /// Arguments:
    ///     fields: A comma-separated list of fields to return. Reference
    ///         parameters of sub-objects with dot notation.
    ///     exclude_fields: A comma-separated list of fields to exclude. Reference
    ///         parameters of sub-objects with dot notation.
    ///     offset: The number of records from a collection to skip.
    ///         Iterating over large collections with this parameter can be slow.
    ///         Default value is 0.
    ///
    pub fn get_abuse_reports(
        &self,
        fields: Option<String>,
        exclude_fields: Option<String>,
        offset: Option<u64>
    ) -> MalchimpIter<ListAbuseReportBuilder> {
        // GET /lists/{list_id}/abuse-reports
        let endpoint = self.get_base_endpoint() + "/abuse-reports";
        let mut filter_params = SimpleFilter::default();

        if let Some(f) = fields {
            filter_params.fields = Some(f);
        }

        if let Some(ex) = exclude_fields {
            filter_params.exclude_fields = Some(ex);
        }
        if let Some(ofs) = offset {
            filter_params.offset = Some(ofs);
        }

        match self
            ._api
            .get::<CollectionListAbuseReport>(&endpoint, filter_params.build_payload())
        {
            Ok(collection) => MalchimpIter {
                builder: ListAbuseReportBuilder {},
                data: collection.abuse_reports,
                cur_filters: filter_params.clone(),
                cur_it: 0,
                total_items: collection.total_items,
                api: self._api.clone(),
                endpoint: endpoint.clone(),
            },
            Err(e) => {
                error!( target: "mailchimp",  "Get Locations: Response Error details: {:?}", e);
                MalchimpIter {
                    builder: ListAbuseReportBuilder {},
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
    /// Get details about a specific abuse report.
    ///
    /// Arguments:
    ///     fields: A comma-separated list of fields to return. Reference
    ///         parameters of sub-objects with dot notation.
    ///     exclude_fields: A comma-separated list of fields to exclude. Reference
    ///         parameters of sub-objects with dot notation.
    ///     offset: The number of records from a collection to skip.
    ///         Iterating over large collections with this parameter can be slow.
    ///         Default value is 0.
    ///
    pub fn get_specific_abuse_reports(
        &self,
        report_id: String
    ) ->  MailchimpResult<ListAbuseReportType> {
        // GET /lists/{list_id}/abuse-reports/{report_id}
        let endpoint = self.get_base_endpoint() + "/abuse-reports";
        let mut params = HashMap::new();
        params.insert("report_id".to_string(), report_id);

        self._api
            .get::<ListAbuseReportType>(&endpoint, params)
    }

    ///
    /// Get Endpoint
    ///
    fn get_base_endpoint(&self) -> String {
        String::from("lists/") + self.id.as_ref().unwrap().as_str()
    }
}
