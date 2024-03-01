use id_ulid::IdError;

use crate::{Fact, PersonId};

use super::name::Name;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Person {
    id: PersonId,
    names: Vec<Name>,
    extracted: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    facts: Vec<Fact>,
}

impl Default for Person {
    fn default() -> Self {
        Self {
            id: Default::default(),
            names: Default::default(),
            extracted: true,
            facts: Default::default(),
        }
    }
}
impl Person {
    pub fn new(id: PersonId) -> Person {
        Self {
            id,
            ..Default::default()
        }
    }
    pub fn with_id(id: &str) -> Result<Person, IdError> {
        let id: String = id.into();
        let person_id = id.parse()?;
        Ok(Self::new(person_id))
    }
}

// Builder lite
impl Person {
    pub fn extracted(mut self, yes: bool) -> Self {
        self.set_extracted(yes);
        self
    }

    // pub fn source<S: Into<SourceReference>>(mut self, source: S) -> Self {
    //     self.add_source(source.into());
    //     self
    // }
    // pub fn analysis<S: Into<DocumentReference>>(mut self, analysis: S) -> Self {
    //     self.set_analysis(analysis.into());
    //     self
    // }
    // pub fn evidence<S: Into<EvidenceReference>>(mut self, evidence: S) -> Self {
    //     self.subject.add_evidence(evidence.into());
    //     self
    // }
    // pub fn gender(mut self, gender: Gender) -> Self {
    //     self.set_gender(gender);
    //     self
    // }

    pub fn name<N: Into<Name>>(&mut self, name: N) {
        self.add_name(name.into());
    }

    pub fn fact(&mut self, fact: Fact) {
        self.add_fact(fact);
    }
}

impl Person {
    pub fn add_name(&mut self, name: Name) {
        self.names.push(name);
    }
    pub fn id(&self) -> &PersonId {
        &self.id
    }
    pub fn is_extracted(&self) -> bool {
        self.extracted
    }
    pub fn names(&self) -> &[Name] {
        &self.names
    }
    pub fn set_extracted(&mut self, yes: bool) {
        self.extracted = yes;
    }
    pub fn add_fact(&mut self, fact: Fact) {
        self.facts.push(fact);
    }
}
