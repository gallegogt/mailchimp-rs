use std::collections::HashMap;

use super::automation_campaign::{
    AutomationCampaignSettingsType, AutomationDelayType, AutomationTrackingOptionsType,
    AutomationTriggerType, CampaignReportSummaryType, RecipientType, SocialCardType,
};
use super::empty::EmptyType;
use super::link::LinkType;
use crate::api::{MailchimpApi, MailchimpApiUpdate};
use crate::internal::error_type::MailchimpErrorType;

// ============ Workflow Email ==============
// GET /automations/{workflow_id}/emails/{workflow_email_id}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkflowEmailType {
    /// A string that uniquely identifies the Automation email.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The ID used in the Mailchimp web application. View this automation in your Mailchimp account at https://{dc}.admin.mailchimp.com/campaigns/show/?id={web_id}.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_id: Option<u64>,
    /// A string that uniquely identifies an Automation workflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
    /// The position of an Automation email in a workflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<u64>,
    /// The delay settings for an Automation email.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<AutomationDelayType>,
    /// The date and time the campaign was created in ISO 8601 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// The date and time the campaign was started in ISO 8601 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// The link to the campaign’s archive version in ISO 8601 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive_url: Option<String>,
    /// The total number of emails sent for this campaign.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emails_sent: Option<u64>,
    /// The date and time a campaign was sent in ISO 8601 format
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send_time: Option<String>,
    /// How the campaign’s content is put together (‘template’, ‘drag_and_drop’, ‘html’, ‘url’).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// Determines if the automation email needs its blocks refreshed by opening the web-based campaign editor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub needs_block_refresh: Option<bool>,
    /// Determines if the campaign contains the |BRAND:LOGO| merge tag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_logo_merge_tag: Option<bool>,
    /// List settings for the campaign.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipients: Option<RecipientType>,
    /// Settings for the campaign including the email subject, from name, and from email address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<AutomationCampaignSettingsType>,
    /// The tracking options for a campaign.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking: Option<AutomationTrackingOptionsType>,
    /// The preview for the campaign, rendered by social networks like Facebook and Twitter. Learn more.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub social_card: Option<SocialCardType>,
    /// Available triggers for Automation workflows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_settings: Option<AutomationTriggerType>,
    /// For sent campaigns, a summary of opens, clicks, and unsubscribes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report_summary: Option<CampaignReportSummaryType>,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _links: Option<Vec<LinkType>>,

    /// ============ CAMPOS NO SERIALIZABLES ========
    /// Mailchimp APi
    #[serde(skip)]
    _api: MailchimpApi,
    // Endpoint del Recurso
    #[serde(skip)]
    _endpoint: String,
}

impl MailchimpApiUpdate for WorkflowEmailType {
    /**
     * Update API
     */
    fn set_api(&mut self, n_api: &MailchimpApi) {
        self._api = n_api.clone()
    }
}

impl WorkflowEmailType {
    // ============== Actions ==============
    ///
    /// Detiene un email automatizado
    ///
    /// En caso de ser satisfactoria la ejecución, devuelve None,
    /// en caso contrario devuelve el error, con su respectiva descripción
    ///
    pub fn pause_all_emails(&self) -> Option<MailchimpErrorType> {
        let mut b_endpoint = self._endpoint.clone();
        b_endpoint.push_str("/actions/pause");
        match self
            ._api
            .post::<EmptyType, HashMap<String, String>>(b_endpoint.as_str(), HashMap::new())
        {
            Ok(_) => None,
            Err(e) => Some(e),
        }
    }

    ///
    /// Inicia un email automatizado
    ///
    /// En caso de ser satisfactoria la ejecución, devuelve None,
    /// en caso contrario devuelve el error, con su respectiva descripción
    ///
    pub fn start_all_emails(&self) -> Option<MailchimpErrorType> {
        let mut b_endpoint = self._endpoint.clone();
        b_endpoint.push_str("/actions/start");
        match self
            ._api
            .post::<EmptyType, HashMap<String, String>>(b_endpoint.as_str(), HashMap::new())
        {
            Ok(_) => None,
            Err(e) => Some(e),
        }
    }

    ///
    /// Obtiene el Endpoint de este recurso
    ///
    ///
    pub fn get_endpoint(&self) -> &String {
        &self._endpoint
    }

    ///
    /// Actualiza el valor del Endpoint para este recurso
    ///
    /// Argumentos:
    ///     n_endpoint: Nuevo Endpoint
    ///
    pub fn set_endpoint<'a>(&mut self, n_endpoint: &'a str) {
        self._endpoint = n_endpoint.to_string();
    }
}

// GET /automations/{workflow_id}/emails
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkflowEmailsType {
    /// An array of objects, each representing an email in an Automation workflow.
    #[serde(default)]
    pub emails: Vec<WorkflowEmailType>,
    /// Desc: The total number of items matching the query regardless of pagination.
    #[serde(default)]
    pub total_items: u32,
    /// Desc: A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}
