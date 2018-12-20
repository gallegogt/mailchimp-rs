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

// ============ Automation Delay ==============
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutomationDelayType {
    /// The delay amount for an automation email.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    /// The type of delay for an automation email.
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub delay_type: Option<String>,
    /// Whether the delay settings describe before or after the delay action of an automation email.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    /// The action that triggers the delay of an automation emails.

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

impl Default for AutomationDelayType {
    fn default() -> Self {
        AutomationDelayType {
            amount: Some(0),
            delay_type: Some("".to_string()),
            direction: Some("".to_string()),
            action: Some("".to_string()),
        }
    }
}

// ============ Recipient ==============
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecipientType {
    /// The unique list id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// Desc: The status of the list used, namely if it’s deleted or disabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_is_active: Option<bool>,
    /// Desc: List Name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_name: Option<String>,
    /// Desc: An object representing all segmentation options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segment_opts: Option<Vec<SegmentOptionsType>>,
    /// Desc: The id of the store.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
}

impl Default for RecipientType {
    fn default() -> Self {
        RecipientType {
            list_id: None,
            list_is_active: None,
            list_name: None,
            segment_opts: None,
            store_id: None,
        }
    }
}

impl RecipientType {
    ///
    /// Función de ayuda para el proceso creación de una automatización
    ///
    /// Argumentos:
    ///     list_id: Id de la lista
    ///     store_id: Id de la store
    ///
    pub fn create<'a>(list_id: &'a str, store_id: &'a str) -> Self {
        RecipientType {
            list_id: Some(list_id.to_string()),
            list_is_active: None,
            list_name: None,
            segment_opts: None,
            store_id: Some(store_id.to_string()),
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow_type: Option<String>,
}

impl Default for AutomationTriggerType {
    fn default() -> Self {
        AutomationTriggerType {
            workflow_type: None,
        }
    }
}


impl AutomationTriggerType {
    ///
    /// Shortcut para el proceso de creación de una automatización
    ///
    /// Argumentos:
    ///     workflow_type: The type of Automation workflow. Currently only supports ‘abandonedCart’.
    ///
    pub fn create<'a>(workflow_type: &'a str) -> Self {
        AutomationTriggerType {
            workflow_type: Some(workflow_type.to_string())
        }
    }
}

// ============ Automation Campaign Settings ==============
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutomationCampaignSettingsType {
    /// The title of the Automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Desc: The ‘from’ name for the Automation (not an email address).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_name: Option<String>,
    /// Desc: The reply-to email address for the Automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    /// Desc: Whether to use Mailchimp’s Conversations feature to manage out-of-office replies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_conversation: Option<bool>,
    /// Desc: The Automation’s custom ‘To’ name, typically the first name merge field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_name: Option<String>,
    /// Desc: Whether Mailchimp authenticated the Automation. Defaults to true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authenticate: Option<bool>,
    /// Desc: Whether to automatically append Mailchimp’s default footer to the Automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_footer: Option<bool>,
    /// Desc: Whether to automatically inline the CSS included with the Automation content.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline_css: Option<bool>,
}

impl Default for AutomationCampaignSettingsType {
    fn default() -> Self {
        AutomationCampaignSettingsType {
            title: None,
            from_name: None,
            reply_to: None,
            use_conversation: None,
            to_name: None,
            authenticate: None,
            auto_footer: None,
            inline_css: None,
        }
    }
}

impl AutomationCampaignSettingsType {
    ///
    /// Shortcut para el proceso de creación de una automatización
    ///
    /// Argumentos:
    ///     from_name: El ‘from’ para la automatización.
    ///     reply_to: La dirección de corrreo para la automatización, reply-to.
    ///
    pub fn create<'a>(from_name: &'a str, reply_to: &'a str) -> Self {
        AutomationCampaignSettingsType {
            title: None,
            from_name: Some(from_name.to_string()),
            reply_to: Some(reply_to.to_string()),
            use_conversation: None,
            to_name: None,
            authenticate: None,
            auto_footer: None,
            inline_css: None,
        }
    }
    ///
    /// Shortcut para el proceso de creación de una automatización
    ///
    /// Argumentos:
    ///     title: Titulo de la automatizacion
    ///     from_name: El ‘from’ para la automatización.
    ///     reply_to: La dirección de corrreo para la automatización, reply-to.
    ///
    pub fn update<'a>(title: &'a str, from_name: &'a str, reply_to: &'a str) -> Self {
        AutomationCampaignSettingsType {
            title: Some(title.to_string()),
            from_name: Some(from_name.to_string()),
            reply_to: Some(reply_to.to_string()),
            use_conversation: None,
            to_name: None,
            authenticate: None,
            auto_footer: None,
            inline_css: None,
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
    pub clicktale: String,
    /// Desc: Salesforce tracking options for an Automation. Must be using Mailchimp’s
    /// built-in Salesforce integration.
    #[serde(default)]
    pub salesforce: SalesforceCRMTrackingType,
    /// Desc: Capsule tracking options for an Automation. Must be using Mailchimp’s
    /// built-in Capsule integration.
    #[serde(default)]
    pub capsule: CapsuleCRMTrackingType,
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
            clicktale: "".to_string(),
            salesforce: SalesforceCRMTrackingType::default(),
            capsule: CapsuleCRMTrackingType::default(),
        }
    }
}

// ============ Automation Workflows ==============
// GET /automations/{workflow_id}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutomationWorkflowType {
    /// A string that identifies the Automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Desc: The date and time the Automation was created in ISO 8601 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,

    /// Desc: The date and time the Automation was started in ISO 8601 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,

    /// Desc: The current status of the Automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Desc: The total number of emails sent for the Automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emails_sent: Option<u64>,

    /// Desc: List settings for the Automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipients: Option<RecipientType>,

    /// Desc: The settings for the Automation workflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<AutomationCampaignSettingsType>,

    /// Desc: The tracking options for the Automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking: Option<AutomationTrackingOptionsType>,

    /// Desc: Available triggers for Automation workflows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_settings:Option<AutomationTriggerType>,

    /// Desc: A summary of opens, clicks, and unsubscribes for sent campaigns.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report_summary: Option<CampaignReportSummaryType>,

    /// Desc: A list of link types and descriptions for the API schema documents.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _links: Option<Vec<LinkType>>,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutomationModifier {
    /// The settings for the Automation workflow.
    #[serde(default)]
    pub settings: Option<AutomationCampaignSettingsType>,
    /// Desc: The total number of items matching the query regardless of pagination.
    #[serde(default)]
    pub delay: Option<AutomationDelayType>,
    /// Desc: A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub recipients: Option<RecipientType>,
    /// Desc: A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub trigger_settings: Option<AutomationTriggerType>,
}