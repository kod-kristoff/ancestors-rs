mod household_id;

pub use household_id::HouseholdId;

use crate::{value_objects::Fact, PersonId};

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Household {
    id: HouseholdId,
    name: String,
    members: Vec<PersonId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    facts: Vec<Fact>,
}

impl Household {
    pub fn add_fact(&mut self, fact: Fact) {
        self.facts.push(fact);
    }
}
