use super::empty::EmptyType;
use super::link::LinkType;
use super::list_interests::{
    CollectionListInterest, InterestParam, ListInterest, ListInterestBuilder,
};
use crate::api::MailchimpApi;
use crate::internal::error_type::MailchimpErrorType;
use crate::internal::request::MailchimpResult;
use crate::iter::{BuildIter, MailchimpCollection, MalchimpIter, ResourceFilter, SimpleFilter};
use log::error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

///
/// Interest Categories
///
/// Manage interest categories for a specific list. Interest
/// categories organize interests, which are used to group subscribers based on their
/// preferences. These correspond to ‘group titles’ in the Mailchimp application. Learn more
/// about groups in Mailchimp.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListInterestCategory {
    /// The unique list id for the category.
    #[serde(default)]
    pub list_id: String,
    /// The id for the interest category.
    #[serde(default)]
    pub id: String,
    /// The text description of this category. This field appears on signup forms
    /// and is often phrased as a question.
    #[serde(default)]
    pub title: String,
    /// The order that the categories are displayed in the list. Lower numbers display first.
    #[serde(default)]
    pub display_order: u64,
    /// Determines how this category’s interests appear on signup forms.
    /// Possible Values: checkboxes dropdown radio hidden
    #[serde(default, rename = "type")]
    pub ic_type: String,
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
/// Interest Category Param
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InterestCategoryParam {
    /// The text description of this category. This field appears on signup forms
    /// and is often phrased as a question.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The order that the categories are displayed in the list. Lower numbers display first.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_order: Option<u64>,
    /// Determines how this category’s interests appear on signup forms.
    /// Possible Values: checkboxes dropdown radio hidden
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub ic_type: Option<String>,
}

impl InterestCategoryParam {
    ///
    /// Arguments:
    ///     title: The text description of this category. This field appears on signup forms
    ///         and is often phrased as a question.
    ///     ic_type: Determines how this category’s interests appear on signup forms.
    ///         Possible Values: checkboxes dropdown radio hidden
    ///     display_order: The order that the categories are displayed in the list.
    ///         Lower numbers display first.
    ///
    pub fn new<'a>(title: &'a str, ic_type: &'a str, display_order: Option<u64>) -> Self {
        InterestCategoryParam {
            title: Some(title.to_string()),
            display_order: display_order,
            ic_type: Some(ic_type.to_string()),
        }
    }
}

impl ListInterestCategory {
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
    /// Delete a specific note for a specific list member.
    ///
    pub fn delete(&self) -> Option<MailchimpErrorType> {
        // DELETE /lists/{list_id}/interest-categories/{interest_category_id}
        let endpoint = self.get_base_endpoint();
        match self._api.delete::<EmptyType>(&endpoint, HashMap::new()) {
            Ok(_) => None,
            Err(e) => Some(e),
        }
    }

    ///
    /// Update a specific note for a specific list member.
    ///
    /// Arguments:
    ///     param: Values tu update
    ///
    pub fn update<'a>(
        &self,
        param: InterestCategoryParam,
    ) -> MailchimpResult<ListInterestCategory> {
        // PATCH /lists/{list_id}/interest-categories/{interest_category_id}
        let endpoint = self.get_base_endpoint();
        self._api
            .patch::<ListInterestCategory, InterestCategoryParam>(&endpoint, param)
    }

    ///
    /// Get a list of this category’s interests.
    ///
    pub fn get_interests(
        &self,
        filters: Option<SimpleFilter>,
    ) -> MalchimpIter<ListInterestBuilder> {
        // GET /lists/{list_id}/interest-categories/{interest_category_id}/interests
        let mut endpoint = self.get_base_endpoint();
        endpoint.push_str("/interests");

        let filter_params = if let Some(f) = filters {
            f
        } else {
            SimpleFilter::default()
        };

        match self
            ._api
            .get::<CollectionListInterest>(&endpoint, filter_params.build_payload())
        {
            Ok(collection) => MalchimpIter {
                builder: ListInterestBuilder {
                    endpoint: endpoint.clone(),
                },
                data: collection.interests,
                cur_filters: filter_params.clone(),
                cur_it: 0,
                total_items: collection.total_items,
                api: self._api.clone(),
                endpoint: endpoint.clone(),
            },
            Err(e) => {
                error!( target: "mailchimp",  "Get List Members: Response Error details: {:?}", e);
                MalchimpIter {
                    builder: ListInterestBuilder {
                        endpoint: endpoint.clone(),
                    },
                    data: Vec::new(),
                    cur_filters: filter_params.clone(),
                    cur_it: 0,
                    total_items: 0,
                    api: self._api.clone(),
                    endpoint: endpoint.clone(),
                }
            }
        }
    }

    ///
    /// Create a new interest in a specific category
    ///
    /// Argument:
    ///     note: The content of the note. Note length is limited to 1,000 characters.
    ///
    pub fn create_interest<'a>(&self, param: InterestParam) -> MailchimpResult<ListInterest> {
        // GET /lists/{list_id}/interest-categories/{interest_category_id}/interests/{interest_id}
        let mut endpoint = self.get_base_endpoint();
        endpoint.push_str("/interest-categories");
        self._api
            .post::<ListInterest, InterestParam>(&endpoint, param)
    }

    ///
    /// Get interests or ‘group names’ for a specific category.
    ///
    /// Argument:
    ///     interest_id: The specific interest or ‘group name’.
    ///
    pub fn get_specific_interest<'a>(
        &self,
        interest_id: &'a str,
    ) -> MailchimpResult<ListInterestCategory> {
        // GET /lists/{list_id}/interest-categories/{interest_category_id}/interests/{interest_id}
        let mut endpoint = self.get_base_endpoint();
        endpoint.push_str("/interests/");
        endpoint.push_str(interest_id);

        self._api
            .get::<ListInterestCategory>(&endpoint, HashMap::new())
    }

    ///
    /// Private function to build endpoint string
    ///
    fn get_base_endpoint(&self) -> String {
        // /lists/{list_id}/interest-categories/{interest_category_id}
        let mut endpoint = self._endpoint.clone();
        endpoint.push_str("/");
        endpoint.push_str(self.id.as_str());
        endpoint
    }
}

///
/// Response for endpoint
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionListInterestCategories {
    /// This array contains individual interest categories.
    #[serde(default)]
    pub categories: Vec<ListInterestCategory>,
    /// The ID for the list that this category belongs to.
    #[serde(default)]
    pub list_id: String,
    /// The list id.
    #[serde(default)]
    pub total_items: u64,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

impl MailchimpCollection<ListInterestCategory> for CollectionListInterestCategories {
    /// Total Items
    fn get_total_items(&self) -> u64 {
        self.total_items
    }

    /// Data
    fn get_values(&self) -> Vec<ListInterestCategory> {
        self.categories.clone()
    }
}

impl Default for CollectionListInterestCategories {
    fn default() -> Self {
        CollectionListInterestCategories {
            list_id: "".to_string(),
            categories: Vec::new(),
            total_items: 0,
            _links: Vec::new(),
        }
    }
}

/// ================================= ITER =====================

#[derive(Debug, Clone)]
pub struct ListInterestCategoryFilter {
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
    /// Restrict results a type of interest group
    pub ic_type: Option<String>,
}

impl Default for ListInterestCategoryFilter {
    fn default() -> Self {
        ListInterestCategoryFilter {
            fields: None,
            exclude_fields: None,
            count: Some(50),
            offset: Some(0),
            ic_type: None,
        }
    }
}

impl ResourceFilter for ListInterestCategoryFilter {
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
        if self.ic_type.is_some() {
            payload.insert(
                "type".to_string(),
                format!("{:}", self.ic_type.as_ref().unwrap().clone()),
            );
        }
        payload
    }
}

///
/// ListInterestCategoryBuilder
///
#[derive(Debug)]
pub struct ListInterestCategoryBuilder {
    /// Resource Endpoint
    pub endpoint: String,
}

impl BuildIter for ListInterestCategoryBuilder {
    type Item = ListInterestCategory;
    type FilterItem = ListInterestCategoryFilter;
    type Collection = CollectionListInterestCategories;

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
