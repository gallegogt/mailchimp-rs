use crate::internal::{AuthorizedAppType, AuthorizedAppsType, CreatedAuthorizedAppType};
use crate::internal::request::MailchimpResult;
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
    /// Devuelve una lista de las aplicaciones conectadas y registradas de una cuenta.
    ///
    pub fn get_authorized_apps(&self) -> MailchimpResult<Vec<AuthorizedAppType>> {
        let resp = self.api.call::<AuthorizedAppsType>(
            RequestMethod::Get,
            "authorized-apps",
            HashMap::new(),
        );
        match resp {
            Ok(value) => Ok(value.apps.clone()),
            Err(e) => Err(e)
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
            Err(e) => Err(e)
        }
    }
}
