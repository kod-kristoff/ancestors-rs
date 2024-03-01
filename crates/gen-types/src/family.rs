use crate::{Fact, PersonId};

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Family {
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
