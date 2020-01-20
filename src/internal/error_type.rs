use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

/// ============ Error Response ==============
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MailchimpErrorType {
    /// Desc: An absolute URI that identifies the problem type. When dereferenced, it should provide human-readable documentation for the problem type.
    #[serde(rename = "type")]
    pub error_type: String,
    /// Desc: A short, human-readable summary of the problem type. It shouldn’t change based on the occurrence of the problem, except for purposes of localization.
    pub title: String,
    /// Desc: The HTTP status code (RFC2616, Section 6) generated by the origin server for this occurrence of the problem.
    pub status: u64,
    /// Desc: A human-readable explanation specific to this occurrence of the problem. Learn more about errors.
    pub detail: String,
    /// Desc: A string that identifies this specific occurrence of the problem. Please provide this ID when contacting support.
    pub instance: String,
}

impl Default for MailchimpErrorType {
    fn default() -> Self {
        MailchimpErrorType {
            error_type:
                "http://developer.mailchimp.com/documentation/mailchimp/guides/error-glossary/"
                    .to_string(),
            title: "Resource Not Found".to_string(),
            status: 404,
            detail: "could not find resource for requested class_path:AuthorizedAppType_Collection"
                .to_string(),
            instance: "3345556-334-434-3444-34343f4f4434f".to_string(),
        }
    }
}

impl fmt::Display for MailchimpErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "HTTP {} {}: \"{}\" {} ({})",
            self.status, self.title, self.detail, self.instance, self.error_type
        )
    }
}

impl Error for MailchimpErrorType {}
