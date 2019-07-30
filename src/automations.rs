//! Implement Mailchimp Automations Endpoint
//!
//! Mailchimp’s free Automation feature lets you build a series of
//! emails that send to subscribers when triggered by a specific date, activity,
//! or event. Use the API to manage Automation workflows, emails, and queues.
//!
//! ```
//!     use mailchimp::MailchimpApi;
//!     use mailchimp::{Automations, AutomationsFilter};
//!     use std::collections::HashMap;
//!
//!     fn main() {
//!         let api = MailchimpApi::new("<API_KEY>");
//!
//!         // Create Instance
//!         let automations = Automations::new(api);
//!
//!         // Get information about all automations in the account.
//!         for w in automations.iter(AutomationsFilter::default()) {
//!             println!("Title             {:?}", settings.title);
//!             println!("Emails Sent       {:?}", w.emails_sent);
//!             println!("Report Summary    {:?}", w.report_summary);
//!             println!("Start Time        {:?}", w.start_time);
//!         }
//!     }
//! ```
//!

use super::api::{MailchimpApi, MailchimpApiUpdate};
use super::internal::request::MailchimpResult;
use super::iter::{BuildIter, MalchimpIter, ResourceFilter};
use super::types::{
    AutomationCampaignSettingsType, AutomationModifier, AutomationTriggerType,
    AutomationWorkflowType, CollectionAutomation, RecipientType,
};
use log::error;
use std::collections::HashMap;

/// Automation Request Filter
#[derive(Debug, Clone)]
pub struct AutomationsFilter {
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
    /// The status of the campaign.
    pub status: Option<String>,
    /// Restrict the response to automations sent before the set time. We recommend
    /// ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    pub before_send_time: Option<String>,
    /// Restrict the response to automations sent after the set time. We recommend
    /// ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    pub since_send_time: Option<String>,
    /// Restrict the response to automations created before the set time. We recommend
    /// ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    pub before_create_time: Option<String>,
    /// Restrict the response to automations created after the set time. We recommend
    /// ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    pub since_create_time: Option<String>,
}

impl Default for AutomationsFilter {
    fn default() -> Self {
        AutomationsFilter {
            fields: None,
            exclude_fields: None,
            count: Some(50),
            offset: Some(0),
            status: None,
            before_send_time: None,
            since_send_time: None,
            before_create_time: None,
            since_create_time: None,
        }
    }
}

impl ResourceFilter for AutomationsFilter {
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
        if self.status.is_some() {
            payload.insert("status".to_string(), self.status.as_ref().unwrap().clone());
        }
        if self.before_send_time.is_some() {
            payload.insert(
                "before_send_time".to_string(),
                self.before_send_time.as_ref().unwrap().clone(),
            );
        }
        if self.since_send_time.is_some() {
            payload.insert(
                "since_send_time".to_string(),
                self.since_send_time.as_ref().unwrap().clone(),
            );
        }
        if self.before_create_time.is_some() {
            payload.insert(
                "before_create_time".to_string(),
                self.before_create_time.as_ref().unwrap().clone(),
            );
        }
        if self.since_create_time.is_some() {
            payload.insert(
                "since_create_time".to_string(),
                self.since_create_time.as_ref().unwrap().clone(),
            );
        }
        payload
    }
}

///
/// Implement Mailchimp Automations Endpoint
///
/// Mailchimp’s free Automation feature lets you build a series of
/// emails that send to subscribers when triggered by a specific date, activity,
/// or event. Use the API to manage Automation workflows, emails, and queues
///
#[derive(Debug, Clone)]
pub struct Automations {
    api: MailchimpApi,
}

#[derive(Debug)]
pub struct AutomationsBuilder {}

impl BuildIter for AutomationsBuilder {
    type Item = AutomationWorkflowType;
    type FilterItem = AutomationsFilter;
    type Collection = CollectionAutomation;

    ///
    /// Create new resource, with the api instance updated
    ///
    fn update_item(&self, data: &Self::Item, api: &MailchimpApi) -> Self::Item {
        let mut in_data = data.clone();
        in_data.set_api(&api);
        in_data
    }
    ///
    /// Update filter params
    ///
    fn update_filter_offset(&self, filter: &Self::FilterItem) -> Self::FilterItem {
        let mut f = filter.clone();
        f.offset = Some(f.count.unwrap() + f.offset.unwrap());
        f
    }
}

impl Automations {
    ///
    /// Argumentos:
    ///     api: MailchimpApi
    ///
    pub fn new(api: MailchimpApi) -> Self {
        Automations { api: api }
    }

    ///
    /// Devuelve información de las listas creadas
    ///
    /// Argumentos:
    ///     filters: Filtros que se requieran aplicar a la hora de obtener las automatizaciones
    ///         Estos filtros se deben pasar en forma de llave, valor donde las llaves puede ser
    ///         cualquiera de los siguientes:
    ///         fields: listado de campos deseados, separados por coma
    ///         exclude_fields: listado de campos excluidos, separados por coma
    ///         count: Número de registros a devolver
    ///         offset: El número de registros de una colección a saltar
    ///         before_date_created: Restrict response to lists created before the set date.
    ///         since_date_created: Restrict results to lists created after the set date.
    ///         before_campaign_last_sent: Restrict results to lists created before the last campaign send date.
    ///         since_campaign_last_sent: Restrict results to lists created after the last campaign send date.
    ///         email: Restrict results to lists that include a specific subscriber’s email address.
    ///         sort_field: Returns files sorted by the specified field.
    ///         sort_dir: Determines the order direction for sorted results.
    ///         folder_id: The unique folder id.
    ///         list_id: The unique id for the list.
    ///
    pub fn get_automations_from_remote(
        &self,
        filters: Option<&AutomationsFilter>,
    ) -> Option<CollectionAutomation> {
        let mut payload = HashMap::new();
        if filters.is_some() {
            payload = filters.as_ref().unwrap().build_payload();
        }
        let response = self.api.get::<CollectionAutomation>("automations", payload);
        match response {
            Ok(value) => Some(value),
            Err(e) => {
                error!( target: "mailchimp",  "Load Campaigns from remote: Response Error details: {:?}", e);
                None
            }
        }
    }

    ///
    /// Devuelve la informacion de la automatizacion especificada
    ///
    /// Argumentos:
    ///     workflow_id: Identificador único de la automatización
    ///     filters: Filtros requeridos a la hora de obtener las automatizaciones
    ///         Estos filtros se deben pasar en forma de llave, valor donde las llaves puede ser
    ///         cualquiera de los siguientes:
    ///         fields: Una lista de campos separados por comas para devolver.
    ///             Parámetros de referencia de subobjetos con notación de puntos.
    ///         exclude_fields: Una lista de campos separados por comas para excluir.
    ///            Parámetros de referencia de subobjetos con notación de puntos.
    ///
    pub fn get_automation_workflow_info<'a>(
        &self,
        workflow_id: &'a str,
        filters: HashMap<String, String>,
    ) -> MailchimpResult<AutomationWorkflowType> {
        let endpoint = String::from("automations/") + workflow_id;
        let response = self
            .api
            .get::<AutomationWorkflowType>(endpoint.as_str(), filters);

        match response {
            Ok(automation) => {
                let mut au = automation;
                au.set_api(&self.api);
                Ok(au)
            }
            Err(e) => Err(e),
        }
    }
    ///
    ///  Crea una automatización
    ///
    /// Argumentos:
    ///     recipients: Contenedores para esta automatización
    ///     trigger_settings: Configuracion de los disparadores
    ///     settings: Configuracion de la automatización a crear
    ///
    pub fn create_automation<'a>(
        &self,
        recipients: RecipientType,
        trigger_settings: AutomationTriggerType,
        settings: Option<AutomationCampaignSettingsType>,
    ) -> MailchimpResult<AutomationWorkflowType> {
        let modifier = AutomationModifier {
            settings: settings,
            delay: None,
            recipients: Some(recipients),
            trigger_settings: Some(trigger_settings),
        };
        let response = self
            .api
            .post::<AutomationWorkflowType, AutomationModifier>("automations", modifier);
        match response {
            Ok(automation) => {
                let mut au = automation;
                au.set_api(&self.api);
                Ok(au)
            }
            Err(e) => Err(e),
        }
    }

    ///
    /// Función para recorrer todas las campañas exitentes. A diferencia de la
    /// anterior esta función te devuelve un iterador
    ///
    pub fn iter(&self, filters: AutomationsFilter) -> MalchimpIter<AutomationsBuilder> {
        if let Some(remote) = self.get_automations_from_remote(Some(&filters)) {
            return MalchimpIter {
                builder: AutomationsBuilder {},
                data: remote.automations,
                cur_filters: filters.clone(),
                cur_it: 0,
                total_items: remote.total_items,
                api: self.api.clone(),
                endpoint: "automations".to_string(),
            };
        }
        MalchimpIter {
            builder: AutomationsBuilder {},
            data: Vec::new(),
            cur_filters: filters.clone(),
            cur_it: 0,
            total_items: 0,
            api: self.api.clone(),
            endpoint: "automations".to_string(),
        }
    }
}
