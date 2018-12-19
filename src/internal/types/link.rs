// ============ LinkType ==============
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinkType {
    /// Desc: As with an HTML ‘rel’ attribute, this describes the type of link.
    #[serde(default)]
    pub rel: String,
    /// Desc: This property contains a fully-qualified URL that can be called to retrieve the linked resource or perform the linked action.
     #[serde(default)]
    pub href: String,
    /// Desc: The HTTP method that should be used when accessing the URL defined in ‘href’.
     #[serde(default)]
    pub method: String,
    /// Desc: For GETs, this is a URL representing the schema that the response should conform to.
    #[serde(rename = "targetSchema")]
    pub target_schema: Option<String>,
    /// Desc: For HTTP methods that can receive bodies (POST and PUT), this is a URL representing the schema that the body should conform to.
     #[serde(default)]
    pub schema: String,
}

impl Default for LinkType {
    fn default() -> Self {
        LinkType {
            rel: "".to_string(),
            href: "".to_string(),
            method: "".to_string(),
            target_schema: None,
            schema: "".to_string(),
        }
    }
}
