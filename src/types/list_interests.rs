use super::empty::EmptyType;
use super::link::LinkType;
use crate::api::MailchimpApi;
use crate::internal::error_type::MailchimpErrorType;
use crate::internal::request::MailchimpResult;
use crate::iter::{BuildIter, MailchimpCollection, SimpleFilter};
use std::collections::HashMap;
use std::rc::Rc;

///
/// Interests
///
/// Manage interests for a specific Mailchimp list. Assign subscribers to interests
/// to group them together. Interests are referred to as ‘group names’ in the
/// Mailchimp application. Learn more about using groups in Mailchimp.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListInterest {
    /// The id for the interest category.
    #[serde(default)]
    pub category_id: String,
    /// The ID for the list that this interest belongs to.
    #[serde(default)]
    pub list_id: String,
    /// The ID for the interest.
    #[serde(default)]
    pub id: String,
    /// The name of the interest. This can be shown publicly on a subscription form.
    #[serde(default)]
    pub name: String,
    /// The number of subscribers associated with this interest.
    #[serde(default)]
    pub subscriber_count: String,
    /// The display order for interests.
    #[serde(default)]
    pub display_order: u64,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,

    /// Mailchimp API
    #[serde(skip)]
    _api: Rc<MailchimpApi>,
    /// Edpoint
    #[serde(default)]
    _endpoint: String,
}
///
/// Interest Param
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InterestParam {
    /// The text description of this category. This field appears on signup forms
    /// and is often phrased as a question.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The order that the categories are displayed in the list. Lower numbers display first.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_order: Option<u64>,
}

impl InterestParam {
    ///
    /// Arguments:
    ///     title: The text description of this category. This field appears on signup forms
    ///         and is often phrased as a question.
    ///     display_order: The order that the categories are displayed in the list.
    ///         Lower numbers display first.
    ///
    pub fn new<'a>(title: &'a str, display_order: Option<u64>) -> Self {
        InterestParam {
            title: Some(title.to_string()),
            display_order: display_order,
        }
    }
}

impl ListInterest {
    ///
    /// Update the api instance
    ///
    pub fn set_api(&mut self, n_api: Rc<MailchimpApi>) {
        self._api = n_api
    }
    ///
    /// Update the endpoint value
    ///
    pub fn set_endpoint<'a>(&mut self, n_endpoint: &'a str) {
        self._endpoint = n_endpoint.to_string();
    }

    ///
    /// Delete interests or group names in a specific category.
    ///
    pub fn delete(&self) -> Option<MailchimpErrorType> {
        // DELETE /lists/{list_id}/interest-categories/{interest_category_id}/interests/{interest_id}
        let endpoint = self.get_base_endpoint();
        match self._api.delete::<EmptyType>(&endpoint, HashMap::new()) {
            Ok(_) => None,
            Err(e) => Some(e),
        }
    }

    ///
    /// Update interests or ‘group names’ for a specific category.
    ///
    /// Arguments:
    ///     param: Values tu update
    ///
    pub fn update<'a>(&self, param: InterestParam) -> MailchimpResult<ListInterest> {
        // PATCH /lists/{list_id}/interest-categories/{interest_category_id}/interests/{interest_id}
        let endpoint = self.get_base_endpoint();
        self._api
            .patch::<ListInterest, InterestParam>(&endpoint, param)
    }

    ///
    /// Private function to build endpoint string
    ///
    fn get_base_endpoint(&self) -> String {
        // /lists/{list_id}/interest-categories/{interest_category_id}/interests/{interest_id}
        format!("{:?}/{:?}", self._endpoint, self.id)
    }
}

///
/// Response for endpoint
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionListInterest {
    /// An array of this category’s interests
    #[serde(default)]
    pub interests: Vec<ListInterest>,
    /// The unique list id that the interests belong to.
    #[serde(default)]
    pub list_id: String,
    /// The id for the interest category.
    #[serde(default)]
    pub category_id: String,
    /// The total number of items matching the query regardless of pagination.
    #[serde(default)]
    pub total_items: u64,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

impl MailchimpCollection<ListInterest> for CollectionListInterest {
    /// Total Items
    fn get_total_items(&self) -> u64 {
        self.total_items
    }

    /// Data
    fn get_values(&self) -> Vec<ListInterest> {
        self.interests.clone()
    }
}

impl Default for CollectionListInterest {
    fn default() -> Self {
        CollectionListInterest {
            interests: Vec::new(),
            list_id: "".to_string(),
            category_id: "".to_string(),
            total_items: 0,
            _links: Vec::new(),
        }
    }
}

/// ================================= ITER =====================

///
/// ListInterestBuilder
///
#[derive(Debug)]
pub struct ListInterestBuilder {
    /// Resource Endpoint
    pub endpoint: String,
}

impl BuildIter for ListInterestBuilder {
    type Item = ListInterest;
    type FilterItem = SimpleFilter;
    type Collection = CollectionListInterest;

    ///
    /// Crea un recurso a partir del dato pasado por parámetro
    ///
    fn update_item(&self, data: &Self::Item, api: Rc<MailchimpApi>) -> Self::Item {
        let mut in_data = data.clone();
        in_data.set_api(api);
        in_data.set_endpoint(&self.endpoint);
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
