use super::link::LinkType;
use super::list_members::{ListMember, ListMemberParams};
use serde::{Deserialize, Serialize};

///
///  Batch subscribe or unsubscribe list members Response
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListBatchParam {
    /// An array of objects, each representing an email address and
    /// the subscription status for a specific list. Up to 500 members
    /// may be added or updated with each API call.
    #[serde(default)]
    pub members: Vec<ListMemberParams>,
    ///
    /// Whether this batch operation will change existing members’ subscription status.
    ///
    #[serde(default)]
    pub update_existing: bool,
}

///
/// Representing an email address that could not be added to the list or updated
/// and an error message providing more details.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListBatchErrors {
    ///
    /// The email address that could not be added or updated.
    ///
    #[serde(default)]
    pub email_address: String,
    ///
    /// The error message indicating why the email address could not be added or updated.
    ///
    #[serde(default)]
    pub error: String,
}

///
///  Batch subscribe or unsubscribe list members Response
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListBatchResponse {
    ///
    /// An array of objects, each representing a new member
    /// that was added to the list.
    ///
    #[serde(default)]
    pub new_members: Vec<ListMember>,
    ///
    /// An array of objects, each representing an existing list member
    /// whose subscription status was updated.
    ///
    #[serde(default)]
    pub updated_members: Vec<ListMember>,
    ///
    /// An array of objects, each representing an email address that
    /// could not be added to the list or updated and an error message
    /// providing more details.
    ///
    #[serde(default)]
    pub errors: Vec<ListBatchErrors>,
    ///
    /// The total number of items matching the query, irrespective of pagination.
    ///
    #[serde(default)]
    pub total_created: u64,
    ///
    /// The total number of items matching the query, irrespective of pagination.
    ///
    #[serde(default)]
    pub total_updated: u64,
    ///
    /// The total number of items matching the query, irrespective of pagination.
    ///
    #[serde(default)]
    pub error_count: u64,
    ///
    /// A list of link types and descriptions for the API schema documents.
    ///
    #[serde(default)]
    pub _links: Vec<LinkType>,
}
