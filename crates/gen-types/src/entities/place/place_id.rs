use id_ulid::{Id, Identifiable};

pub struct PlaceTag;

impl Identifiable for PlaceTag {
    const PREFIX: &'static str = "Plac";
}

pub type PlaceId = Id<PlaceTag>;
