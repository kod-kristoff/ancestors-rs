use id_ulid::{Id, Identifiable};

pub struct RelationshipTag;

impl Identifiable for RelationshipTag {
    const PREFIX: &'static str = "Rela";
}

pub type RelationshipId = Id<RelationshipTag>;
