use crate::api::MailchimpApi;
use crate::internal::types::{
    CampaignType,  LinkType,  RecipientType, CampaignSettingsType, VariateSettingsType, CampaignTrackingOptionsType, RSSOptionsType, ABTestingOptionsType, SocialCardType, CampaignReportSummaryType, CampaignDeliveryStatusType
};

///
/// CampaignResource
///
#[derive(Debug, Clone)]
pub struct CampaignResource {
    api: MailchimpApi,
    id: String,
    inner_campaign: CampaignType,
}

impl CampaignResource {
    ///
    /// Argumentos:
    ///     api: MailchimpApi
    ///     campaign: CampaignType
    ///
    pub fn new(api: MailchimpApi, campaign: &CampaignType) -> Self {
        let campaign_id = campaign.id.as_ref();

        CampaignResource {
            api: api,
            id: campaign_id.unwrap().to_string(),
            inner_campaign: campaign.clone(),
        }
    }
    /// A string that uniquely identifies this campaign.
    pub fn id(&self) -> Option<&String> {
        self.inner_campaign.id.as_ref()
    }
    /// The ID used in the Mailchimp web application. View this campaign in
    /// your Mailchimp account at https://{dc}.admin.mailchimp.com/campaigns/show/?id={web_id}.
    pub fn web_id(&self) -> Option<&u64> {
        self.inner_campaign.web_id.as_ref()
    }
    /// If this campaign is the child of another campaign, this identifies the parent
    /// campaign. For Example, for RSS or Automation children.
    pub fn parent_campaign_id(&self) -> Option<&String> {
        self.inner_campaign.parent_campaign_id.as_ref()
    }
    /// There are four types of campaigns you can create in Mailchimp. A/B Split
    /// campaigns have been deprecated and variate campaigns should be used instead.
    pub fn campaign_type(&self) -> Option<&String> {
        self.inner_campaign.campaign_type.as_ref()
    }
    /// The date and time the campaign was created in ISO 8601 format.
    pub fn create_time(&self) -> Option<&String> {
        self.inner_campaign.create_time.as_ref()
    }
    /// The link to the campaign’s archive version in ISO 8601 format.
    pub fn archive_url(&self) -> Option<&String> {
        self.inner_campaign.archive_url.as_ref()
    }
    /// The original link to the campaign’s archive version.
    pub fn long_archive_url(&self) -> Option<&String> {
        self.inner_campaign.long_archive_url.as_ref()
    }
    /// The current status of the campaign.
    pub fn status(&self) -> Option<&String> {
        self.inner_campaign.status.as_ref()
    }
    /// The total number of emails sent for this campaign.
    pub fn emails_sent(&self) -> Option<&u64> {
        self.inner_campaign.emails_sent.as_ref()
    }
    /// The date and time a campaign was sent.
    pub fn send_time(&self) -> Option<&String> {
        self.inner_campaign.send_time.as_ref()
    }
    /// How the campaign’s content is put together (‘template’, ‘drag_and_drop’, ‘html’, ‘url’).
    pub fn content_type(&self) -> Option<&String> {
        self.inner_campaign.content_type.as_ref()
    }
    /// Determines if the campaign needs its blocks refreshed by opening the web-based campaign editor.
    pub fn needs_block_refresh(&self) -> Option<&bool> {
        self.inner_campaign.needs_block_refresh.as_ref()
    }
    /// Determines if the campaign contains the |BRAND:LOGO| merge tag.
    pub fn has_logo_merge_tag(&self) -> Option<&bool> {
        self.inner_campaign.has_logo_merge_tag.as_ref()
    }
    /// Determines if the campaign qualifies to be resent to non-openers.
    pub fn resendable(&self) -> Option<&bool> {
        self.inner_campaign.resendable.as_ref()
    }
    ///  List settings for the campaign.
    pub fn recipients(&self) -> Option<&RecipientType> {
        self.inner_campaign.recipients.as_ref()
    }
    /// The settings for your campaign, including subject, from name,
    /// reply-to address, and more.
    pub fn settings(&self) -> Option<&CampaignSettingsType> {
        self.inner_campaign.settings.as_ref()
    }
    /// The settings specific to A/B test campaigns.
    pub fn variate_settings(&self) -> Option<&VariateSettingsType> {
        self.inner_campaign.variate_settings.as_ref()
    }
    /// The tracking options for a campaign.
    pub fn tracking(&self) -> Option<&CampaignTrackingOptionsType> {
        self.inner_campaign.tracking.as_ref()
    }
    /// RSS options for a campaign.
    pub fn rss_opts(&self) -> Option<&RSSOptionsType> {
        self.inner_campaign.rss_opts.as_ref()
    }
    /// A/B Testing options for a campaign.
    pub fn ab_split_opts(&self) -> Option<&ABTestingOptionsType> {
        self.inner_campaign.ab_split_opts.as_ref()
    }
    /// The preview for the campaign, rendered by social networks like
    /// Facebook and Twitter. Learn more.
    pub fn social_card(&self) -> Option<&SocialCardType> {
        self.inner_campaign.social_card.as_ref()
    }
    /// For sent campaigns, a summary of opens, clicks, and e-commerce data.
    pub fn report_summary(&self) -> Option<&CampaignReportSummaryType> {
        self.inner_campaign.report_summary.as_ref()
    }
    /// Updates on campaigns in the process of sending.
    pub fn delivery_status(&self) -> Option<&CampaignDeliveryStatusType> {
        self.inner_campaign.delivery_status.as_ref()
    }
    /// Updates on campaigns in the process of sending.
    pub fn _links(&self) -> Option<&Vec<LinkType>> {
        self.inner_campaign._links.as_ref()
    }
}

///
/// Listados de Recursos de tipo Automatizacion
///
pub type CampaignResources = Vec<CampaignResource>;
