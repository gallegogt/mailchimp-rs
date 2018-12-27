use super::api::MailchimpApi;
use super::internal::request::MailchimpResult;
use super::internal::types::{
    AuthorizedAppType, AuthorizedAppsType, CreatedAuthorizedAppType
};
use super::iter::{BuildIter, MalchimpIter};
use super::resources::AuthorizedAppResource;
use log::error;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AuthorizedFilter {
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
}

impl Default for AuthorizedFilter {
    fn default() -> Self {
        AuthorizedFilter {
            fields: None,
            exclude_fields: None,
            count: Some(50),
            offset: Some(0),
        }
    }
}

impl AuthorizedFilter {
    pub fn get_payload(&self) -> HashMap<String, String> {
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
        payload
    }
}

///
/// campaigns
///
#[derive(Debug, Clone)]
pub struct AuthorizedApps {
    api: MailchimpApi,
}

impl BuildIter for AuthorizedApps {
    type Item = AuthorizedAppType;
    type Resource = AuthorizedAppResource;
    type FilterItem = AuthorizedFilter;

    ///
    /// Obtiene los datos remotos y devuelve un listado
    ///
    fn get_data_from_remote(&self, filter: &Self::FilterItem) -> Vec<Self::Item> {
        if let Some(resp) = self.get_authorized_apps_from_remote(Some(filter)) {
            return resp.apps;
        }
        Vec::new()
    }
    ///
    /// Crea un recurso a partir del dato pasado por parámetro
    ///
    fn create_resource(&self, data: &Self::Item) -> Self::Resource {
        AuthorizedAppResource::new(self.api.clone(), &data)
    }
    ///
    /// Actualiza el offset
    ///
    fn update_filter_offset(&self, filter: &Self::FilterItem) -> Self::FilterItem {
        let mut f = filter.clone();
        f.offset = Some(f.count.unwrap() + f.offset.unwrap());
        f
    }
}

impl AuthorizedApps {
    ///
    /// Argumentos:
    ///     api: MailchimpApi
    ///
    pub fn new(api: MailchimpApi) -> Self {
        AuthorizedApps { api: api }
    }

    ///
    /// Devuelve información de las listas creadas
    ///
    ///
    /// Devuelve una lista de las aplicaciones conectadas y registradas de una cuenta.
    ///
    /// Argumentos:
    ///     filters: Filtros que requieras aplicar a la hora de obtener las aplicaciones
    ///         fields: Una lista de campos separados por comas para devolver.
    ///             Parámetros de referencia de subobjetos con notación de puntos.
    ///         exclude_fields: Una lista de campos separados por comas para excluir.
    ///            Parámetros de referencia de subobjetos con notación de puntos.
    ///         count: Cantidad de registros a devolver
    ///         offset: El número de registros de una colección a omitir. Por defecto es 0
    ///
    pub fn get_authorized_apps_from_remote(
        &self,
        filters: Option<&AuthorizedFilter>,
    ) -> Option<AuthorizedAppsType> {
        let mut payload = HashMap::new();
        if filters.is_some() {
            payload = filters.as_ref().unwrap().get_payload();
        }
        let response = self.api.get::<AuthorizedAppsType>("authorized-apps", payload);
        match response {
            Ok(value) => Some(value),
            Err(e) => {
                error!( target: "mailchimp",  "Load Authorized Apps from remote: Response Error details: {:?}", e);
                None
            }
        }
    }

    ///
    /// Obtiene las credenciales basadas en OAuth2 para asociar las llamadas a
    /// la API con su aplicación.
    ///
    /// Argumentos:
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

        let resp = self
            .api
            .post::<CreatedAuthorizedAppType, HashMap<String, String>>("authorized-apps", payload);

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
        let resp = self
            .api
            .get::<AuthorizedAppType>(endpoint.as_str(), filters);
        match resp {
            Ok(value) => Ok(value.clone()),
            Err(e) => Err(e),
        }
    }

    ///
    /// Función para recorrer todas las campañas exitentes. A diferencia de la
    /// anterior esta función te devuelve un iterador
    ///
    pub fn iter(&self, filters: AuthorizedFilter) -> MalchimpIter<Self> {
        if let Some(remote) = self.get_authorized_apps_from_remote(Some(&filters)) {
            return MalchimpIter {
                builder: &self,
                data: remote.apps,
                cur_filters: filters.clone(),
                cur_it: 0,
            };
        }
        MalchimpIter {
            builder: &self,
            data: Vec::new(),
            cur_filters: filters.clone(),
            cur_it: 0,
        }
    }
}
