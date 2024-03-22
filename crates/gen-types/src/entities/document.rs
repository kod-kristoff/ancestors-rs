mod document_id;
pub use document_id::DocumentId;
pub type DocumentReference = IdReference<DocumentId>;

use crate::shared::IdReference;

/// An abstract document that contains derived (conclusionary) text -- for example, a transcription or researcher analysis.

#[derive(Debug, Default, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Document {
    text: String,
    id: DocumentId,
}

impl Document {
    pub fn new(id: DocumentId) -> Self {
        Self {
            id,
            text: String::new(),
        }
    }
}

// Builder lite
impl Document {
    // pub fn id<S: Into<String>>(mut self, id: S) -> Self {
    //     self.set_id(id.into());
    //     self
    // }
    pub fn text<S: Into<String>>(mut self, text: S) -> Self {
        self.set_text(text.into());
        self
    }
}

impl Document {
    // pub fn set_id(&mut self, id: String) {
    //     self.id = id;
    // }
    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

// impl From<&str> for Document {
//     fn from(s: &str) -> Self {
//         Self::new().name_form(NameForm::new().full_text(s.into()))
//     }
// }

impl From<&Document> for DocumentReference {
    fn from(doc: &Document) -> Self {
        DocumentReference::new(doc.id)
    }
}

// impl From<DocumentReference> for ResourceReference {
//     fn from(doc_ref: DocumentReference) -> Self {
//         doc_ref.0
//     }
// }
