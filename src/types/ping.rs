///
/// A health check for the API that won’t return any account-specific information.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ping {
    /// This will return a constant string value if the request is successful.
    /// Ex. “Everything’s Chimpy!”
    #[serde(default)]
    health_status: String,
}
