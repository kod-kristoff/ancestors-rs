use id_ulid::{Id, Identifiable};

pub struct DocumentTag;

impl Identifiable for DocumentTag {
    const PREFIX: &'static str = "Docu";
}

pub type DocumentId = Id<DocumentTag>;
