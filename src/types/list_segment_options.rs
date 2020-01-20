use serde::{Deserialize, Serialize};
use serde_json::Value;

///
/// Segment Conditions
///
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "condition_type")]
pub enum SegmentConditionsType {
    /// Aim Segment
    Aim(SegmentConditionInner),
    /// Automation Segment
    Automation(SegmentConditionInner),
    /// Poll Activity Segment
    CampaignPoll(SegmentConditionInner),
    /// Conversation Segment
    Conversation(SegmentConditionInner),
    /// Date Segment
    Date(SegmentConditionInner),
    /// Email Client Segment
    EmailClient(SegmentConditionInner),
    /// Language Segment
    Language(SegmentConditionInner),
    /// Member Rating Segment
    MemberRating(SegmentConditionInner),
    /// Signup Source Segment
    SignupSource(SegmentConditionInner),
    /// Survey Monkey Segment
    SurveyMonkey(SegmentConditionInner),
    /// VIP Segment
    VIP(SegmentConditionFlag),
    /// Interests Segment
    Interests(SegmentConditionInner),
    /// Ecommerce Category Segment
    EcommCategory(SegmentConditionInner),
    /// Ecommerce Number Segment
    EcommNumber(SegmentConditionInner),
    /// Ecommerce Purchased Segment
    EcommPurchased(SegmentConditionFlag),
    /// Ecommerce Spent Segment
    EcommSpent(SegmentConditionInner),
    /// Ecommerce Purchased Store Segment
    EcommStore(SegmentConditionInner),
    /// Goal Activity Segment
    GoalActivity(SegmentConditionInner),
    /// Goal Timestamp Segment
    GoalTimestamp(SegmentConditionInner),
    /// Similar Subscribers Segment Member Segment
    FuzzySegment(SegmentConditionInner),
    /// Static Segment Member Segment
    StaticSegment(SegmentConditionInner),
    /// Location-Based Segment
    IPGeoCountryState(SegmentConditionInner),
    /// Geolocation Segment
    IPGeoIn(SegmentGeoCondition),
    /// US Zip Code Segment
    IPGeoInZip(SegmentConditionInner),
    /// Unknown Location-Based Segment
    IPGeoUnknown(SegmentConditionFlag),
    /// Zip Code Location-Based Segment
    IPGeoZip(SegmentConditionInner),
    /// Social Profiles Age Segment
    SocialAge(SegmentConditionInner),
    /// Social Profiles Gender Segment
    SocialGender(SegmentConditionInner),
    /// Social Profiles Influence Segment
    SocialInfluence(SegmentConditionInner),
    /// Social Profiles Social Network Segment
    SocialNetworkMember(SegmentConditionInner),
    /// Social Profiles Social Network Follow Segment
    SocialNetworkFollow(SegmentConditionInner),
    /// Address Merge Field Segment
    AddressMerge(SegmentConditionInner),
    /// Address/Zip Merge Field Segment
    ZipMerge(SegmentConditionInner),
    /// Birthday Merge Field Segment
    BirthdayMerge(SegmentConditionInner),
    /// Date Merge Field Segment
    DateMerge(SegmentConditionInner),
    /// Dropdown/Radio Merge Field Segment
    SelectMerge(SegmentConditionInner),
    /// Text or Number Merge Field Segment
    TextMerge(SegmentConditionInner),
    /// Email Segment
    EmailAddress(SegmentConditionInner),
    /// Predicted Gender Segment
    PredictedGender(SegmentConditionInner),
    /// Predicted Age Segment
    PredictedAge(SegmentConditionInner),
    /// New Subscribers Prebuilt Segment
    NewSubsribers(SegmentConditionInner),
}

/// Segment condition without a value
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SegmentConditionFlag {
    /// Condition Field
    pub field: String,
    /// Condition Op
    pub op: SegmentConditionOp,
}

/// Segment condition with a value and optionally extra data
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SegmentConditionInner {
    /// field
    pub field: String,
    /// op
    pub op: SegmentConditionOp,
    /// value
    pub value: Value,
    /// Extra
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
}

/// Segment condition on a geo-spatial value
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SegmentGeoCondition {
    /// field
    pub field: String,
    /// Op
    pub op: SegmentConditionOp,
    /// Value
    pub value: Value,
    /// addr
    pub addr: String,
    /// lat
    pub lat: String,
    /// lng
    pub lng: String,
}

///
/// Segment Operator
///
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged, rename_all = "snake_case")]
pub enum SegmentConditionOp {
    // Aim conditions
    /// The campaign was opened by the subscriber.
    Open,
    /// The campaign was clicked by the subscriber.
    Click,
    /// The campaign was sent to the subscriber.
    Sent,
    /// The campaign was not opened by the subscriber.
    Noopen,
    /// The campaign was not clicked by the subscriber.
    Noclick,
    /// The campaign was not sent to the subscriber.
    Nosent,

    // Automation, SurveyMonkey conditions
    /// The member has started the automation workflow or survey.
    Started,
    /// The member has completed the automation workflow or survey.
    Completed,
    /// The member has not started the automation workflow or survey.
    NotStarted,
    /// The member has not completed the automation workflow or survey.
    NotCompleted,

    // CampaignPoll, Conversation, SocialNetwork, VIP conditions
    /// The subscriber is a member.
    Member,
    /// The subscriber is not a member.
    Notmember,

    // Date, etc. conditions
    /// The field is greater than the value.
    Greater,
    /// The field is less than the value.
    Less,
    /// The field is exactly the value.
    Is,
    /// The field is not equal to the value.
    Not,
    /// The field is blank.
    Blank,
    /// The field is not blank.
    BlankNot,
    /// The field is within the range.
    Within,
    /// The field is not within the range.
    Notwithin,

    // EmailClient conditions
    /// The email client is the value.
    ClientIs,
    /// The email client is not the value.
    ClientNot,

    // SignupSource conditions
    /// The signup source is the value.
    SourceIs,
    /// The signup source is not the value.
    SourceNot,

    // EcommCategory, AddressMerge conditions
    /// The field contains the value.
    Contains,
    /// The field does not contain the value.
    Notcontain,
    /// The field starts with the value.
    Starts,
    /// The field ends with the value.
    Ends,

    // Goal conditions
    /// The goal is not the value. (Inverse of `Is` for Goal Segments.)
    GoalNot,
    /// The goal does not contain the value. (Inverse of `Contains` for Goal Segments.)
    GoalNotcontain,

    // FuzzySegment conditions
    /// The subscriber is in the value's "similar subscribers" segment.
    FuzzyIs,
    /// The subscriber is not in the value's "similar subscribers" segment.
    FuzzyNot,

    // StaticSegment conditions
    /// The subscriber is has the value's tag.
    StaticIs,
    /// The subscriber does not have the value's tag.
    StaticNot,

    // IPGeoCountryState conditions
    /// The subscriber is in the value's country code.
    Ipgeocountry,
    /// The subscriber is not in the value's country code.
    Ipgeonotcountry,
    /// The subscriber is in the value's state code.
    Ipgeostate,
    /// The subscriber is not in the value's state code.
    Ipgeonotstate,

    // IPGeoIn conditions
    /// The subscriber is in a radius of {value} around {addr} or the {lat}/{lng} coordinates.
    Ipgeoin,
    /// The subscriber is not in a radius of {value} around {addr} or the {lat}/{lng} coordinates.
    Ipgeonotin,

    // IPGeoInZip condition
    /// The subscriber is in a radius of {value} around the ZIP code, {extra}.
    Ipgeoinzip,

    // IPGeoUnknown condition
    /// The subscriber's location is unknown.
    Ipgeounknown,

    // IPGeoZip conditions
    /// The subscriber's ZIP code is the value.
    Ipgeoiszip,
    /// The subscriber's ZIP code is not the value.
    Ipgeonotzip,

    // SocialNetworkFollow conditions
    /// The subscriber follows you on social media.
    Follow,
    /// The subscriber does not follow you on social media.
    Notfollow,

    // ZipMerge condition
    /// The subscriber's address is {value} distance from the {extra} city or ZIP code.
    Geoin,

    // NewSubscribers condition
    /// The subscriber joined within a given time period.
    DateWithin,
}

///
/// Segment Options
///
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(default)]
pub struct SegmentOptionsType {
    /// The id for an existing saved segment.
    pub saved_segment_id: u64,
    /// The prebuilt segment id, if a prebuilt segment has been designated for this campaign.
    pub prebuilt_segment_id: String,
    /// Desc: Segment match type.
    #[serde(rename = "match")]
    pub match_filter: String,
    /// An array of segment conditions.
    pub conditions: Vec<SegmentConditionsType>,
}
