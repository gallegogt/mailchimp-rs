///
/// Segment Conditions
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SegmentConditionsType {
    /// The type of segment, for example: date, language, Mandrill, static, and more.
    #[serde(default)]
    pub condition_type: String,
}

impl Default for SegmentConditionsType {
    fn default() -> Self {
        SegmentConditionsType {
            condition_type: "".to_string(),
        }
    }
}

///
/// Segment Options
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SegmentOptionsType {
    /// The id for an existing saved segment.
    #[serde(default)]
    pub saved_segment_id: u64,
    /// The prebuilt segment id, if a prebuilt segment has been designated for this campaign.
    #[serde(default)]
    pub prebuilt_segment_id: String,
    /// Desc: Segment match type.
    #[serde(default, rename = "match")]
    pub match_filter: String,
    /// An array of segment conditions.
    #[serde(default)]
    pub conditions: Vec<SegmentConditionsType>,
}

impl Default for SegmentOptionsType {
    fn default() -> Self {
        SegmentOptionsType {
            saved_segment_id: 0,
            prebuilt_segment_id: "".to_string(),
            match_filter: "".to_string(),
            conditions: Vec::new(),
        }
    }
}
