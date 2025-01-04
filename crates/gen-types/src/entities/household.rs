mod household_id;

pub use household_id::HouseholdId;
use household_id::HouseholdTag;

use crate::{value_objects::Fact, PersonId};

use super::shared::Entity;

pub type Household = Entity<HouseholdTag, HouseholdBody>;

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct HouseholdBody {
    // id: HouseholdId,
    name: String,
    members: Vec<PersonId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    facts: Vec<Fact>,
}

// Builder lite
impl HouseholdBody {
    pub fn member(mut self, person_id: PersonId) -> Self {
        self.add_member(person_id);
        self
    }
}
impl HouseholdBody {
    pub fn add_fact(&mut self, fact: Fact) {
        self.facts.push(fact);
    }
    pub fn add_member(&mut self, person_id: PersonId) {
        if !self.members.contains(&person_id) {
            self.members.push(person_id);
        }
    }
}
