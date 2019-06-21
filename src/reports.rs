use super::api::MailchimpApi;
use super::internal::request::MailchimpResult;
use super::types::{CollectionReports, ReportType, ReportsBuilder, ReportsFilter};
use crate::iter::{MalchimpIter, ResourceFilter};
use log::error;
use std::collections::HashMap;

///
/// Reports
///
/// Manage campaign reports for your Mailchimp account. All Reports
/// endpoints are read-only. Mailchimp’s campaign and Automation reports analyze
/// clicks, opens, subscribers’ social activity, e-commerce data, and more. Learn
/// more about campaign reports. Note: Campaign IDs for A/B Testing Campaigns are
/// available through the Campaign API Endpoint’s Read method
///
#[derive(Debug, Clone)]
pub struct Reports {
    api: MailchimpApi,
}

impl Reports {
    ///
    /// Arguments:
    ///     api: MailchimpApi
    ///
    pub fn new(api: MailchimpApi) -> Self {
        Reports { api: api }
    }

    ///
    /// Get campaign reports
    ///
    pub fn iter_reports(&self, filter: Option<ReportsFilter>) -> MalchimpIter<ReportsBuilder> {
        // GET /reports
        let endpoint = "reports";
        let mut filter_params = ReportsFilter::default();

        if let Some(f) = filter {
            filter_params = f;
        }

        match self
            .api
            .get::<CollectionReports>(&endpoint, filter_params.build_payload())
        {
            Ok(collection) => MalchimpIter {
                builder: ReportsBuilder {},
                data: collection.reports,
                cur_filters: filter_params.clone(),
                cur_it: 0,
                total_items: collection.total_items,
                api: self.api.clone(),
                endpoint: endpoint.to_string(),
            },
            Err(e) => {
                error!( target: "mailchimp",  "Get Reports: Response Error details: {:?}", e);
                MalchimpIter {
                    builder: ReportsBuilder {},
                    data: Vec::new(),
                    cur_filters: filter_params.clone(),
                    cur_it: 0,
                    total_items: 0,
                    api: self.api.clone(),
                    endpoint: endpoint.to_string(),
                }
            }
        }
    }

    ///
    /// Get a specific campaign report
    ///
    /// Get report details for a specific sent campaign.
    ///
    pub fn get_campaign_report<'a>(&self, campaign_id: &'a str) -> MailchimpResult<ReportType> {
        let endpoint = format!("reports/{}", campaign_id);
        let mut payload = HashMap::new();
        payload.insert("campaign_id".to_string(), campaign_id.to_string());
        self.api.get::<ReportType>(&endpoint, payload)
    }
}
