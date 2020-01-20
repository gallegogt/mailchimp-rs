use super::link::LinkType;
use crate::api::MailchimpApi;
use crate::iter::{BuildIter, MailchimpCollection, SimpleFilter};
use serde::{Deserialize, Serialize};
use std::rc::Rc;

// ============ List Locations ==============
///
/// Get the locations (countries) that the list’s subscribers have been tagged to based
/// on geocoding their IP address.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListLocationType {
    /// The name of the country.
    #[serde(default)]
    pub country: String,
    /// The ISO 3166 2 digit country code.
    #[serde(default)]
    pub cc: String,
    /// The percent of subscribers in the country.
    #[serde(default)]
    pub percent: f32,
    /// The total number of subscribers in the country.
    #[serde(default)]
    pub total: u64,
}

///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionListLocations {
    /// Recent list activity.
    #[serde(default)]
    pub locations: Vec<ListLocationType>,
    /// The unique id for the list.
    #[serde(default)]
    pub list_id: String,
    /// The total number of items matching the query regardless of pagination.
    #[serde(default)]
    pub total_items: u64,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

impl MailchimpCollection<ListLocationType> for CollectionListLocations {
    /// Total Items
    fn get_total_items(&self) -> u64 {
        self.total_items
    }

    /// Data
    fn get_values(&self) -> Vec<ListLocationType> {
        self.locations.clone()
    }
}

impl Default for CollectionListLocations {
    fn default() -> Self {
        CollectionListLocations {
            list_id: String::new(),
            locations: Vec::new(),
            total_items: 0,
            _links: Vec::new(),
        }
    }
}

/// ================================= ITER =====================

#[derive(Debug)]
pub struct ListLocationsBuilder {}

impl BuildIter for ListLocationsBuilder {
    type Item = ListLocationType;
    type FilterItem = SimpleFilter;
    type Collection = CollectionListLocations;

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
