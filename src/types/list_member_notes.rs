use super::empty::EmptyType;
use super::link::LinkType;
use crate::api::MailchimpApi;
use crate::internal::error_type::MailchimpErrorType;
use crate::internal::request::MailchimpResult;
use crate::iter::{BuildIter, MailchimpCollection, SimpleFilter};
use std::collections::HashMap;

///
/// Member Notes
///
/// Retrieve recent notes for a specific list member.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListMemberNote {
    /// The note id.
    #[serde(default)]
    pub id: u64,
    /// The date and time the note was created in ISO 8601 format.
    #[serde(default)]
    pub created_at: String,
    /// The author of the note.
    #[serde(default)]
    pub created_by: String,
    /// The date and time the note was last updated in ISO 8601 format.
    #[serde(default)]
    pub updated_at: String,
    /// The content of the note.
    #[serde(default)]
    pub note: String,
    /// The unique id for the list.
    #[serde(default)]
    pub list_id: String,
    /// The MD5 hash of the lowercase version of the list member’s email address.
    #[serde(default)]
    pub email_id: String,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,

    /// Mailchimp API
    #[serde(skip)]
    _api: MailchimpApi,
    /// Edpoint
    #[serde(default)]
    _endpoint: String,
}

impl ListMemberNote {
    ///
    /// Update the api instance
    ///
    pub fn set_api(&mut self, n_api: &MailchimpApi) {
        self._api = n_api.clone()
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
        // DELETE /lists/{list_id}/members/{subscriber_hash}/notes/{note_id}
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
    ///     note: The content of the note. Note length is limited to 1,000 characters.
    ///
    pub fn update<'a>(&self, note: &'a str) -> MailchimpResult<ListMemberNote> {
        // PATCH /lists/{list_id}/members/{subscriber_hash}/notes/{note_id}
        let endpoint = self.get_base_endpoint();
        let mut payload = HashMap::new();
        payload.insert("note".to_string(), note.to_string());
        self._api
            .patch::<ListMemberNote, HashMap<String, String>>(&endpoint, payload)
    }

    ///
    /// Private function to build endpoint string
    ///
    fn get_base_endpoint(&self) -> String {
        // /lists/{list_id}/members/{subscriber_hash}/notes
        format!("{:?}/{:?}", self._endpoint, self.id)
    }
}

///
/// Response for endpoint
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionListMemberNote {
    /// An array of objects, each representing a note resource.
    #[serde(default)]
    pub notes: Vec<ListMemberNote>,
    /// The MD5 hash of the lowercase version of the list member’s email address.
    #[serde(default)]
    pub email_id: String,
    /// The MD5 hash of the lowercase version of the list member’s email address.
    #[serde(default)]
    pub list_id: String,
    /// The list id.
    #[serde(default)]
    pub total_items: u64,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

impl MailchimpCollection<ListMemberNote> for CollectionListMemberNote {
    /// Total Items
    fn get_total_items(&self) -> u64 {
        self.total_items
    }

    /// Data
    fn get_values(&self) -> Vec<ListMemberNote> {
        self.notes.clone()
    }
}

impl Default for CollectionListMemberNote {
    fn default() -> Self {
        CollectionListMemberNote {
            list_id: "".to_string(),
            email_id: "".to_string(),
            notes: Vec::new(),
            total_items: 0,
            _links: Vec::new(),
        }
    }
}

///
/// ListMemberNoteBuilder
///
#[derive(Debug)]
pub struct ListMemberNoteBuilder {
    /// Resource Endpoint
    pub endpoint: String,
}

impl BuildIter for ListMemberNoteBuilder {
    type Item = ListMemberNote;
    type FilterItem = SimpleFilter;
    type Collection = CollectionListMemberNote;

    ///
    /// Crea un recurso a partir del dato pasado por parámetro
    ///
    fn update_item(&self, data: &Self::Item, api: &MailchimpApi) -> Self::Item {
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
