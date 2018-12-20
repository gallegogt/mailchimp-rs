use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde_json;
use std::collections::HashMap;

use crate::internal::request::{BasicAuth, HttpReq, MailchimpResult};
use crate::internal::types::MailchimpErrorType;

///
/// Definición del API Interno
///
#[derive(Debug, Clone)]
pub struct Api<R>
where
    R: HttpReq,
{
    domain: String,
    api_version: String,
    api_key: String,
    req: Box<R>,
    basic_auth: Option<BasicAuth>,
}

impl<R> Api<R>
where
    R: HttpReq,
{
    ///
    /// Devuelve la instancia del API para el acceso mediante el ACCESS_TOKEN
    ///
    /// Argumentos
    ///     dc: Mailchimp Datacenter
    ///     api_key: Mailchimp Access Token
    ///     http_transport: Interfaz por donde se harían las peticiones Get y Post al servicio
    ///
    pub fn new<'a>(dc: &'a str, api_key: &'a str, http_transport: Box<R>) -> Self {
        Api {
            domain: format!("https://{}.api.mailchimp.com/", dc),
            api_version: "3.0".to_string(),
            api_key: api_key.to_string(),
            req: http_transport,
            basic_auth: Some(BasicAuth {
                username: "".to_string(),
                api_token: api_key.to_string(),
            }),
        }
    }
    ///
    /// Devuelve una cadena de texto con el dominio
    ///
    pub fn domain(&self) -> String {
        self.domain.clone()
    }
    ///
    /// Devuelve una cadena de texto la versión del API
    ///
    pub fn api_version(&self) -> String {
        self.api_version.clone()
    }

    ///
    /// Función para darle forma a la url
    ///
    /// Argumentos
    ///     endpoint: Endpoint desde donde se va a extraer los datos
    ///     params: Parámetros de la url
    ///
    pub fn build_url<'a>(&self, endpoint: &'a str, params: &HashMap<String, String>) -> Url {
        let mut api_url = Url::parse(&self.domain).unwrap();
        let data = self.api_version.clone() + "/";
        // Adiciona la versión del API
        api_url = api_url
            .join(data.as_str())
            .unwrap();
        // Adiciona Endpoint
        api_url = api_url.join(endpoint).unwrap();
        for (key, value) in params {
            api_url
                .query_pairs_mut()
                .append_pair(key.as_str(), value.as_str());
        }
        api_url
    }

    ///
    /// Conforma los headers para realizar la petición al servidor
    ///
    pub fn build_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_str("application/json").unwrap(),
        );
        headers
    }

    ///
    ///
    /// Argumentos
    ///     endpoint: Endpoint desde donde se va a extraer los datos
    ///     params: Parámetros de la url
    ///     is_public: indica si el endpoint es public
    ///
    pub fn get_edge<'a, T>(
        &self,
        endpoint: &'a str,
        params: HashMap<String, String>,
    ) -> MailchimpResult<T>
    where
        T: DeserializeOwned,
    {
        let api_url = self.build_url(endpoint, &params);
        let headers = self.build_headers();
        let mut result = self.req.get(api_url, headers, &self.basic_auth)?;
        if result.len() == 0 {
            result = "{}".to_string();
        }
        match serde_json::from_str(&result) {
            Ok(sr) => Ok(sr),
            Err(e) => {
                println!("{:?}", e);
                Err(MailchimpErrorType::default())
            }
        }
    }

    ///
    ///
    /// Argumentos
    ///     endpoint: Endpoint hacia donde se van a enviar los datos
    ///     payload: Dato a enviar
    ///
    ///
    pub fn post_edge<'a, T>(
        &self,
        endpoint: &'a str,
        payload: HashMap<String, String>,
    ) -> MailchimpResult<T>
    where
        T: DeserializeOwned,
    {
        let api_url = self.build_url(endpoint, &HashMap::new());
        let headers = self.build_headers();
        let mut result = self.req.post(api_url, headers, payload, &self.basic_auth)?;
        if result.len() == 0 {
            result = "{}".to_string();
        }
        match serde_json::from_str(&result) {
            Ok(sr) => Ok(sr),
            Err(e) => {
                println!("Post Edge {:?}", e);
                Err(MailchimpErrorType::default())
            }
        }
    }
}
