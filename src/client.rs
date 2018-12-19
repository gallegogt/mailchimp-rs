use crate::automation_workflow_resource::{
    AutomationWorkflowResource, AutomationWorkflowResources,
};
use crate::internal::request::MailchimpResult;
use crate::internal::types::{
    AuthorizedAppType, AuthorizedAppsType, AutomationWorkflowType, AutomationsType,
    CreatedAuthorizedAppType,
};
use crate::{MailchimpApi, RequestMethod};

use std::collections::HashMap;

///
/// Implementación del cliente para el API
///
/// # Ejemplo
///
/// En este ejemplo se imrpimen el tricker actual de todos los mercados disponibles
///
/// ```
/// extern crate mailchimp_rs;
/// use mailchimp_rs::MailchimpClient;
///
/// const DC: &'static str = "<DC>";
/// const API_KEY: &'static str = "<API KEY>";
///
/// let client = MailchimpClient::new(API_KEY, API_KEY);
/// ```
///
pub struct MailchimpClient {
    api: MailchimpApi,
}

impl MailchimpClient {
    ///
    /// Crea una instancia pasandole como parámetros el Mailchimp Datacentery el API Key
    ///
    /// Argumentos:
    ///
    ///     dc: Datacenter
    ///     api_key: API KEY
    ///
    pub fn new<'a>(dc: &'a str, api_key: &'a str) -> Self {
        MailchimpClient {
            api: MailchimpApi::new(dc, api_key),
        }
    }
    ///
    ///  ===================== AUTHORIZED ==================
    ///
    ///
    /// Devuelve una lista de las aplicaciones conectadas y registradas de una cuenta.
    ///
    /// Argumentos:
    ///
    ///     filters: Filtros que requieras aplicar a la hora de obtener las aplicaciones
    ///
    ///         fields: Una lista de campos separados por comas para devolver.
    ///             Parámetros de referencia de subobjetos con notación de puntos.
    ///         exclude_fields: Una lista de campos separados por comas para excluir.
    ///            Parámetros de referencia de subobjetos con notación de puntos.
    ///         count: Cantidad de registros a devolver
    ///         offset: El número de registros de una colección a omitir. Por defecto es 0
    ///
    pub fn get_authorized_apps(
        &self,
        filters: HashMap<String, String>,
    ) -> MailchimpResult<Vec<AuthorizedAppType>> {
        let resp =
            self.api
                .call::<AuthorizedAppsType>(RequestMethod::Get, "authorized-apps", filters);
        match resp {
            Ok(value) => Ok(value.apps.clone()),
            Err(e) => Err(e),
        }
    }
    ///
    /// Obtiene las credenciales basadas en OAuth2 para asociar las llamadas a
    /// la API con su aplicación.
    ///
    /// Argumentos:
    ///
    ///     client_id: Id o nombre de usuario único para la autorización
    ///     client_secret: Contraseña del cliente para la autorización
    ///
    pub fn link_authorized_apps<'a>(
        &self,
        client_id: &'a str,
        client_secret: &'a str,
    ) -> MailchimpResult<CreatedAuthorizedAppType> {
        let mut payload = HashMap::new();
        payload.insert("client_id".to_string(), client_id.to_string());
        payload.insert("client_secret".to_string(), client_secret.to_string());

        let resp = self.api.call::<CreatedAuthorizedAppType>(
            RequestMethod::Post,
            "authorized-apps",
            payload,
        );

        match resp {
            Ok(value) => Ok(value.clone()),
            Err(e) => Err(e),
        }
    }
    ///
    ///
    /// Devuelve una lista de las aplicaciones conectadas y registradas de una cuenta.
    ///
    /// Argumentos:
    ///     app_id: identificador de la aplicación autorizada
    ///     filters: Filtros que requieras aplicar a la hora de obtener las aplicaciones
    ///
    ///         fields: Una lista de campos separados por comas para devolver.
    ///             Parámetros de referencia de subobjetos con notación de puntos.
    ///         exclude_fields: Una lista de campos separados por comas para excluir.
    ///            Parámetros de referencia de subobjetos con notación de puntos.
    ///
    pub fn get_authorized_app_info<'a>(
        &self,
        app_id: &'a str,
        filters: HashMap<String, String>,
    ) -> MailchimpResult<AuthorizedAppType> {
        let endpoint = String::from("authorized-apps/") + app_id;
        let resp =
            self.api
                .call::<AuthorizedAppType>(RequestMethod::Get, endpoint.as_str(), filters);
        match resp {
            Ok(value) => Ok(value.clone()),
            Err(e) => Err(e),
        }
    }

    ///
    ///  ===================== ACCOUNTS AUTOMATION ==================
    ///
    /// Devuelve un resumen de las automatizaciones de una cuenta.
    ///
    /// Argumentos:
    ///
    ///     filters: Filtros que se requieran aplicar a la hora de obtener las automatizaciones
    ///         Estos filtros se deben pasar en forma de llave, valor donde las llaves puede ser
    ///         cualquiera de los siguientes:
    ///
    ///         fields: Una lista de campos separados por comas para devolver.
    ///             Parámetros de referencia de subobjetos con notación de puntos.
    ///         exclude_fields: Una lista de campos separados por comas para excluir.
    ///            Parámetros de referencia de subobjetos con notación de puntos.
    ///         before_create_time: Restringe la respuesta a las automatizaciones creadas
    ///             antes del tiempo establecido. Recomendamos el formato de hora
    ///             ISO 8601: 2015-10-21T15: 41: 36 + 00: 00.
    ///         since_create_time: Restringe la respuesta a las automatizaciones creadas
    ///             después del tiempo establecido. Recomendamos el formato de hora
    ///             ISO 8601: 2015-10-21T15: 41: 36 + 00: 00.
    ///         before_send_time: Restringe la respuesta a las automatizaciones enviadas
    ///             antes del tiempo establecido. Recomendamos el formato de hora
    ///             ISO 8601: 2015-10-21T15: 41: 36 + 00: 00.
    ///         since_send_time: Restringe la respuesta a las automatizaciones enviadas después
    ///             del tiempo establecido. Recomendamos el formato de hora
    ///             ISO 8601: 2015-10-21T15: 41: 36 + 00: 00.
    ///         status: Restringe los resultados a automatizaciones con el estado especificado.
    ///
    pub fn get_account_automations(
        &self,
        filters: HashMap<String, String>,
    ) -> MailchimpResult<AutomationWorkflowResources> {
        let response = self
            .api
            .call::<AutomationsType>(RequestMethod::Get, "automations", filters);

        match response {
            Ok(value) => {
                let automations = value
                    .automations
                    .iter()
                    .map(move |data| {
                        AutomationWorkflowResource::new(self.api.clone(), data.clone())
                    })
                    .collect::<AutomationWorkflowResources>();
                Ok(automations)
            }
            Err(e) => Err(e),
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
    ///
    ///         fields: Una lista de campos separados por comas para devolver.
    ///             Parámetros de referencia de subobjetos con notación de puntos.
    ///         exclude_fields: Una lista de campos separados por comas para excluir.
    ///            Parámetros de referencia de subobjetos con notación de puntos.
    ///
    pub fn get_automation_workflow_info<'a>(
        &self,
        workflow_id: &'a str,
        filters: HashMap<String, String>,
    ) -> MailchimpResult<AutomationWorkflowResource> {
        let endpoint = String::from("automations/") + workflow_id;
        let response =
            self.api
                .call::<AutomationWorkflowType>(RequestMethod::Get, endpoint.as_str(), filters);

        match response {
            Ok(automation) => Ok(AutomationWorkflowResource::new(
                self.api.clone(),
                automation.clone(),
            )),
            Err(e) => Err(e),
        }
    }
}
