mod household_id;

pub use household_id::HouseholdId;
use household_id::HouseholdTag;

use crate::{
    value_objects::{Fact, MemberInfo},
    PersonId,
};

use super::shared::Entity;

pub type Household = Entity<HouseholdTag, HouseholdBody>;

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct HouseholdBody {
    // id: HouseholdId,
    name: String,
    members: Vec<MemberInfo<PersonId>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    facts: Vec<Fact>,
}

// Builder lite
impl HouseholdBody {
    pub fn member(mut self, member: MemberInfo<PersonId>) -> Self {
        self.add_member(member);
        self
    }
}
impl HouseholdBody {
    pub fn add_fact(&mut self, fact: Fact) {
        self.facts.push(fact);
    }
    pub fn add_member(&mut self, new_member: MemberInfo<PersonId>) {
        for member in &self.members {
            if member.id() == new_member.id() {
                return;
            }
        }
        self.members.push(new_member);
    }

    pub fn members(&self) -> &[MemberInfo<PersonId>] {
        &self.members
    }

    pub fn facts(&self) -> &[Fact] {
        &self.facts
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
