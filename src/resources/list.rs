use crate::api::MailchimpApi;
use crate::internal::types::{
    CampaignDefaultsType, ContactType, LinkType, ListType, StatisticsType,
};

///
/// ListResource
///
#[derive(Debug, Clone)]
pub struct ListResource {
    api: MailchimpApi,
    id: String,
    inner_list: ListType,
}

impl ListResource {
    ///
    /// Argumentos:
    ///     api: MailchimpApi
    ///     list: ListType
    ///
    pub fn new(api: MailchimpApi, list: &ListType) -> Self {
        let list_id = list.id.as_ref();

        ListResource {
            api: api,
            id: list_id.unwrap().to_string(),
            inner_list: list.clone(),
        }
    }
    ///
    /// A string that uniquely identifies this list.
    ///
    pub fn id(&self) -> Option<&String> {
        self.inner_list.id.as_ref()
    }

    ///
    /// The ID used in the Mailchimp web application. View this list in
    /// your Mailchimp account at https://{dc}.admin.mailchimp.com/lists/members/?id={web_id}.
    ///
    pub fn web_id(&self) -> Option<&u64> {
        self.inner_list.web_id.as_ref()
    }
    ///
    /// The name of the list.
    ///
    pub fn name(&self) -> Option<&String> {
        self.inner_list.name.as_ref()
    }
    ///
    /// Contact information displayed in campaign footers to comply with international spam laws.
    ///
    pub fn contact(&self) -> Option<&ContactType> {
        self.inner_list.contact.as_ref()
    }
    ///
    /// The permission reminder for the list.
    ///
    pub fn permission_reminder(&self) -> Option<&String> {
        self.inner_list.permission_reminder.as_ref()
    }
    ///
    /// Whether campaigns for this list use the Archive Bar in archives by default.
    ///
    pub fn use_archive_bar(&self) -> Option<&bool> {
        self.inner_list.use_archive_bar.as_ref()
    }
    ///
    /// Default values for campaigns created for this list.
    ///
    pub fn campaign_defaults(&self) -> Option<&CampaignDefaultsType> {
        self.inner_list.campaign_defaults.as_ref()
    }
    ///
    /// The email address to send subscribe notifications to.
    ///
    pub fn notify_on_subscribe(&self) -> Option<&String> {
        self.inner_list.notify_on_subscribe.as_ref()
    }
    ///
    /// The email address to send unsubscribe notifications to.
    ///
    pub fn notify_on_unsubscribe(&self) -> Option<&String> {
        self.inner_list.notify_on_unsubscribe.as_ref()
    }
    ///
    /// The date and time that this list was created in ISO 8601 format.
    ///
    pub fn date_created(&self) -> Option<&String> {
        self.inner_list.date_created.as_ref()
    }
    ///
    /// An auto-generated activity score for the list (0-5).
    ///
    pub fn list_rating(&self) -> Option<&u64> {
        self.inner_list.list_rating.as_ref()
    }
    ///
    /// Whether the list supports multiple formats for emails
    ///
    pub fn email_type_option(&self) -> Option<&bool> {
        self.inner_list.email_type_option.as_ref()
    }
    ///
    /// Our EepURL shortened version of this list’s subscribe form.
    ///
    pub fn subscribe_url_short(&self) -> Option<&String> {
        self.inner_list.subscribe_url_short.as_ref()
    }
    ///
    /// The full version of this list’s subscribe form (host will vary).
    ///
    pub fn subscribe_url_long(&self) -> Option<&String> {
        self.inner_list.subscribe_url_long.as_ref()
    }
    ///
    /// The list’s Email Beamer address.
    ///
    pub fn beamer_address(&self) -> Option<&String> {
        self.inner_list.beamer_address.as_ref()
    }
    ///
    /// Whether this list is public or private.
    ///
    pub fn visibility(&self) -> Option<&String> {
        self.inner_list.visibility.as_ref()
    }
    ///
    /// Whether or not to require the subscriber to confirm subscription via email.
    ///
    pub fn double_optin(&self) -> Option<&bool> {
        self.inner_list.double_optin.as_ref()
    }
    ///
    /// Whether or not this list has a welcome automation connected.
    ///
    pub fn has_welcome(&self) -> Option<&bool> {
        self.inner_list.has_welcome.as_ref()
    }
    ///
    /// Whether or not the list has marketing permissions (eg. GDPR) enabled.
    ///
    pub fn marketing_permissions(&self) -> Option<&bool> {
        self.inner_list.marketing_permissions.as_ref()
    }
    ///
    /// Whether or not the list has marketing permissions (eg. GDPR) enabled.
    ///
    pub fn modules(&self) -> Option<&Vec<String>> {
        self.inner_list.modules.as_ref()
    }
    ///
    ///  Stats for the list. Many of these are cached for at least five minutes.
    ///
    pub fn stats(&self) -> Option<&StatisticsType> {
        self.inner_list.stats.as_ref()
    }
    ///
    ///  A list of link types and descriptions for the API schema documents.
    ///
    pub fn _links(&self) -> Option<&Vec<LinkType>> {
        self.inner_list._links.as_ref()
    }
}

///
/// Listados de Recursos de tipo Automatizacion
///
pub type ListResources = Vec<ListResource>;
