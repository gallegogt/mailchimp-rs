mod error;
mod link;
mod authorized_apps;
mod automation_campaign;

pub use self::error::MailchimpErrorType;
pub use self::link::LinkType;
pub use self::authorized_apps::{AuthorizedAppType, AuthorizedAppsType, CreatedAuthorizedAppType};
pub use self::automation_campaign::*;