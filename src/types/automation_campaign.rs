//! Implement Automations Model Types

use super::automation_subscriber::{
    AutomationSubscriberBuilder, AutomationSubscriberType, CollectionAutomationSubscriber,
};
use super::ecommerce::ECommerceReportType;
use super::empty::EmptyType;
use super::link::LinkType;
use super::list_segment_options::SegmentOptionsType;
use super::workflow_email::{WorkflowEmailType, WorkflowEmailsType};
use crate::api::{MailchimpApi, MailchimpApiUpdate};
use crate::internal::error_type::MailchimpErrorType;
use crate::internal::request::MailchimpResult;
use crate::iter::MailchimpCollection;
use crate::iter::{MalchimpIter, ResourceFilter, SimpleFilter};
use std::collections::HashMap;
use std::rc::Rc;

///
/// Automation Delay Type
///
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

///
/// Recipient Type
///
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
    /// Desc: A description of the segment used for the campaign. Formatted as a string marked up with HTML.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segment_text: Option<String>,
    /// Desc: Count of the recipients on the associated list. Formatted as an integer..
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient_count: Option<u64>,
    /// Desc: An object representing all segmentation options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segment_opts: Option<SegmentOptionsType>,
    /// Desc: The id of the store.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
}

impl Default for RecipientType {
    fn default() -> Self {
        RecipientType {
            list_id: None,
            list_is_active: None,
            segment_text: None,
            list_name: None,
            recipient_count: None,
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
            segment_text: None,
            list_is_active: None,
            list_name: None,
            recipient_count: None,
            segment_opts: None,
            store_id: Some(store_id.to_string()),
        }
    }
}

///
/// Salesforce CRM Tracking Type
///
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

///
/// Capsule CRM Tracking
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CapsuleCRMTrackingType {
    /// Update contact notes for a campaign based on a subscriber’s email addresses.
    #[serde(default)]
    pub notes: bool,
}

impl Default for CapsuleCRMTrackingType {
    fn default() -> Self {
        CapsuleCRMTrackingType { notes: false }
    }
}

///
/// Campaign Report Summary
///
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
    /// E-Commerce stats for a campaign.
    #[serde(default)]
    pub ecommerce: Option<ECommerceReportType>,
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
            ecommerce: None,
        }
    }
}

///
/// Automation Trigger
///
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
            workflow_type: Some(workflow_type.to_string()),
        }
    }
}

///
/// Automation Campaign Settings
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutomationCampaignSettingsType {
    /// The subject line for the campaign.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject_line: Option<String>,
    /// The preview text for the campaign.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview_text: Option<String>,
    /// The title of the Automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Desc: The ‘from’ name for the Automation (not an email address).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_name: Option<String>,
    /// Desc: The reply-to email address for the Automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    /// Desc: Whether Mailchimp authenticated the Automation. Defaults to true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authenticate: Option<bool>,
    /// Desc: Whether to use Mailchimp’s Conversations feature to manage out-of-office replies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_conversation: Option<bool>,
    /// Desc: The Automation’s custom ‘To’ name, typically the first name merge field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_name: Option<String>,
    /// Desc: If the campaign is listed in a folder, the id for that folder.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    /// Desc: Whether to automatically append Mailchimp’s default footer to the Automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_footer: Option<bool>,
    /// Desc: Whether to automatically inline the CSS included with the Automation content.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline_css: Option<bool>,
    /// Automatically tweet a link to the campaign archive page when the campaign is sent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_tweet: Option<bool>,
    /// An array of Facebook page ids to auto-post to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_fb_post: Option<Vec<u64>>,
    /// Allows Facebook comments on the campaign (also force-enables the Campaign Archive toolbar). Defaults to true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fb_comments: Option<bool>,
    /// Send this campaign using Timewarp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timewarp: Option<bool>,
    /// Allows Facebook comments on the campaign (also force-enables the Campaign Archive toolbar). Defaults to true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_id: Option<u64>,
    /// Whether the campaign uses the drag-and-drop editor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drag_and_drop: Option<bool>,
}

/// Campaigns Settings Type
pub type CampaignSettingsType = AutomationCampaignSettingsType;

impl Default for AutomationCampaignSettingsType {
    fn default() -> Self {
        AutomationCampaignSettingsType {
            subject_line: None,
            preview_text: None,
            title: None,
            from_name: None,
            reply_to: None,
            use_conversation: None,
            to_name: None,
            folder_id: None,
            authenticate: None,
            auto_footer: None,
            inline_css: None,
            auto_tweet: None,
            auto_fb_post: None,
            fb_comments: None,
            timewarp: None,
            template_id: None,
            drag_and_drop: None,
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
            subject_line: None,
            preview_text: None,
            title: None,
            from_name: Some(from_name.to_string()),
            folder_id: None,
            reply_to: Some(reply_to.to_string()),
            use_conversation: None,
            to_name: None,
            authenticate: None,
            auto_footer: None,
            inline_css: None,
            auto_tweet: None,
            auto_fb_post: None,
            fb_comments: None,
            timewarp: None,
            template_id: None,
            drag_and_drop: None,
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
            subject_line: None,
            preview_text: None,
            title: Some(title.to_string()),
            from_name: Some(from_name.to_string()),
            folder_id: None,
            reply_to: Some(reply_to.to_string()),
            use_conversation: None,
            to_name: None,
            authenticate: None,
            auto_footer: None,
            inline_css: None,
            auto_tweet: None,
            auto_fb_post: None,
            fb_comments: None,
            timewarp: None,
            template_id: None,
            drag_and_drop: None,
        }
    }
}

///
/// Automation Tracking Option
///
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

/// Campaign Tracking Options Type
pub type CampaignTrackingOptionsType = AutomationTrackingOptionsType;

///
/// Automation Workflows
///
/// Endpoint
///     GET /automations/{workflow_id}
///
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
    pub trigger_settings: Option<AutomationTriggerType>,

    /// Desc: A summary of opens, clicks, and unsubscribes for sent campaigns.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report_summary: Option<CampaignReportSummaryType>,

    /// Desc: A list of link types and descriptions for the API schema documents.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _links: Option<Vec<LinkType>>,

    /// Mailchimp APi
    #[serde(default, skip)]
    pub _api: Rc<MailchimpApi>,
}

impl MailchimpApiUpdate for AutomationWorkflowType {
    /**
     * Update API
     */
    fn set_api(&mut self, n_api: Rc<MailchimpApi>) {
        self._api = n_api.clone()
    }
}

///
/// Update Params For Workflow Email
///
#[derive(Serialize, Deserialize, Debug, Clone)]
struct UpdateParamsForWorkflowEmail {
    /// Settings for the campaign including the email subject, from name, and from email address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<AutomationCampaignSettingsType>,
    /// The delay settings for an Automation email.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<AutomationDelayType>,
}

impl AutomationWorkflowType {
    // ============== Actions ==============
    ///
    /// Detiene todos los emails para esta automatización
    ///
    /// En caso de ser satisfactoria la ejecución, devuelve None,
    /// en caso contrario devuelve el error, con su respectiva descripción
    ///
    pub fn pause_all_emails(&self) -> Option<MailchimpErrorType> {
        let mut b_endpoint = self.get_base_endpoint();
        b_endpoint.push_str("/actions/pause-all-emails");
        match self
            ._api
            .post::<EmptyType, HashMap<String, String>>(b_endpoint.as_str(), HashMap::new())
        {
            Ok(_) => None,
            Err(e) => Some(e),
        }
    }

    ///
    /// Inicia todos los emails para esta automatización
    ///
    /// En caso de ser satisfactoria la ejecución, devuelve None,
    /// en caso contrario devuelve el error, con su respectiva descripción
    ///
    pub fn start_all_emails(&self) -> Option<MailchimpErrorType> {
        let mut b_endpoint = self.get_base_endpoint();
        b_endpoint.push_str("/actions/start-all-emails");
        match self
            ._api
            .post::<EmptyType, HashMap<String, String>>(b_endpoint.as_str(), HashMap::new())
        {
            Ok(_) => None,
            Err(e) => Some(e),
        }
    }
    ///
    /// Actualiza la automatización y devuelve una instancia nueva
    ///
    /// Argumentos:
    ///     settings: Configuracion de la automatización a crear
    ///     delay: Ajustes de retraso para un correo electrónico de automatización.
    ///
    pub fn remote_update<'a>(
        &self,
        settings: Option<AutomationCampaignSettingsType>,
        delay: Option<AutomationDelayType>,
    ) -> MailchimpResult<Self> {
        let modifier = AutomationModifier {
            settings: settings,
            delay: delay,
            recipients: None,
            trigger_settings: None,
        };
        let response = self
            ._api
            .patch::<AutomationWorkflowType, AutomationModifier>("automations", modifier);
        match response {
            Ok(automation) => {
                let mut au = automation;
                au.set_api(self._api.clone());
                Ok(au)
            }
            Err(e) => Err(e),
        }
    }

    /// ============= EMAILS ============================
    ///
    /// Devuelve una lista de los emails automatizados
    ///
    pub fn get_workflow_emails(&self) -> MailchimpResult<Vec<WorkflowEmailType>> {
        let endpoint = self.get_base_endpoint() + "/emails";
        let response = self
            ._api
            .get::<WorkflowEmailsType>(endpoint.as_str(), HashMap::new());
        match response {
            Ok(value) => {
                let emails = value
                    .emails
                    .iter()
                    .map(move |data| {
                        let mut inner = endpoint.clone();
                        inner.push_str(data.id.as_ref().unwrap());
                        let mut inner_data = data.clone();
                        inner_data.set_api(self._api.clone());
                        inner_data.set_endpoint(&inner);
                        inner_data
                    })
                    .collect::<Vec<WorkflowEmailType>>();
                Ok(emails)
            }
            Err(e) => Err(e),
        }
    }

    ///
    /// Update settings for a Automation workflow email, and return the Automation workflow email Updated
    ///
    /// Argumentos:
    ///     campaign_id: The unique id for the campaign.
    ///     settings: Settings for the campaign including the email subject, from name, and from email address.
    ///     delay: The delay settings for an automation email.
    ///
    pub fn update_workflow_email<'a>(
        &self,
        workflow_email_id: &'a str,
        settings: &AutomationCampaignSettingsType,
        delay: &AutomationDelayType,
    ) -> MailchimpResult<WorkflowEmailType> {
        let mut endpoint = self.get_base_endpoint().clone();
        endpoint.push_str("/emails/");
        endpoint.push_str(workflow_email_id);

        let payload = UpdateParamsForWorkflowEmail {
            settings: Some(settings.clone()),
            delay: Some(delay.clone()),
        };

        let response = self
            ._api
            .patch::<WorkflowEmailType, UpdateParamsForWorkflowEmail>(endpoint.as_str(), payload);
        match response {
            Ok(workflow_email) => {
                let mut eml = workflow_email;
                eml.set_api(self._api.clone());
                eml.set_endpoint(&endpoint);
                Ok(eml)
            }
            Err(e) => Err(e),
        }
    }

    ///
    /// Removes an individual Automation workflow email. Emails from certain workflow types,
    /// including the Abandoned Cart Email (abandonedCart) and Product Retargeting Email
    /// (abandonedBrowse) Workflows, cannot be deleted.
    ///
    /// Argumentos:
    ///     workflow_email_id: The unique id for the Automation workflow email.
    ///
    pub fn delete_automation_workflow_email<'a>(
        &self,
        workflow_email_id: &'a str,
    ) -> Option<MailchimpErrorType> {
        let mut endpoint = self.get_base_endpoint().clone();
        endpoint.push_str("/emails/");
        endpoint.push_str(workflow_email_id);

        let response = self
            ._api
            .delete::<EmptyType>(endpoint.as_str(), HashMap::new());

        match response {
            Ok(_) => None,
            Err(e) => Some(e),
        }
    }

    ///
    /// Devuelve la informacion sobre un flujo de trabajos de automatizacion de emails
    ///
    /// Argumentos:
    ///     workflow_email_id: Identificador único de la automatización
    ///
    pub fn get_automation_workflow_email_info<'a>(
        &self,
        workflow_email_id: &'a str,
    ) -> MailchimpResult<WorkflowEmailType> {
        let mut endpoint = self.get_base_endpoint().clone();
        endpoint.push_str("/emails/");
        endpoint.push_str(workflow_email_id);

        let response = self
            ._api
            .get::<WorkflowEmailType>(endpoint.as_str(), HashMap::new());

        match response {
            Ok(workflow_email) => {
                let mut we = workflow_email;
                we.set_api(self._api.clone());
                we.set_endpoint(&endpoint);
                Ok(we)
            }
            Err(e) => Err(e),
        }
    }

    // ============== Subscribers Removed ==============

    ///
    /// View all subscribers removed from a workflow
    ///
    /// Return Iterator
    ///
    pub fn get_subscribers_removed(&self) -> MalchimpIter<AutomationSubscriberBuilder> {
        let endpoint = self.get_base_endpoint() + "/removed-subscribers";
        let filters = SimpleFilter::default();
        let response = self
            ._api
            .get::<CollectionAutomationSubscriber>(endpoint.as_str(), filters.build_payload());
        match response {
            Ok(collection) => MalchimpIter {
                builder: AutomationSubscriberBuilder {},
                data: collection.subscribers,
                cur_filters: filters.clone(),
                cur_it: 0,
                total_items: collection.total_items,
                api: self._api.clone(),
                endpoint: endpoint.clone(),
            },
            Err(_) => MalchimpIter {
                builder: AutomationSubscriberBuilder {},
                data: Vec::new(),
                cur_filters: filters.clone(),
                cur_it: 0,
                total_items: 0,
                api: self._api.clone(),
                endpoint: endpoint.clone(),
            },
        }
    }

    ///
    /// Remove a subscriber from a specific Automation workflow. You can remove a
    /// subscriber at any point in an Automation workflow, regardless of how many
    /// emails they’ve been sent from that workflow. Once they’re removed, they can never
    /// be added back to the same workflow.
    ///
    /// Arguments:
    ///     email_address: The list member’s email address.
    ///
    pub fn add_subscriber_to_workflow<'a>(
        &self,
        email_address: &'a str,
    ) -> MailchimpResult<AutomationSubscriberType> {
        // POST /automations/{workflow_id}/removed-subscribers
        let mut queue_endpoint = self.get_base_endpoint() + "/removed-subscribers";
        queue_endpoint.push_str("/queue");
        let mut payload = HashMap::new();
        payload.insert("email_address".to_string(), email_address.to_string());
        self._api
            .post::<AutomationSubscriberType, HashMap<String, String>>(&queue_endpoint, payload)
    }

    // ============== Private Functions ==============
    fn get_base_endpoint(&self) -> String {
        // /automations/{workflow_id}
        let mut b_endpoint = String::from("automations/");
        b_endpoint.push_str(self.id.as_ref().unwrap());
        b_endpoint
    }
}

///
/// Automations
///
/// Endpoint
///     GET /automations
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionAutomation {
    /// An array of objects, each representing an authorized application.
    #[serde(default)]
    pub automations: Vec<AutomationWorkflowType>,
    /// Desc: The total number of items matching the query regardless of pagination.
    #[serde(default)]
    pub total_items: u64,
    /// Desc: A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

impl MailchimpCollection<AutomationWorkflowType> for CollectionAutomation {
    /// Total Items
    fn get_total_items(&self) -> u64 {
        self.total_items
    }

    /// Data
    fn get_values(&self) -> Vec<AutomationWorkflowType> {
        self.automations.clone()
    }
}

impl Default for CollectionAutomation {
    fn default() -> Self {
        CollectionAutomation {
            automations: Vec::new(),
            total_items: 0,
            _links: Vec::new(),
        }
    }
}

///
/// Automation Modifier
///
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

///
/// Workflow Email
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SocialCardType {
    /// The url for the header image for the card.
    #[serde(default)]
    pub image_url: Option<String>,
    /// A short summary of the campaign to display.
    #[serde(default)]
    pub description: Option<String>,
    /// The title for the card. Typically the subject line of the campaign.
    #[serde(default)]
    pub title: Option<String>,
}

impl Default for SocialCardType {
    fn default() -> Self {
        SocialCardType {
            image_url: None,
            description: None,
            title: None,
        }
    }
}
