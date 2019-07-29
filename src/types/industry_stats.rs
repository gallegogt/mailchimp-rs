///
/// The average campaign statistics for your industry.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IndustryStatsType {
    /// The type of business industry associated with your account.
    /// For example: retail, education, etc.
    #[serde(default, rename = "type")]
    pub industry_stats_type: String,
    /// The industry open rate.
    #[serde(default)]
    pub open_rate: f32,
    /// The industry click rate.
    #[serde(default)]
    pub click_rate: f32,
    /// The industry bounce rate.
    #[serde(default)]
    pub bounce_rate: f32,
    /// The industry unopened rate.
    #[serde(default)]
    pub unopen_rate: f32,
    /// The industry unsubscribe rate.
    #[serde(default)]
    pub unsub_rate: f32,
    /// The industry abuse rate.
    #[serde(default)]
    pub abuse_rate: f32,
}

impl Default for IndustryStatsType {
    fn default() -> Self {
        Self {
            industry_stats_type: String::new(),
            open_rate: 0.0,
            click_rate: 0.0,
            bounce_rate: 0.0,
            unopen_rate: 0.0,
            unsub_rate: 0.0,
            abuse_rate: 0.0,
        }
    }
}
