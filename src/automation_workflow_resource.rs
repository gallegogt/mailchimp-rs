use crate::api::MailchimpApi;
use crate::internal::types::{
    AutomationCampaignSettingsType, AutomationTrackingOptionsType, AutomationWorkflowType,
    CampaignReportSummaryType, RecipientType,AutomationTriggerType
};

///
/// AutomationWorkflowResource
///
#[derive(Debug, Clone)]
pub struct AutomationWorkflowResource {
    api: MailchimpApi,
    automation: AutomationWorkflowType,
}

impl AutomationWorkflowResource {
    ///
    /// Argumentos:
    ///     api: MailchimpApi
    ///     automation: AutomationWorkflowType
    ///
    pub fn new(api: MailchimpApi, automation: AutomationWorkflowType) -> Self {
        AutomationWorkflowResource {
            api: api,
            automation: automation,
        }
    }
    ///
    /// Devuelve el resumen de la campaña
    ///
    pub fn get_report_summary(&self) -> &CampaignReportSummaryType {
        &self.automation.report_summary
    }
    ///
    /// Devuelve la configuracion de la campaña
    ///
    pub fn get_settings(&self) -> &AutomationCampaignSettingsType {
        &self.automation.settings
    }

    ///
    /// Devuelve el estado de la automatizacion
    ///
    pub fn get_status(&self) -> &String {
        &self.automation.status
    }

    ///
    /// Devuelve las opciones de seguimiento para la automatización.
    ///
    pub fn get_tracking(&self) -> &AutomationTrackingOptionsType {
        &self.automation.tracking
    }

    ///
    /// Devuelve el identificador de la automatizacion
    ///
    pub fn get_id(&self) -> &String {
        &self.automation.id
    }
    ///
    /// Devuelve la fecha y el tiempo en que fue creada la automatizacion
    /// formato ISO 8601
    ///
    pub fn get_create_time(&self) -> &String {
        &self.automation.create_time
    }
    ///
    /// Devuelve la fecha y el tiempo en que fue iniciada la automatizacion
    /// formato ISO 8601
    ///
    pub fn get_start_time(&self) -> &String {
        &self.automation.start_time
    }
    ///
    /// Devuelve el total de emails enviados
    ///
    pub fn get_emails_sent(&self) -> &u64 {
        &self.automation.emails_sent
    }
    ///
    /// Devuelve una lista de configuraciones de la automatizacion
    ///
    pub fn get_recipients(&self) -> &RecipientType {
        &self.automation.recipients
    }
    ///
    /// Disparadores disponibles en el flujo de la automatización
    ///
    pub fn get_trigger_settings(&self) -> &AutomationTriggerType {
        &self.automation.trigger_settings
    }
}

///
/// Listados de Recursos de tipo Automatizacion
///
pub type AutomationWorkflowResources = Vec<AutomationWorkflowResource>;
