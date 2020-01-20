use super::link::LinkType;
use crate::api::MailchimpApi;
use crate::iter::{BuildIter, MailchimpCollection, SimpleFilter};
use serde::{Deserialize, Serialize};
use std::rc::Rc;

///
/// Signup form header options
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListSignupFormHeader {
    /// Header image URL.
    #[serde(default)]
    pub image_url: String,
    /// Header text.
    #[serde(default)]
    pub text: String,
    /// Image width, in pixels.
    #[serde(default)]
    pub image_width: String,
    /// Image height, in pixels.
    #[serde(default)]
    pub image_height: String,
    /// Alt text for the image.
    #[serde(default)]
    pub image_alt: String,
    /// The URL that the header image will link to.
    #[serde(default)]
    pub image_link: String,
    /// Image alignment. PV: none left center right
    #[serde(default)]
    pub image_align: String,
    /// Image border width.
    #[serde(default)]
    pub image_border_width: String,
    /// Image border style.
    /// Possible Values: none - solid - dotted - dashed - double - groove - outset - inset - ridge
    #[serde(default)]
    pub image_border_style: String,
    /// Image border color.
    #[serde(default)]
    pub image_border_color: String,
    /// Image link target.
    /// Possible Values: _blank - null
    #[serde(default)]
    pub image_target: String,
}

impl Default for ListSignupFormHeader {
    fn default() -> Self {
        ListSignupFormHeader {
            image_url: "".to_string(),
            text: "".to_string(),
            image_width: "".to_string(),
            image_height: "".to_string(),
            image_alt: "".to_string(),
            image_link: "".to_string(),
            image_align: "".to_string(),
            image_border_width: "".to_string(),
            image_border_style: "".to_string(),
            image_border_color: "".to_string(),
            image_target: "".to_string(),
        }
    }
}

///
/// The signup form body content.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListSignupFormContent {
    /// The content section name.
    /// Possible Values: signup_message - unsub_message - signup_thank_you_title
    #[serde(default)]
    pub section: String,
    /// The content section text.
    #[serde(default)]
    pub value: String,
}

///
/// An array of objects, each representing an element style for the signup form.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListSignupFormStyleOptions {
    ///
    /// A string that identifies the property.
    ///
    #[serde(default)]
    pub property: String,
    ///
    /// A string that identifies value of the property.
    ///
    #[serde(default)]
    pub value: String,
}

///
/// An array of objects, each representing an element style for the signup form.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListSignupFormStyle {
    /// A string that identifies the element selector.
    ///
    /// Possible Values:
    ///     page_background - page_header - page_outer_wrapper - body_background
    ///     body_link_style - forms_buttons - forms_buttons_hovered - forms_field_label
    ///     forms_field_text - forms_required - forms_required_legend - forms_help_text
    ///     forms_errors - monkey_rewards_badge
    ///
    #[serde(default)]
    pub selector: String,
    /// A collection of options for a selector.
    #[serde(default)]
    pub options: Vec<ListSignupFormStyleOptions>,
}

///
/// Signup Forms
///
///  Manage list signup forms.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListSignupForm {
    /// Options for customizing your signup form header.
    #[serde(default)]
    pub header: ListSignupFormHeader,
    /// The signup form body content.
    #[serde(default)]
    pub contents: Vec<ListSignupFormContent>,
    /// An array of objects, each representing an element style for the signup form.
    #[serde(default)]
    pub styles: Vec<ListSignupFormStyle>,
    /// Signup form URL.
    #[serde(default)]
    pub signup_form_url: String,
    /// The signup form’s list id.
    #[serde(default)]
    pub list_id: String,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

///
/// Response for endpoint  GET /lists/{list_id}/members/{subscriber_hash}/activity
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionListSignupForm {
    /// List signup form.
    #[serde(default)]
    pub signup_forms: Vec<ListSignupForm>,
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

impl MailchimpCollection<ListSignupForm> for CollectionListSignupForm {
    /// Total Items
    fn get_total_items(&self) -> u64 {
        self.total_items
    }

    /// Data
    fn get_values(&self) -> Vec<ListSignupForm> {
        self.signup_forms.clone()
    }
}

impl Default for CollectionListSignupForm {
    fn default() -> Self {
        CollectionListSignupForm {
            signup_forms: Vec::new(),
            list_id: "".to_string(),
            total_items: 0,
            _links: Vec::new(),
        }
    }
}

/// ================================= ITER =====================

///
/// ListSignupFormBuilder
///
#[derive(Debug)]
pub struct ListSignupFormBuilder {}

impl BuildIter for ListSignupFormBuilder {
    type Item = ListSignupForm;
    type FilterItem = SimpleFilter;
    type Collection = CollectionListSignupForm;

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
