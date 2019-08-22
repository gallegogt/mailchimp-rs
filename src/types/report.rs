use super::campaign::CampaignDeliveryStatusType;
use super::ecommerce::ECommerceReportType;
use super::industry_stats::IndustryStatsType;
use super::link::LinkType;
use crate::api::MailchimpApi;
use crate::iter::{BuildIter, MailchimpCollection, ResourceFilter};
use std::collections::HashMap;
use std::rc::Rc;

///
/// An object describing the bounce summary for the campaign.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HardBouncesType {
    /// The total number of hard bounced email addresses.
    #[serde(default)]
    pub hard_bounces: u64,
    /// The total number of soft bounced email addresses.
    #[serde(default)]
    pub soft_bounces: u64,
    /// The total number of addresses that were syntax-related bounces.
    #[serde(default)]
    pub syntax_errors: u64,
}

impl Default for HardBouncesType {
    fn default() -> Self {
        Self {
            hard_bounces: 0,
            soft_bounces: 0,
            syntax_errors: 0,
        }
    }
}

///
/// An object describing the forwards and forward activity for the campaign.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForwardsType {
    /// How many times the campaign has been forwarded.
    #[serde(default)]
    pub forwards_count: u64,
    /// How many times the forwarded campaign has been opened.
    #[serde(default)]
    pub forwards_opens: u64,
}

impl Default for ForwardsType {
    fn default() -> Self {
        Self {
            forwards_count: 0,
            forwards_opens: 0,
        }
    }
}

///
/// An object describing the open activity for the campaign.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpensType {
    /// The total number of opens for a campaign.
    #[serde(default)]
    pub opens_total: u64,
    /// The total number of unique opens.
    #[serde(default)]
    pub unique_opens: u64,
    /// The number of unique opens divided by the total number of successful deliveries.
    #[serde(default)]
    pub open_rate: f32,
    /// The date and time of the last recorded open in ISO 8601 format.
    #[serde(default)]
    pub last_open: String,
}

impl Default for OpensType {
    fn default() -> Self {
        Self {
            opens_total: 0,
            unique_opens: 0,
            open_rate: 0.0,
            last_open: String::new(),
        }
    }
}

///
/// An object describing the click activity for the campaign.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClicksType {
    /// The total number of clicks for the campaign.
    #[serde(default)]
    pub clicks_total: u64,
    /// The total number of unique clicks for links across a campaign.
    #[serde(default)]
    pub unique_clicks: u64,
    /// The total number of subscribers who clicked on a campaign.
    #[serde(default)]
    pub unique_subscriber_clicks: u64,
    /// The number of unique clicks divided by the total number of successful deliveries.
    #[serde(default)]
    pub click_rate: f32,
    /// The date and time of the last recorded click for the campaign in ISO 8601 format.
    #[serde(default)]
    pub last_click: String,
}

impl Default for ClicksType {
    fn default() -> Self {
        Self {
            clicks_total: 0,
            unique_clicks: 0,
            unique_subscriber_clicks: 0,
            click_rate: 0.0,
            last_click: String::new(),
        }
    }
}

///
/// An object describing campaign engagement on Facebook.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FacebookLikesType {
    /// The number of recipients who liked the campaign on Facebook.
    #[serde(default)]
    pub recipient_likes: u64,
    /// The number of unique likes.
    #[serde(default)]
    pub unique_likes: u64,
    /// The number of Facebook likes for the campaign.
    #[serde(default)]
    pub facebook_likes: u64,
}

impl Default for FacebookLikesType {
    fn default() -> Self {
        Self {
            recipient_likes: 0,
            unique_likes: 0,
            facebook_likes: 0,
        }
    }
}

///
/// The average campaign statistics for your list. This won’t be present if we
/// haven’t calculated it yet for this list.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListStatsType {
    /// The average number of subscriptions per month for the list.
    #[serde(default)]
    pub sub_rate: f32,
    /// The average number of unsubscriptions per month for the list.
    #[serde(default)]
    pub unsub_rate: f32,
    /// The average open rate (a percentage represented as a number between 0 and 100)
    /// per campaign for the list.
    #[serde(default)]
    pub open_rate: f32,
    /// The average click rate (a percentage represented as a number between 0 and 100)
    ///  per campaign for the list.
    #[serde(default)]
    pub click_rate: f32,
}

impl Default for ListStatsType {
    fn default() -> Self {
        Self {
            sub_rate: 0.0,
            unsub_rate: 0.0,
            open_rate: 0.0,
            click_rate: 0.0,
        }
    }
}

///
/// Stats for Campaign A / B.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AbStatsType {
    /// Bounces for Campaign A / B.
    #[serde(default)]
    pub bounces: u64,
    /// Abuse reports for Campaign A / B.
    #[serde(default)]
    pub abuse_reports: u64,
    /// Unsubscribes for Campaign A / B.
    #[serde(default)]
    pub unsubs: u64,
    /// Recipient Clicks for Campaign A / B.
    #[serde(default)]
    pub recipient_clicks: u64,
    /// Forwards for Campaign A / B.
    #[serde(default)]
    pub forwards: u64,
    /// Opens from forwards for Campaign A / B.
    #[serde(default)]
    pub forwards_opens: u64,
    /// Opens for Campaign A / B.
    #[serde(default)]
    pub opens: u64,
    /// The last open for Campaign A / B.
    #[serde(default)]
    pub last_open: String,
    /// Unique opens for Campaign A / B.
    #[serde(default)]
    pub unique_opens: u64,
}

impl Default for AbStatsType {
    fn default() -> Self {
        Self {
            bounces: 0,
            abuse_reports: 0,
            unsubs: 0,
            recipient_clicks: 0,
            forwards: 0,
            forwards_opens: 0,
            opens: 0,
            last_open: String::new(),
            unique_opens: 0,
        }
    }
}

///
/// General stats about different groups of an A/B Split campaign.
/// Does not return information about Mailchimp Pro’s Multivariate Campaigns.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AbSplitStatsType {
    /// Stats for Campaign A.
    #[serde(default)]
    pub a: AbStatsType,
    /// Stats for Campaign B.
    #[serde(default)]
    pub b: AbStatsType,
}

impl Default for AbSplitStatsType {
    fn default() -> Self {
        Self {
            a: AbStatsType::default(),
            b: AbStatsType::default(),
        }
    }
}

///
/// An hourly breakdown of sends, opens, and clicks if a campaign is
/// sent using timewarp.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimewarpStatsType {
    /// For campaigns sent with timewarp, the time zone group the member is apart of.
    #[serde(default)]
    pub gmt_offset: u64,
    /// The number of opens.
    #[serde(default)]
    pub opens: u64,
    /// The date and time of the last open in ISO 8601 format.
    #[serde(default)]
    pub last_open: String,
    /// The number of unique opens.
    #[serde(default)]
    pub unique_opens: u64,
    /// The number of clicks.
    #[serde(default)]
    pub clicks: u64,
    /// The date and time of the last click in ISO 8601 format.
    #[serde(default)]
    pub last_click: String,
    /// The number of unique clicks.
    #[serde(default)]
    pub unique_clicks: u64,
    /// The number of bounces.
    #[serde(default)]
    pub bounces: u64,
}

impl Default for TimewarpStatsType {
    fn default() -> Self {
        Self {
            gmt_offset: 0,
            opens: 0,
            last_open: String::new(),
            unique_opens: 0,
            clicks: 0,
            last_click: String::new(),
            unique_clicks: 0,
            bounces: 0,
        }
    }
}

///
/// An hourly breakdown of the performance of the campaign over the first 24 hours.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeSerieType {
    /// The date and time for the series in ISO 8601 format.
    #[serde(default)]
    pub timestamp: String,
    /// The number of emails sent in the timeseries.
    #[serde(default)]
    pub emails_sent: u64,
    /// The number of unique opens in the timeseries.
    #[serde(default)]
    pub unique_opens: u64,
    /// The number of clicks in the timeseries.
    #[serde(default)]
    pub recipients_clicks: u64,
}

impl Default for TimeSerieType {
    fn default() -> Self {
        Self {
            timestamp: String::new(),
            emails_sent: 0,
            unique_opens: 0,
            recipients_clicks: 0,
        }
    }
}

///
/// The url and password for the VIP report.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShareReportType {
    /// The URL for the VIP report.
    #[serde(default)]
    pub share_url: String,
    /// If password protected, the password for the VIP report.
    #[serde(default)]
    pub share_password: String,
}

impl Default for ShareReportType {
    fn default() -> Self {
        Self {
            share_url: String::new(),
            share_password: String::new(),
        }
    }
}

// ============ Reports ==============
///
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportType {
    /// A string that uniquely identifies this campaign.
    #[serde(default)]
    pub id: String,
    /// The title of the campaign.
    #[serde(default)]
    pub campaign_title: String,
    /// The type of campaign (regular, plain-text, ab_split, rss, automation, variate, or auto).
    #[serde(default, rename = "type")]
    pub report_type: String,
    /// The unique list id.
    #[serde(default)]
    pub list_id: String,
    /// The status of the list used, namely if it’s deleted or disabled.
    #[serde(default)]
    pub list_is_active: bool,
    /// The name of the list.
    #[serde(default)]
    pub list_name: String,
    /// The subject line for the campaign.
    #[serde(default)]
    pub subject_line: String,
    /// The preview text for the campaign.
    #[serde(default)]
    pub preview_text: String,
    /// The total number of emails sent for this campaign.
    #[serde(default)]
    pub emails_sent: u64,
    /// The number of abuse reports generated for this campaign.
    #[serde(default)]
    pub abuse_reports: u64,
    /// The total number of unsubscribed members for this campaign.
    #[serde(default)]
    pub unsubscribed: u64,
    /// The date and time a campaign was sent in ISO 8601 format.
    #[serde(default)]
    pub send_time: String,
    /// For RSS campaigns, the date and time of the last send in ISO 8601 format.
    #[serde(default)]
    pub rss_last_send: String,
    /// An object describing the bounce summary for the campaign.
    #[serde(default)]
    pub bounces: HardBouncesType,
    /// An object describing the forwards and forward activity for the campaign.
    #[serde(default)]
    pub forwards: ForwardsType,
    /// An object describing the open activity for the campaign.
    #[serde(default)]
    pub opens: OpensType,
    /// An object describing the click activity for the campaign.
    #[serde(default)]
    pub clicks: ClicksType,
    /// An object describing campaign engagement on Facebook.
    #[serde(default)]
    pub facebook_likes: FacebookLikesType,
    /// The average campaign statistics for your industry.
    #[serde(default)]
    pub industry_stats: IndustryStatsType,
    /// The average campaign statistics for your list. This won’t be present
    /// if we haven’t calculated it yet for this list.
    #[serde(default)]
    pub list_stats: ListStatsType,
    /// General stats about different groups of an A/B Split campaign. Does
    /// not return information about Mailchimp Pro’s Multivariate Campaigns.
    #[serde(default)]
    pub ab_split: AbSplitStatsType,
    /// An hourly breakdown of sends, opens, and clicks if a campaign is sent using timewarp.
    #[serde(default)]
    pub timewarp: Vec<TimewarpStatsType>,
    /// An hourly breakdown of the performance of the campaign over the first 24 hours.
    #[serde(default)]
    pub timeseries: Vec<TimeSerieType>,
    /// The url and password for the VIP report.
    #[serde(default)]
    pub share_report: ShareReportType,
    /// E-Commerce stats for a campaign.
    #[serde(default)]
    pub ecommerce: ECommerceReportType,
    /// Updates on campaigns in the process of sending.
    #[serde(default)]
    pub delivery_status: CampaignDeliveryStatusType,

    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

///
/// Collection Reports
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionReports {
    /// An array of objects, each representing a report resource.
    #[serde(default)]
    pub reports: Vec<ReportType>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(default)]
    pub total_items: u64,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

impl MailchimpCollection<ReportType> for CollectionReports {
    /// Total Items
    fn get_total_items(&self) -> u64 {
        self.total_items
    }

    /// Data
    fn get_values(&self) -> Vec<ReportType> {
        self.reports.clone()
    }
}

impl Default for CollectionReports {
    fn default() -> Self {
        CollectionReports {
            reports: Vec::new(),
            total_items: 0,
            _links: Vec::new(),
        }
    }
}

///
/// Reports Filter
///
#[derive(Debug, Clone)]
pub struct ReportsFilter {
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
    /// The campaign type
    /// Possible Values:
    ///     regular - plaintext - absplit - rss - variate
    pub campaign_type: Option<String>,
    /// Restrict the response to campaigns sent before the set time.
    /// We recommend ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    pub before_send_time: Option<String>,
    /// Restrict the response to campaigns sent after the set time.
    /// We recommend ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    pub since_send_time: Option<String>,
}

impl Default for ReportsFilter {
    fn default() -> Self {
        Self {
            fields: None,
            exclude_fields: None,
            count: Some(50),
            offset: Some(0),
            campaign_type: None,
            before_send_time: None,
            since_send_time: None,
        }
    }
}

impl ResourceFilter for ReportsFilter {
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
        if self.campaign_type.is_some() {
            payload.insert(
                "campaign_type".to_string(),
                format!("{:}", self.campaign_type.as_ref().unwrap().clone()),
            );
        }
        if self.before_send_time.is_some() {
            payload.insert(
                "before_send_time".to_string(),
                format!("{:}", self.before_send_time.as_ref().unwrap().clone()),
            );
        }
        if self.since_send_time.is_some() {
            payload.insert(
                "since_send_time".to_string(),
                format!("{:}", self.since_send_time.as_ref().unwrap().clone()),
            );
        }
        payload
    }
}

///
/// Reports Builder
///
#[derive(Debug)]
pub struct ReportsBuilder {}

impl BuildIter for ReportsBuilder {
    type Item = ReportType;
    type FilterItem = ReportsFilter;
    type Collection = CollectionReports;

    ///
    /// Crea un recurso a partir del dato pasado por parámetro
    ///
    fn update_item(&self, data: &Self::Item, _: Rc<MailchimpApi>) -> Self::Item {
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
