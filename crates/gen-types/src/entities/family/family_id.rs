use id_ulid::{Id, Identifiable};

pub struct FamilyTag;

impl Identifiable for FamilyTag {
    const PREFIX: &'static str = "Fam";
}

pub type FamilyId = Id<FamilyTag>;
