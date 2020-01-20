//! Implement API Root Model Types

use super::contact::ContactType;
use super::industry_stats::IndustryStatsType;
use super::link::LinkType;
use serde::{Deserialize, Serialize};

///
/// API Root Type (GET /)
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiRootType {
    /// The Mailchimp account ID, used for features like list subscribe forms.
    #[serde(default)]
    pub account_id: String,
    /// The ID associated with the user who owns this API key. If you can
    ///     login to multiple accounts, this ID will be the same for each account.
    #[serde(default)]
    pub login_id: String,
    /// The name of the account.
    #[serde(default)]
    pub account_name: String,
    /// The account email address.
    #[serde(default)]
    pub email: String,
    /// The first name tied to the account.
    #[serde(default)]
    pub first_name: String,
    /// The last name tied to the account.
    #[serde(default)]
    pub last_name: String,
    /// The username tied to the account.
    #[serde(default)]
    pub username: String,
    /// URL of the avatar for the user.
    #[serde(default)]
    pub avatar_url: String,
    /// The user role for the account.
    #[serde(default)]
    pub role: String,
    /// The date and time that the account was created in ISO 8601 format.
    #[serde(default)]
    pub member_since: String,
    /// The type of pricing plan the account is on.
    /// Possible Values:
    ///     monthly pay_as_you_go forever_free
    #[serde(default)]
    pub pricing_plan_type: String,
    /// Date of first payment for monthly plans.
    #[serde(default)]
    pub first_payment: String,
    /// The timezone currently set for the account.
    #[serde(default)]
    pub account_timezone: String,
    /// The user-specified industry associated with the account.
    #[serde(default)]
    pub account_industry: String,
    /// Information about the account contact.
    #[serde(default)]
    pub contact: ContactType,
    /// Whether the account includes Mailchimp Pro.
    #[serde(default)]
    pub pro_enabled: bool,
    /// The date and time of the last login for this account in ISO 8601 format.
    #[serde(default)]
    pub last_login: String,
    /// The total number of subscribers across all lists in the account.
    #[serde(default)]
    pub total_subscribers: u64,
    /// The average campaign statistics for all campaigns in the account’s specified industry.
    #[serde(default)]
    pub industry_stats: IndustryStatsType,
    /// The average campaign statistics for all campaigns in the account’s specified industry.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}
