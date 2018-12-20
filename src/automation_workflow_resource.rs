use crate::api::MailchimpApi;
use crate::internal::types::{
    AutomationCampaignSettingsType, AutomationTrackingOptionsType, AutomationTriggerType,
    AutomationWorkflowType, CampaignReportSummaryType, EmptyType, MailchimpErrorType,
    RecipientType,
};
use std::collections::HashMap;

///
/// AutomationWorkflowResource
///
#[derive(Debug, Clone)]
pub struct AutomationWorkflowResource {
    api: MailchimpApi,
    id: String,
    automation: AutomationWorkflowType,
}

impl AutomationWorkflowResource {
    ///
    /// Argumentos:
    ///     api: MailchimpApi
    ///     automation: AutomationWorkflowType
    ///
    pub fn new(api: MailchimpApi, automation: &AutomationWorkflowType) -> Self {
        let automation_id = automation.id.as_ref();

        AutomationWorkflowResource {
            api: api,
            id: automation_id.unwrap().to_string(),
            automation: automation.clone(),
        }
    }
    ///
    /// Devuelve el resumen de la campaña
    ///
    pub fn get_report_summary(&self) -> &Option<CampaignReportSummaryType> {
        &self.automation.report_summary
    }
    ///
    /// Devuelve la configuracion de la campaña
    ///
    pub fn get_settings(&self) -> &Option<AutomationCampaignSettingsType> {
        &self.automation.settings
    }

    ///
    /// Devuelve el estado de la automatizacion
    ///
    pub fn get_status(&self) -> &Option<String> {
        &self.automation.status
    }

    ///
    /// Devuelve las opciones de seguimiento para la automatización.
    ///
    pub fn get_tracking(&self) -> &Option<AutomationTrackingOptionsType> {
        &self.automation.tracking
    }

    ///
    /// Devuelve el identificador de la automatizacion
    ///
    pub fn get_id(&self) -> &Option<String> {
        &self.automation.id
    }
    ///
    /// Devuelve la fecha y el tiempo en que fue creada la automatizacion
    /// formato ISO 8601
    ///
    pub fn get_create_time(&self) -> &Option<String> {
        &self.automation.create_time
    }
    ///
    /// Devuelve la fecha y el tiempo en que fue iniciada la automatizacion
    /// formato ISO 8601
    ///
    pub fn get_start_time(&self) -> &Option<String> {
        &self.automation.start_time
    }
    ///
    /// Devuelve el total de emails enviados
    ///
    pub fn get_emails_sent(&self) -> &Option<u64> {
        &self.automation.emails_sent
    }
    ///
    /// Devuelve una lista de configuraciones de la automatizacion
    ///
    pub fn get_recipients(&self) -> &Option<RecipientType> {
        &self.automation.recipients
    }
    ///
    /// Disparadores disponibles en el flujo de la automatización
    ///
    pub fn get_trigger_settings(&self) -> &Option<AutomationTriggerType> {
        &self.automation.trigger_settings
    }

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
        match self.api.post::<EmptyType, HashMap<String, String>>(
            b_endpoint.as_str(),
            HashMap::new(),
        ) {
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
        match self.api.post::<EmptyType, HashMap<String, String>>(
            b_endpoint.as_str(),
            HashMap::new(),
        ) {
            Ok(_) => None,
            Err(e) => Some(e),
        }
    }

    // ============== Private Functions ==============
    fn get_base_endpoint(&self) -> String {
        let mut b_endpoint = String::from("automations/");
        b_endpoint.push_str(self.id.as_str());
        b_endpoint
    }
}

///
/// Listados de Recursos de tipo Automatizacion
///
pub type AutomationWorkflowResources = Vec<AutomationWorkflowResource>;
