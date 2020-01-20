//! Implement Campaign Content Types
//!

use super::link::LinkType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

///
///  Use this template to generate the HTML content for the campaign.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TemplateContent {
    /// The id of the template to use.
    #[serde(default)]
    pub id: String,
    /// Content for the sections of the template. Each key should be
    /// the unique mc:edit area name from the template.
    #[serde(default)]
    pub sections: HashMap<String, String>,
}

impl Default for TemplateContent {
    fn default() -> Self {
        TemplateContent {
            id: "".to_string(),
            sections: HashMap::new(),
        }
    }
}

///
///  Use this template to generate the HTML content for the campaign.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UploadArchive {
    /// The base64-encoded representation of the archive file.
    #[serde(default)]
    pub archive_content: String,
    /// The type of encoded file. Defaults to zip.
    /// Possible Values: zip tar.gz tar.bz2 tar tgz tbz
    #[serde(default)]
    pub archive_type: String,
}

impl Default for UploadArchive {
    fn default() -> Self {
        UploadArchive {
            archive_content: "".to_string(),
            archive_type: "".to_string(),
        }
    }
}

///
/// Variate Content
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VariateContent {
    /// Label used to identify the content option.
    #[serde(default)]
    pub content_label: String,
    /// The plain-text portion of the campaign. If left unspecified,
    /// we’ll generate this automatically.
    #[serde(default)]
    pub plain_text: String,
    /// The raw HTML for the campaign.
    #[serde(default)]
    pub html: String,
    /// When importing a campaign, the URL for the HTML.
    #[serde(default)]
    pub url: String,
    /// Use this template to generate the HTML content for the campaign.
    #[serde(default)]
    pub template: TemplateContent,
    /// Available when uploading an archive to create campaign content.
    /// The archive should include all campaign content and images. Learn more.
    #[serde(default)]
    pub archive: UploadArchive,
}

impl Default for VariateContent {
    fn default() -> Self {
        VariateContent {
            content_label: String::new(),
            plain_text: String::new(),
            html: String::new(),
            url: "".to_string(),
            template: TemplateContent::default(),
            archive: UploadArchive::default(),
        }
    }
}

///
/// Campaign Content Type
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CampaignContentType {
    /// Content options for multivariate campaigns.
    #[serde(default)]
    pub variate_contents: Vec<VariateContent>,
    /// The plain-text portion of the campaign. If left unspecified, we’ll generate this automatically.
    #[serde(default)]
    pub plain_text: String,
    /// The raw HTML for the campaign.
    #[serde(default)]
    pub html: String,
    /// The Archive HTML for the campaign.
    #[serde(default)]
    pub archive_html: String,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

///
/// Campaign Content Param
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CampaignContentParam {
    /// The plain-text portion of the campaign. If left unspecified, we’ll generate this automatically.
    #[serde(default)]
    pub plain_text: String,
    /// The raw HTML for the campaign.
    #[serde(default)]
    pub html: String,
    /// When importing a campaign, the URL where the HTML lives.
    #[serde(default)]
    pub url: String,
    /// Use this template to generate the HTML content of the campaign
    #[serde(default)]
    pub template: TemplateContent,
    /// Available when uploading an archive to create campaign content.
    /// The archive should include all campaign content and images. Learn more.
    #[serde(default)]
    pub archive: UploadArchive,
    /// Content options for Multivariate Campaigns. Each content option must
    /// provide HTML content and may optionally provide plain text. For campaigns
    /// not testing content, only one object should be provided.
    #[serde(default)]
    pub variate_contents: Vec<VariateContent>,
}
