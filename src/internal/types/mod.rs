mod error;
mod link;
mod authorized_apps;
mod automation_campaign;
mod contact;
mod api_root;
mod list;

pub use self::error::MailchimpErrorType;
pub use self::link::LinkType;
pub use self::authorized_apps::{AuthorizedAppType, AuthorizedAppsType, CreatedAuthorizedAppType};
pub use self::automation_campaign::*;
pub use self::contact::ContactType;
pub use self::api_root::*;
pub use self::list::*;

// ============ Empty Type ==============
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmptyType {}