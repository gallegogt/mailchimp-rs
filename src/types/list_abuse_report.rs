use super::link::LinkType;
use crate::api::MailchimpApi;
use crate::iter::{BuildIter, MailchimpCollection, SimpleFilter};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

///
/// Abuse Reports
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListAbuseReportType {
    /// The id for the abuse report
    #[serde(default)]
    pub id: u64,
    /// The campaign id for the abuse report
    #[serde(default)]
    pub campaign_id: String,
    /// The list id for the abuse report.
    #[serde(default)]
    pub list_id: String,
    /// The MD5 hash of the lowercase version of the list member’s email address.
    #[serde(default)]
    pub email_id: String,
    /// Email address for a subscriber.
    #[serde(default)]
    pub email_address: String,
    /// An individual merge var and value for a member.
    #[serde(default)]
    pub merge_fields: HashMap<String, String>,
    /// VIP status for subscriber.
    #[serde(default)]
    pub vip: bool,
    /// Date for the abuse report
    #[serde(default)]
    pub date: String,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

///
/// Get all abuse reports for a specific list.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionListAbuseReport {
    /// The list id for the abuse report.
    #[serde(default)]
    pub list_id: String,
    /// An array of objects, each representing an abuse report resource.
    #[serde(default)]
    pub abuse_reports: Vec<ListAbuseReportType>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(default)]
    pub total_items: u64,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

impl MailchimpCollection<ListAbuseReportType> for CollectionListAbuseReport {
    /// Total Items
    fn get_total_items(&self) -> u64 {
        self.total_items
    }

    /// Data
    fn get_values(&self) -> Vec<ListAbuseReportType> {
        self.abuse_reports.clone()
    }
}

impl Default for CollectionListAbuseReport {
    fn default() -> Self {
        CollectionListAbuseReport {
            list_id: String::new(),
            abuse_reports: Vec::new(),
            total_items: 0,
            _links: Vec::new(),
        }
    }
}

///
/// List AbuseReport Builder
///
#[derive(Debug)]
pub struct ListAbuseReportBuilder {}

impl BuildIter for ListAbuseReportBuilder {
    type Item = ListAbuseReportType;
    type FilterItem = SimpleFilter;
    type Collection = CollectionListAbuseReport;

    ///
    /// Crea un recurso a partir del dato pasado por parámetro
    ///
    fn update_item(&self, data: &Self::Item, _: Rc<MailchimpApi>) -> Self::Item {
        let in_data = data.clone();
        in_data
    }
    ///
    /// Actualiza el offset
    ///
    fn update_filter_offset(&self, filter: &Self::FilterItem) -> Self::FilterItem {
        let mut f = filter.clone();
        f.offset = Some(f.count.unwrap() + f.offset.unwrap());
        f
    }
}
