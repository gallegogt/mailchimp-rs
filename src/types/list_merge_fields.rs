use super::empty::EmptyType;
use super::link::LinkType;
use crate::api::MailchimpApi;
use crate::internal::error_type::MailchimpErrorType;
use crate::internal::request::MailchimpResult;
use crate::iter::{BuildIter, MailchimpCollection, ResourceFilter};
///
/// Merge Fields
///
/// Manage merge fields (formerly merge vars) for a specific list. Learn more about merge field limits in Mailchimp.
///
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

///
/// Merge Field Options
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MergeFieldOptions {
    /// In an address field, the default country code if none supplied.
    #[serde(default)]
    pub default_country: u64,
    /// In a phone field, the phone number type: US or International.
    #[serde(default)]
    pub phone_format: String,
    /// In a date or birthday field, the format of the date.
    #[serde(default)]
    pub date_format: String,
    /// In a radio or dropdown non-group field, the available options for members to pick from.
    #[serde(default)]
    pub choices: Vec<String>,
    /// In a text field, the default length of the text field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
}

impl Default for MergeFieldOptions {
    fn default() -> Self {
        MergeFieldOptions {
            default_country: 0,
            phone_format: "".to_string(),
            date_format: "".to_string(),
            choices: Vec::new(),
            size: None,
        }
    }
}

///
/// Merge Fields
///
/// Manage merge fields (formerly merge vars) for a specific list. Learn more about merge field limits in Mailchimp.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListMergeField {
    /// An unchanging id for the merge field.
    #[serde(default)]
    pub merge_id: u64,
    /// The tag used in Mailchimp campaigns and for the /members endpoint.
    #[serde(default)]
    pub tag: String,
    /// The name of the merge field.
    #[serde(default)]
    pub name: String,
    /// The type for the merge field.
    #[serde(default, rename = "type")]
    pub mf_type: String,
    /// The boolean value if the merge field is required.
    #[serde(default)]
    pub required: bool,
    /// The default value for the merge field if null.
    #[serde(default)]
    pub default_value: String,
    /// Whether the merge field is displayed on the signup form.
    #[serde(default)]
    pub public: bool,
    /// The order that the merge field displays on the list signup form.
    #[serde(default)]
    pub display_order: u64,
    /// Extra options for some merge field types.
    #[serde(default)]
    pub options: MergeFieldOptions,
    /// Extra text to help the subscriber fill out the form.
    #[serde(default)]
    pub help_text: String,
    /// A string that identifies this merge field collections’ list.
    #[serde(default)]
    pub list_id: String,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,

    /// Mailchimp API
    #[serde(skip)]
    _api: Rc<MailchimpApi>,

    /// Endpoint
    #[serde(skip)]
    _endpoint: String,
}
///
/// Merge Fields Param
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListMergeFieldParam {
    /// The tag used in Mailchimp campaigns and for the /members endpoint.
    #[serde(default)]
    pub tag: Option<String>,
    /// The name of the merge field.
    #[serde(default)]
    pub name: Option<String>,
    /// The type for the merge field.
    #[serde(default, rename = "type")]
    pub mf_type: Option<String>,
    /// The boolean value if the merge field is required.
    #[serde(default)]
    pub required: Option<bool>,
    /// The default value for the merge field if null.
    #[serde(default)]
    pub default_value: Option<String>,
    /// Whether the merge field is displayed on the signup form.
    #[serde(default)]
    pub public: Option<bool>,
    /// The order that the merge field displays on the list signup form.
    #[serde(default)]
    pub display_order: Option<u64>,
    /// Extra options for some merge field types.
    #[serde(default)]
    pub options: MergeFieldOptions,
    /// Extra text to help the subscriber fill out the form.
    #[serde(default)]
    pub help_text: Option<String>,
}

///
/// Response for endpoint  GET /lists/{list_id}/members/{subscriber_hash}/activity
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionListMergeField {
    /// An array of objects, each representing a merge field resource.
    #[serde(default)]
    pub merge_fields: Vec<ListMergeField>,
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

impl MailchimpCollection<ListMergeField> for CollectionListMergeField {
    /// Total Items
    fn get_total_items(&self) -> u64 {
        self.total_items
    }
    /// Data
    fn get_values(&self) -> Vec<ListMergeField> {
        self.merge_fields.clone()
    }
}

impl Default for CollectionListMergeField {
    fn default() -> Self {
        CollectionListMergeField {
            list_id: "".to_string(),
            merge_fields: Vec::new(),
            total_items: 0,
            _links: Vec::new(),
        }
    }
}

/// ================================= ITER =====================

#[derive(Debug, Clone)]
pub struct ListMergeFieldFilter {
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
    /// The merge field type.
    pub f_type: Option<String>,
    /// The boolean value if the merge field is required.
    pub required: Option<bool>,
}

impl Default for ListMergeFieldFilter {
    fn default() -> Self {
        ListMergeFieldFilter {
            fields: None,
            exclude_fields: None,
            count: Some(50),
            offset: Some(0),
            f_type: None,
            required: None,
        }
    }
}

impl ResourceFilter for ListMergeFieldFilter {
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
        if self.f_type.is_some() {
            payload.insert(
                "type".to_string(),
                format!("{:}", self.f_type.as_ref().unwrap().clone()),
            );
        }
        if self.required.is_some() {
            payload.insert(
                "required".to_string(),
                format!("{:}", self.required.as_ref().unwrap().clone()),
            );
        }
        payload
    }
}

///
/// ListMergeFieldBuilder
///
#[derive(Debug)]
pub struct ListMergeFieldBuilder {
    /// Resource Endpoint
    pub endpoint: String,
}

impl BuildIter for ListMergeFieldBuilder {
    type Item = ListMergeField;
    type FilterItem = ListMergeFieldFilter;
    type Collection = CollectionListMergeField;

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

impl ListMergeField {
    ///
    /// Delete a specific merge field in a list.
    ///
    pub fn update(&self, param: ListMergeFieldParam) -> MailchimpResult<ListMergeField> {
        // PATCH /lists/{list_id}/merge-fields/{merge_id}
        let endpoint = self.get_base_endpoint();
        self._api
            .patch::<ListMergeField, ListMergeFieldParam>(&endpoint, param)
    }

    ///
    /// Delete a specific merge field in a list.
    ///
    pub fn delete(&self) -> Option<MailchimpErrorType> {
        // DELETE /lists/{list_id}/merge-fields/{merge_id}
        let endpoint = self.get_base_endpoint();
        match self._api.delete::<EmptyType>(&endpoint, HashMap::new()) {
            Ok(_) => None,
            Err(e) => Some(e),
        }
    }
    ///
    /// Set API
    ///
    pub fn set_api(&mut self, api: Rc<MailchimpApi>) {
        self._api = api;
    }
    ///
    /// Set Endpoint
    ///
    pub fn set_endpoint<'a>(&mut self, endpoint: &'a str) {
        self._endpoint = endpoint.to_string();
    }
    ///
    /// Get Base Endpoint
    ///
    fn get_base_endpoint(&self) -> String {
        let mut endpoint = self._endpoint.clone();
        endpoint.push_str(format!("/{}", self.merge_id).as_str());
        endpoint
    }
}
