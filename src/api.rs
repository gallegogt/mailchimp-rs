use crate::internal::api::Api;
use crate::internal::error_type::MailchimpErrorType;
use crate::internal::request::MailchimpRequest;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use std::collections::HashMap;

///
/// Mailchimp API
///
/// Permite el acceso al API de Mailchimp si conoces bien los diferentes
/// endpoints.
/// Para más información sobre los endpoints ir a la página de desarrollos
/// mailchimp [Developers Mailchimp](https://developer.mailchimp.com/)
///
/// ## Ejemplo
///
/// ```
/// extern crate mailchimp;
/// use mailchimp::MailchimpApi;
///
/// let api = MailchimpApi::new("<API Key>");
/// println!("Api version: {}", api.version());
/// println!("Api domain: {}", api.domain());
/// ```
///
#[derive(Debug, Clone)]
pub struct MailchimpApi {
    i_api: Box<Api<MailchimpRequest>>,
}

impl MailchimpApi {
    ///
    /// Crea la nueva instancia del API
    ///
    /// Argumentos
    ///     api_key: Mailchimp API KEY
    ///     http_transport: Interfaz por donde se harían las peticiones Get y Post al servicio
    pub fn new<'a>(api_key: &'a str) -> Self {
        let mut creds = api_key.split('-').collect::<Vec<&str>>();
        if creds.len() <= 1 {
            creds.push("usX");
        }
        MailchimpApi {
            i_api: Box::new(Api::<MailchimpRequest>::new(
                creds[1],
                creds[0],
                Box::new(MailchimpRequest::new()),
            )),
        }
    }
    ///
    /// Devuelve el dominio
    ///
    pub fn domain(&self) -> String {
        self.i_api.domain()
    }

    ///
    /// Devuelve la version del API
    ///
    pub fn version(&self) -> String {
        self.i_api.api_version()
    }

    ///
    /// Realiza una petición de tipo POST
    /// ```
    /// extern crate mailchimp;
    /// use std::collections::HashMap;
    /// use mailchimp::MailchimpApi;
    /// use mailchimp::AuthorizedAppsType;
    /// fn main() {
    ///     let api = MailchimpApi::new("aac1e319006883125e18a89e529b5abb73de4c81-usX");
    ///     let data = api.post::<AuthorizedAppsType, HashMap<String, String>>("authorized-apps", HashMap::new());
    ///     match data {
    ///         Ok(resp) => {
    ///             for app in resp.apps.iter() {
    ///                 println!("{:?}", app)
    ///             }
    ///         },
    ///         Err(e) => println!("Error Title: {:?} \n Error detail {:?}", e.title, e.detail)
    ///     }
    /// }
    /// ```
    /// #Argumentos
    ///     `endpoint`: Cadena de texto con el endpoint de la API al que se requiere acceder, no debe comenzar por "/"
    ///     `payload`: Dato a enviar al servidor
    ///
    pub fn post<'a, T, P>(&self, endpoint: &'a str, payload: P) -> Result<T, MailchimpErrorType>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        self.i_api.post_edge::<T, P>(endpoint, payload)
    }

    ///
    /// Realiza una petición de tipo GET
    /// ```
    /// extern crate mailchimp;
    /// use std::collections::HashMap;
    /// use mailchimp::MailchimpApi;
    /// use mailchimp::AuthorizedAppType;
    ///
    /// fn main() {
    ///     let api = MailchimpApi::new("aac1e319006883125e18a89e529b5abb73de4c81-usX");
    ///     let mut params = HashMap::new();
    ///     params.insert("client_id".to_string(), "".to_string());
    ///     params.insert("client_secret".to_string(), "".to_string());
    ///     let data = api.get::<AuthorizedAppType>("authorized-apps", params);
    ///     match data {
    ///         Ok(resp) => {
    ///            println!("{:?}", resp)
    ///         },
    ///         Err(e) => println!("Error Title: {:?} \n Error detail {:?}", e.title, e.detail)
    ///     }
    /// }
    /// ```
    /// #Argumentos
    ///     `endpoint`: Cadena de texto con el endpoint de la API al que se requiere acceder, no debe comenzar por "/"
    ///     `payload`: Listado llave valor de los parametros o data
    ///
    pub fn get<'a, T>(
        &self,
        endpoint: &'a str,
        payload: HashMap<String, String>,
    ) -> Result<T, MailchimpErrorType>
    where
        T: DeserializeOwned,
    {
        self.i_api.get_edge(endpoint, payload)
    }
}

impl Default for MailchimpApi {
    fn default() -> Self {
        MailchimpApi {
            i_api: Box::new(Api::<MailchimpRequest>::new(
                "",
                "",
                Box::new(MailchimpRequest::new()),
            )),
        }
    }
}

pub trait MailchimpApiUpdate {
    /**
     * Update API
     */
    fn set_api(&mut self, api: &MailchimpApi);
}
