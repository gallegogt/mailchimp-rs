use crate::api::MailchimpApi;
use crate::internal::types::{ AuthorizedAppType, LinkType};

///
/// AuthorizedAppResource
///
#[derive(Debug, Clone)]
pub struct AuthorizedAppResource {
    api: MailchimpApi,
    inner_auth_app: AuthorizedAppType,
}

impl AuthorizedAppResource {
    ///
    /// Argumentos:
    ///     api: MailchimpApi
    ///     automation: AuthorizedAppResource
    ///
    pub fn new(api: MailchimpApi, auth_app: &AuthorizedAppType) -> Self {
        AuthorizedAppResource {
            api: api,
            inner_auth_app: auth_app.clone(),
        }
    }
    ///
    /// The ID for the application.
    ///
    pub fn id(&self) -> &u64 {
        &self.inner_auth_app.id
    }
    ///
    /// Devuelve la configuracion de la campaña
    ///
    pub fn name(&self) -> &String {
        &self.inner_auth_app.name
    }

    ///
    /// A short description of the application.
    ///
    pub fn description(&self) -> &String {
        &self.inner_auth_app.description
    }

    ///
    /// An array of usernames for users who have linked the app.
    ///
    pub fn users(&self) -> &Vec<String> {
        &self.inner_auth_app.users
    }
    ///
    /// An array of usernames for users who have linked the app.
    ///
    pub fn links(&self) -> &Vec<LinkType> {
        &self.inner_auth_app._links
    }
}

///
/// Listados de Recursos de tipo Automatizacion
///
pub type AuthorizedAppResources = Vec<AuthorizedAppResource>;
