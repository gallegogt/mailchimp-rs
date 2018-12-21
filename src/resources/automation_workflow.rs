use crate::api::MailchimpApi;
use crate::internal::request::MailchimpResult;
use crate::internal::types::{
    AutomationCampaignSettingsType, AutomationDelayType, AutomationModifier,
    AutomationTrackingOptionsType, AutomationTriggerType, AutomationWorkflowType,
    CampaignReportSummaryType, EmptyType, MailchimpErrorType, RecipientType, WorkflowEmailType,
    WorkflowEmailsType,
};
use super::workflow_email::{WorkflowEmailResource, WorkflowEmailResources};
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
        match self
            .api
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
            .api
            .post::<EmptyType, HashMap<String, String>>(b_endpoint.as_str(), HashMap::new())
        {
            Ok(_) => None,
            Err(e) => Some(e),
        }
    }
    ///
    /// Actualiza la automatizació y devuelve una instancia nueva
    ///
    /// Argumentos:
    ///     settings: Configuracion de la automatización a crear
    ///     delay: Ajustes de retraso para un correo electrónico de automatización.
    ///
    pub fn remote_update<'a>(
        &self,
        settings: Option<AutomationCampaignSettingsType>,
        delay: Option<AutomationDelayType>,
    ) -> MailchimpResult<AutomationWorkflowResource> {
        let modifier = AutomationModifier {
            settings: settings,
            delay: delay,
            recipients: None,
            trigger_settings: None,
        };
        let response = self
            .api
            .post::<AutomationWorkflowType, AutomationModifier>("automations", modifier);
        match response {
            Ok(automation) => Ok(AutomationWorkflowResource::new(
                self.api.clone(),
                &automation,
            )),
            Err(e) => Err(e),
        }
    }
    /// ============= EMAILS ============================
    ///
    /// Devuelve una lista de los emails automatizados
    ///
    pub fn get_workflow_emails(&self) -> MailchimpResult<WorkflowEmailResources> {
        let endpoint = self.get_base_endpoint() + "/emails";
        let response = self
            .api
            .get::<WorkflowEmailsType>(endpoint.as_str(), HashMap::new());
        match response {
            Ok(value) => {
                let emails = value
                    .emails
                    .iter()
                    .map(move |data| {
                        let mut inner = endpoint.clone();
                        inner.push_str(data.id.as_ref().unwrap());
                        WorkflowEmailResource::new(self.api.clone(), &data, &inner)
                    })
                    .collect::<WorkflowEmailResources>();
                Ok(emails)
            }
            Err(e) => Err(e),
        }
    }

    ///
    /// Devuelve la informacion sobre un flujo de trabajos de automatizacion de emails
    ///
    /// Argumentos:
    ///     workflow_email_id: Identificador único de la automatización
    ///
    pub fn get_automation_workflow_info<'a>(
        &self,
        workflow_email_id: &'a str,
    ) -> MailchimpResult<WorkflowEmailResource> {
        let mut endpoint = self.get_base_endpoint().clone();
        endpoint.push_str("/emails/");
        endpoint.push_str(workflow_email_id);

        let response = self
            .api
            .get::<WorkflowEmailType>(endpoint.as_str(), HashMap::new());

        match response {
            Ok(workflow_email) => Ok(WorkflowEmailResource::new(
                self.api.clone(),
                &workflow_email,
                &endpoint
            )),
            Err(e) => Err(e),
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
