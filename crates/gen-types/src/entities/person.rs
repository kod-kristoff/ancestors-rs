mod person_id;

pub use person_id::PersonId;
pub type PersonReference = IdReference<PersonId>;

use id_ulid::IdError;

use super::shared::Subject;
use crate::{
    shared::IdReference,
    value_objects::{Fact, Gender, Identifier, Name},
    DocumentReference, SourceReference,
};

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Person {
    id: PersonId,
    names: Vec<Name>,
    #[serde(flatten)]
    subject: Subject<PersonReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    gender: Option<Gender>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    facts: Vec<Fact>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    sources: Vec<SourceReference>,
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
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
    // pub fn extracted(mut self, yes: bool) -> Self {
    //     self.set_extracted(yes);
    //     self
    // }

    pub fn source<S: Into<SourceReference>>(mut self, source: S) -> Self {
        self.add_source(source.into());
        self
    }
    pub fn analysis<S: Into<DocumentReference>>(mut self, analysis: S) -> Self {
        self.set_analysis(analysis.into());
        self
    }
    pub fn evidence<S: Into<PersonReference>>(mut self, evidence: S) -> Self {
        self.add_evidence(evidence.into());
        self
    }
    pub fn gender(mut self, gender: Gender) -> Self {
        self.set_gender(gender);
        self
    }

    pub fn name<N: Into<Name>>(mut self, name: N) -> Self {
        self.add_name(name.into());
        self
    }

    pub fn fact(mut self, fact: Fact) -> Self {
        self.add_fact(fact);
        self
    }
    pub fn identifier(mut self, identifier: Identifier) -> Self {
        self.add_identifier(identifier);
        self
    }
}

impl Person {
    pub fn id(&self) -> &PersonId {
        &self.id
    }
    pub fn is_extracted(&self) -> bool {
        self.subject.is_extracted()
    }
    pub fn names(&self) -> &[Name] {
        &self.names
    }
    pub fn facts(&self) -> &[Fact] {
        &self.facts
    }
    pub fn evidences(&self) -> &[PersonReference] {
        &self.subject.evidences()
    }
    pub fn identifiers(&self) -> &[Identifier] {
        &self.subject.identifiers()
    }
    pub fn add_name(&mut self, name: Name) {
        self.names.push(name);
    }
    pub fn set_gender(&mut self, gender: Gender) {
        self.gender = Some(gender);
    }

    pub fn add_source(&mut self, source: SourceReference) {
        self.sources.push(source);
    }
    pub fn add_fact(&mut self, fact: Fact) {
        self.facts.push(fact);
    }
    pub fn set_analysis(&mut self, analysis: DocumentReference) {
        self.subject.set_analysis(analysis);
    }
    pub fn add_evidence(&mut self, evidence: PersonReference) {
        self.subject.add_evidence(evidence);
    }
    pub fn add_identifier(&mut self, identifier: Identifier) {
        self.subject.add_identifier(identifier);
    }
}

impl From<&Person> for PersonReference {
    fn from(value: &Person) -> Self {
        Self::new(value.id.clone())
    }
}
