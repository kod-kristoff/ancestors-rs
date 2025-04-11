use id_ulid::{Id, Identifiable};

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct DocumentTag;

impl Identifiable for DocumentTag {
    const PREFIX: &'static str = "Docu";
}

pub type DocumentId = Id<DocumentTag>;
