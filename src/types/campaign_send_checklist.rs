use super::link::LinkType;

///
/// A list of feedback items to review before sending your campaign.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChecklistItem {
    /// The item type. success warning error
    #[serde(default, rename="type")]
    pub item_type: String,
    /// The ID for the specific item.
    #[serde(default)]
    pub id: u64,
    /// The heading for the specific item.
    #[serde(default)]
    pub heading: String,
    /// Details about the specific feedback item.
    #[serde(default)]
    pub details: String,
}

impl Default for ChecklistItem {
    fn default() -> Self {
        ChecklistItem {
            item_type: "".to_string(),
            id: 0,
            heading: "".to_string(),
            details: "".to_string(),
        }
    }
}

///
/// Review the send checklist for your campaign, and resolve any issues before sending.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SendChecklistType {
    /// The id of the template to use.
    #[serde(default)]
    pub is_ready: bool,
    /// Content for the sections of the template. Each key should be
    /// the unique mc:edit area name from the template.
    #[serde(default)]
    pub items: Vec<ChecklistItem>,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}
