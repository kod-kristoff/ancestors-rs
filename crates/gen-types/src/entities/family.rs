mod family_id;

pub use family_id::FamilyId;

use crate::{value_objects::Fact, PersonId};

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Family {
    id: FamilyId,
    name: String,
    members: Vec<PersonId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    facts: Vec<Fact>,
}

impl Family {
    pub fn add_fact(&mut self, fact: Fact) {
        self.facts.push(fact);
    }
}
