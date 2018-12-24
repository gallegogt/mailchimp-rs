// ============ Contact ==============
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContactType {
    /// The company name for the account.
    #[serde(default)]
    pub company: String,
    /// Desc: The street address for the account contact.
    #[serde(default)]
    pub addr1: String,
    /// Desc: The street address for the account contact.
    #[serde(default)]
    pub addr2: String,
    /// Desc: The street address for the account contact.
    #[serde(default)]
    pub address1: String,
    /// Desc: The street address for the account contact.
    #[serde(default)]
    pub address2: String,
    /// Desc: The city for the account contact.
    #[serde(default)]
    pub city: String,
    /// Desc: The state for the account contact.
    #[serde(default)]
    pub state: String,
    /// Desc: The zip code for the account contact.
    #[serde(default)]
    pub zip: String,
    /// Desc: The country for the account contact.
    #[serde(default)]
    pub country: String,
    /// The phone number for the list contact.
    #[serde(default)]
    pub phone: String,
}

impl Default for ContactType {
    fn default() -> Self {
        ContactType {
            company: "".to_string(),
            addr1: "".to_string(),
            addr2: "".to_string(),
            address1: "".to_string(),
            address2: "".to_string(),
            city: "".to_string(),
            state: "".to_string(),
            zip: "".to_string(),
            country: "".to_string(),
            phone: "".to_string(),
        }
    }
}
