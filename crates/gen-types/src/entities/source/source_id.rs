use id_ulid::{Id, Identifiable};

pub struct SourceTag;

impl Identifiable for SourceTag {
    const PREFIX: &'static str = "Sour";
}

pub type SourceId = Id<SourceTag>;
