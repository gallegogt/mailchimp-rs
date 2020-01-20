use super::link::LinkType;
use crate::api::MailchimpApi;
use crate::iter::{BuildIter, MailchimpCollection, ResourceFilter};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

///
/// List Growth History Type
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListGrowthHistoryType {
    /// The list id for the growth activity report.
    #[serde(default)]
    pub list_id: String,
    /// The month that the growth history is describing.
    #[serde(default)]
    pub month: String,
    /// Existing members on the list for a specific month.
    #[serde(default)]
    pub existing: u64,
    /// Imported members on the list for a specific month.
    #[serde(default)]
    pub imports: u64,
    /// Newly opted-in members on the list for a specific month.
    #[serde(default)]
    pub optins: u64,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

///
/// Get a month-by-month summary of a specific list’s growth activity.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionListGrowthHistory {
    /// The list id for the abuse report.
    #[serde(default)]
    pub list_id: String,
    /// An array of objects, each representing an abuse report resource.
    #[serde(default)]
    pub history: Vec<ListGrowthHistoryType>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(default)]
    pub total_items: u64,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

impl MailchimpCollection<ListGrowthHistoryType> for CollectionListGrowthHistory {
    /// Total Items
    fn get_total_items(&self) -> u64 {
        self.total_items
    }

    /// Data
    fn get_values(&self) -> Vec<ListGrowthHistoryType> {
        self.history.clone()
    }
}

impl Default for CollectionListGrowthHistory {
    fn default() -> Self {
        CollectionListGrowthHistory {
            list_id: String::new(),
            history: Vec::new(),
            total_items: 0,
            _links: Vec::new(),
        }
    }
}

///
/// List Growth History Filter
///
#[derive(Debug, Clone)]
pub struct ListGrowthHistoryFilter {
    /// A comma-separated list of fields to return. Reference
    /// parameters of sub-objects with dot notation.
    pub fields: Option<String>,
    /// A comma-separated list of fields to exclude. Reference
    /// parameters of sub-objects with dot notation.
    pub exclude_fields: Option<String>,
    /// The number of records to return. Default value is 10.
    pub count: Option<u64>,
    /// The number of records from a collection to skip. Iterating over
    /// large collections with this parameter can be slow. Default value is 0..
    pub offset: Option<u64>,
    /// Returns files sorted by the specified field. Posible Values month
    pub sort_field: Option<String>,
    /// Determines the order direction for sorted results. Possible Values: ASC/DESC
    pub sort_dir: Option<String>,
}

impl Default for ListGrowthHistoryFilter {
    fn default() -> Self {
        ListGrowthHistoryFilter {
            fields: None,
            exclude_fields: None,
            count: Some(50),
            offset: Some(0),
            sort_field: None,
            sort_dir: None,
        }
    }
}

impl ResourceFilter for ListGrowthHistoryFilter {
    fn build_payload(&self) -> HashMap<String, String> {
        let mut payload = HashMap::new();

        if self.fields.is_some() {
            payload.insert("fields".to_string(), self.fields.as_ref().unwrap().clone());
        }
        if self.exclude_fields.is_some() {
            payload.insert(
                "exclude_fields".to_string(),
                self.exclude_fields.as_ref().unwrap().clone(),
            );
        }
        if self.count.is_some() {
            payload.insert(
                "count".to_string(),
                format!("{:}", self.count.as_ref().unwrap().clone()),
            );
        }
        if self.offset.is_some() {
            payload.insert(
                "offset".to_string(),
                format!("{:}", self.offset.as_ref().unwrap().clone()),
            );
        }
        if self.sort_field.is_some() {
            payload.insert(
                "sort_field".to_string(),
                format!("{:}", self.sort_field.as_ref().unwrap().clone()),
            );
        }
        if self.sort_dir.is_some() {
            payload.insert(
                "sort_dir".to_string(),
                format!("{:}", self.sort_dir.as_ref().unwrap().clone()),
            );
        }
        payload
    }
}

///
/// List Growth History Builder
///
#[derive(Debug)]
pub struct ListGrowthHistoryBuilder {}

impl BuildIter for ListGrowthHistoryBuilder {
    type Item = ListGrowthHistoryType;
    type FilterItem = ListGrowthHistoryFilter;
    type Collection = CollectionListGrowthHistory;

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
