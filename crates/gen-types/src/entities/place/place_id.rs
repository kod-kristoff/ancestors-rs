use id_ulid::{Id, Identifiable};

#[derive(Debug, Clone, Copy)]
pub struct PlaceTag;

impl Identifiable for PlaceTag {
    const PREFIX: &'static str = "Plac";
}

pub type PlaceId = Id<PlaceTag>;
