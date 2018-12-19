use crate::internal::types::LinkType;

// ============ Segment Conditions ==============
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SegmentConditionsType {
    /// The type of segment, for example: date, language, Mandrill, static, and more.
    #[serde(default)]
    pub condition_type: String,
}

impl Default for SegmentConditionsType {
    fn default() -> Self {
        SegmentConditionsType {
            condition_type: "".to_string(),
        }
    }
}

// ============ Segment Options ==============
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SegmentOptionsType {
    /// The id for an existing saved segment.
    #[serde(default)]
    pub saved_segment_id: u64,
    /// Desc: Segment match type.
    #[serde(default, rename = "match")]
    pub match_filter: String,
}

impl Default for SegmentOptionsType {
    fn default() -> Self {
        SegmentOptionsType {
            saved_segment_id: 0,
            match_filter: "".to_string(),
        }
    }
}

// ============ Recipient ==============
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecipientType {
    /// The unique list id.
    #[serde(default)]
    pub list_id: String,
    /// Desc: The status of the list used, namely if it’s deleted or disabled.
    #[serde(default)]
    pub list_is_active: bool,
    /// Desc: List Name.
    #[serde(default)]
    pub list_name: String,
    /// Desc: An object representing all segmentation options.
    #[serde(default)]
    pub segment_opts: Vec<SegmentOptionsType>,
    /// Desc: The id of the store.
    #[serde(default)]
    pub store_id: String,
}

impl Default for RecipientType {
    fn default() -> Self {
        RecipientType {
            list_id: "".to_string(),
            list_is_active: false,
            list_name: "".to_string(),
            segment_opts: Vec::new(),
            store_id: "".to_string(),
        }
    }
}

// ============ Salesforce CRM Tracking ==============
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SalesforceCRMTrackingType {
    /// Create a campaign in a connected Salesforce account.
    #[serde(default)]
    pub campaign: bool,
    /// Update contact notes for a campaign based on a subscriber’s email address.
    #[serde(default)]
    pub notes: bool,
}

impl Default for SalesforceCRMTrackingType {
    fn default() -> Self {
        SalesforceCRMTrackingType {
            campaign: false,
            notes: false,
        }
    }
}

// ============ Capsule CRM Tracking ==============
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CapsuleCRMTrackingType {
    /// Update contact notes for a campaign based on a subscriber’s email addresses.
    #[serde(default)]
    pub notes: bool,
}

impl Default for CapsuleCRMTrackingType {
    fn default() -> Self {
        CapsuleCRMTrackingType {
            notes: false,
        }
    }
}

// ============ Campaign Report Summary ==============
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CampaignReportSummaryType {
    /// The total number of opens for a campaign.
    #[serde(default)]
    pub opens: u64,
    /// The number of unique opens.
    #[serde(default)]
    pub unique_opens: u64,
    /// The number of unique opens divided by the total number of successful deliveries.
    #[serde(default)]
    pub open_rate: f32,
    /// The total number of clicks for an campaign.
    #[serde(default)]
    pub clicks: u64,
    /// The number of unique clicks.
    #[serde(default)]
    pub subscriber_clicks: u64,
    /// The number of unique clicks, divided by the total number of successful deliveries.
    #[serde(default)]
    pub click_rate: f32,
}

impl Default for CampaignReportSummaryType {
    fn default() -> Self {
        CampaignReportSummaryType {
            opens: 0,
            unique_opens: 0,
            open_rate: 0.0,
            clicks: 0,
            subscriber_clicks: 0,
            click_rate: 0.0,
        }
    }
}

// ============ Automation Trigger ==============
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutomationTriggerType {
    /// The type of Automation workflow.
    #[serde(default)]
    pub workflow_type: String,
}

impl Default for AutomationTriggerType {
    fn default() -> Self {
        AutomationTriggerType {
            workflow_type: "".to_string(),
        }
    }
}

// ============ Automation Campaign Settings ==============
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutomationCampaignSettingsType {
    /// The title of the Automation.
    #[serde(default)]
    pub title: String,
    /// Desc: The ‘from’ name for the Automation (not an email address).
    #[serde(default)]
    pub from_name: String,
    /// Desc: The reply-to email address for the Automation.
    #[serde(default)]
    pub reply_to: String,
    /// Desc: Whether to use Mailchimp’s Conversations feature to manage out-of-office replies.
    #[serde(default)]
    pub use_conversation: bool,
    /// Desc: The Automation’s custom ‘To’ name, typically the first name merge field.
    #[serde(default)]
    pub to_name: String,
    /// Desc: Whether Mailchimp authenticated the Automation. Defaults to true.
    #[serde(default)]
    pub authenticate: bool,
    /// Desc: Whether to automatically append Mailchimp’s default footer to the Automation.
    #[serde(default)]
    pub auto_footer: bool,
    /// Desc: Whether to automatically inline the CSS included with the Automation content.
    #[serde(default)]
    pub inline_css: bool,
}

impl Default for AutomationCampaignSettingsType {
    fn default() -> Self {
        AutomationCampaignSettingsType {
            title: "".to_string(),
            from_name: "".to_string(),
            reply_to: "".to_string(),
            use_conversation: false,
            to_name: "".to_string(),
            authenticate: true,
            auto_footer: false,
            inline_css: false,
        }
    }
}

// ============ Automation Tracking Options ==============
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutomationTrackingOptionsType {
    /// Whether to track opens. Defaults to true.
    #[serde(default)]
    pub opens: bool,
    /// Desc: Whether to track clicks in the HTML version of the Automation. Defaults to true.
    #[serde(default)]
    pub html_clicks: bool,
    /// Desc: Whether to track clicks in the plain-text version of the Automation. Defaults to true.
    #[serde(default)]
    pub text_clicks: bool,
    /// Desc: Whether to enable eCommerce360 tracking.
    #[serde(default)]
    pub goal_tracking: bool,
    /// Desc: The Automation’s custom ‘To’ name, typically the first name merge field.
    #[serde(default)]
    pub ecomm360: bool,
    /// Desc: The custom slug for Google Analytics tracking (max of 50 bytes).
    #[serde(default)]
    pub google_analytics: String,
    /// Desc: The custom slug for ClickTale tracking (max of 50 bytes).
    #[serde(default)]
    pub clicktale: bool,
    /// Desc: Salesforce tracking options for an Automation. Must be using Mailchimp’s
    /// built-in Salesforce integration.
    #[serde(default)]
    pub salesforce: SalesforceCRMTrackingType,
    /// Desc: Capsule tracking options for an Automation. Must be using Mailchimp’s
    /// built-in Capsule integration.
    #[serde(default)]
    pub capsule: CapsuleCRMTrackingType,
    /// Desc: Available triggers for Automation workflows.
    #[serde(default)]
    pub trigger_settings: AutomationTriggerType,
}

impl Default for AutomationTrackingOptionsType {
    fn default() -> Self {
        AutomationTrackingOptionsType {
            opens: true,
            html_clicks: true,
            text_clicks: true,
            goal_tracking: false,
            ecomm360: false,
            google_analytics: "".to_string(),
            clicktale: false,
            salesforce: SalesforceCRMTrackingType::default(),
            capsule: CapsuleCRMTrackingType::default(),
            trigger_settings: AutomationTriggerType::default(),
        }
    }
}

// ============ Automation Workflows ==============
// GET /automations/{workflow_id}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutomationWorkflowType {
    /// A string that identifies the Automation.
    #[serde(default)]
    pub id: String,

    /// Desc: The date and time the Automation was created in ISO 8601 format.
    #[serde(default)]
    pub create_time: String,

    /// Desc: The date and time the Automation was started in ISO 8601 format.
    #[serde(default)]
    pub start_time: String,

    /// Desc: The current status of the Automation.
    #[serde(default)]
    pub status: String,

    /// Desc: The total number of emails sent for the Automation.
    #[serde(default)]
    pub emails_sent: u64,

    /// Desc: List settings for the Automation.
    #[serde(default)]
    pub recipients: RecipientType,
    /// Desc: List settings for the Automation.
    #[serde(default)]
    pub settings: AutomationCampaignSettingsType,
    /// Desc: A summary of opens, clicks, and unsubscribes for sent campaigns.
    #[serde(default)]
    pub report_summary: CampaignReportSummaryType,
    /// Desc: A summary of opens, clicks, and unsubscribes for sent campaigns.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

// ============ Authorized Apps ==============
// GET /automations
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutomationsType {
    /// An array of objects, each representing an authorized application.
    #[serde(default)]
    pub automations: Vec<AutomationWorkflowType>,
    /// Desc: The total number of items matching the query regardless of pagination.
    #[serde(default)]
    pub total_items: u32,
    /// Desc: A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}
