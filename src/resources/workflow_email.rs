use crate::api::MailchimpApi;
use crate::internal::types::{
    AutomationCampaignSettingsType, AutomationTrackingOptionsType, AutomationTriggerType,
    CampaignReportSummaryType, RecipientType, WorkflowEmailType, EmptyType, MailchimpErrorType
};
use std::collections::HashMap;

///
/// WorkflowEmailResource
///
#[derive(Debug, Clone)]
pub struct WorkflowEmailResource {
    api: MailchimpApi,
    id: String,
    workflow_email: WorkflowEmailType,
    endpoint: String,
}

impl WorkflowEmailResource {
    ///
    /// Argumentos:
    ///     api: MailchimpApi
    ///     workflow_email: WorkflowEmailType
    ///
    pub fn new<'a>(api: MailchimpApi, workflow_email: &WorkflowEmailType, endpoint: &'a str) -> Self {
        let workflow_mail_id = workflow_email.id.as_ref();

        WorkflowEmailResource {
            api: api,
            id: workflow_mail_id.unwrap().to_string(),
            workflow_email: workflow_email.clone(),
            endpoint: endpoint.to_string(),
        }
    }
    ///
    /// Devuelve el resumen de la campaña
    ///
    pub fn get_report_summary(&self) -> &Option<CampaignReportSummaryType> {
        &self.workflow_email.report_summary
    }
    ///
    /// Devuelve la configuracion de la campaña
    ///
    pub fn get_settings(&self) -> &Option<AutomationCampaignSettingsType> {
        &self.workflow_email.settings
    }

    ///
    /// Devuelve las opciones de seguimiento para la automatización.
    ///
    pub fn get_tracking(&self) -> &Option<AutomationTrackingOptionsType> {
        &self.workflow_email.tracking
    }

    ///
    /// Devuelve el identificador de la automatizacion
    ///
    pub fn get_id(&self) -> &Option<String> {
        &self.workflow_email.id
    }
    ///
    /// Devuelve la fecha y el tiempo en que fue creada la automatizacion
    /// formato ISO 8601
    ///
    pub fn get_create_time(&self) -> &Option<String> {
        &self.workflow_email.create_time
    }
    ///
    /// Devuelve la fecha y el tiempo en que fue iniciada la automatizacion
    /// formato ISO 8601
    ///
    pub fn get_start_time(&self) -> &Option<String> {
        &self.workflow_email.start_time
    }
    ///
    /// Devuelve el total de emails enviados
    ///
    pub fn get_emails_sent(&self) -> &Option<u64> {
        &self.workflow_email.emails_sent
    }
    ///
    /// Devuelve una lista de configuraciones de la automatizacion
    ///
    pub fn get_recipients(&self) -> &Option<RecipientType> {
        &self.workflow_email.recipients
    }
    ///
    /// Disparadores disponibles en el flujo de la automatización
    ///
    pub fn get_trigger_settings(&self) -> &Option<AutomationTriggerType> {
        &self.workflow_email.trigger_settings
    }

    // ============== Actions ==============
    ///
    /// Detiene un email automatizado
    ///
    /// En caso de ser satisfactoria la ejecución, devuelve None,
    /// en caso contrario devuelve el error, con su respectiva descripción
    ///
    pub fn pause_all_emails(&self) -> Option<MailchimpErrorType> {
        let mut b_endpoint = self.endpoint.clone();
        b_endpoint.push_str("/actions/pause");
        match self
            .api
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
        let mut b_endpoint = self.endpoint.clone();
        b_endpoint.push_str("/actions/start");
        match self
            .api
            .post::<EmptyType, HashMap<String, String>>(b_endpoint.as_str(), HashMap::new())
        {
            Ok(_) => None,
            Err(e) => Some(e),
        }
    }

    pub fn get_endpoint(&self) -> &String {
        &self.endpoint
    }
}

///
/// Listados de Recursos de tipo Automatizacion
///
pub type WorkflowEmailResources = Vec<WorkflowEmailResource>;
