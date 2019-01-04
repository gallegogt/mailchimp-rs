use super::api::MailchimpApi;
use super::internal::request::MailchimpResult;
use super::types::ApiRootType;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ApiRoot {
    api: MailchimpApi,
}

impl ApiRoot {
    ///
    /// Argumentos:
    ///     api: MailchimpApi
    ///
    pub fn new(api: MailchimpApi) -> Self {
        ApiRoot { api: api }
    }

    ///
    /// Devuelve detalles de la cuenta de usuario, asi como los links a los recursos asociados
    ///
    /// Argumentos:
    ///     filters: Filtros que requieras aplicar a la hora de obtener las aplicaciones
    ///         fields: Una lista de campos separados por comas para devolver.
    ///             Parámetros de referencia de subobjetos con notación de puntos.
    ///         exclude_fields: Una lista de campos separados por comas para excluir.
    ///            Parámetros de referencia de subobjetos con notación de puntos.
    ///
    pub fn get_info(&self, filters: HashMap<String, String>) -> MailchimpResult<ApiRootType> {
        let resp = self.api.get::<ApiRootType>("", filters);
        match resp {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }
}
