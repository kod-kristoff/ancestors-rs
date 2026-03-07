use id_ulid::{Id, Identifiable};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct HouseholdTag;

impl Identifiable for HouseholdTag {
    const PREFIX: &'static str = "Hous";
}

pub type HouseholdId = Id<HouseholdTag>;
