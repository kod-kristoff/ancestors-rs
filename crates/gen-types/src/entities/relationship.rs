mod relationship_id;
mod relationship_type;

pub use relationship_id::RelationshipId;
pub use relationship_type::RelationshipType;
pub type RelationshipReference = IdReference<RelationshipId>;

use super::shared::Subject;
use crate::shared::IdReference;
use crate::value_objects::{Fact, ResourceReference};
use crate::{DocumentReference, PersonReference};

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Relationship {
    // #[serde(default)]
    // id: Option<String>,
    r#type: RelationshipType,
    #[serde(flatten)]
    subject: Subject<RelationshipReference>,
    person1: Option<PersonReference>,
    person2: Option<PersonReference>,
    source: Option<ResourceReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    facts: Vec<Fact>,
    id: RelationshipId,
}

impl Relationship {
    pub fn new(r#type: RelationshipType) -> Self {
        Self {
            // id: None,
            r#type,
            subject: Subject::default(),
            person1: None,
            person2: None,
            source: None,
            facts: Vec::new(),
            id: Default::default(),
        }
    }
}

// Builder lite
impl Relationship {
    // pub fn id<S: Into<String>>(mut self, id: S) -> Self {
    //     self.set_id(id.into());
    //     self
    // }
    pub fn extracted(mut self, yes: bool) -> Self {
        self.subject = self.subject.extracted(yes);
        self
    }

    pub fn source<S: Into<ResourceReference>>(mut self, source: S) -> Self {
        self.source = Some(source.into());
        self
    }
    pub fn analysis<S: Into<DocumentReference>>(mut self, analysis: S) -> Self {
        self.set_analysis(analysis.into());
        self
    }
    pub fn evidence<S: Into<RelationshipReference>>(mut self, evidence: S) -> Self {
        self.add_evidence(evidence.into());
        self
    }
    pub fn person1<R: Into<PersonReference>>(mut self, person1: R) -> Self {
        self.set_person1(person1.into());
        self
    }

    pub fn person2<R: Into<PersonReference>>(mut self, person2: R) -> Self {
        self.set_person2(person2.into());
        self
    }

    pub fn fact(mut self, fact: Fact) -> Self {
        self.facts.push(fact);
        self
    }
}

impl Relationship {
    // pub fn set_id(&mut self, id: String) {
    //     self.id = Some(id);
    // }
    pub fn set_person1(&mut self, person1: PersonReference) {
        self.person1 = Some(person1);
    }
    pub fn set_person2(&mut self, person2: PersonReference) {
        self.person2 = Some(person2);
    }
    pub fn is_extracted(&self) -> bool {
        self.subject.is_extracted()
    }
    pub fn set_analysis(&mut self, analysis: DocumentReference) {
        self.subject.set_analysis(analysis);
    }
    pub fn add_evidence(&mut self, evidence: RelationshipReference) {
        self.subject.add_evidence(evidence);
    }
}

// impl From<&Relationship> for EvidenceReference {
//     fn from(p: &Relationship) -> Self {
//         EvidenceReference::with_resource(format!("#{}", p.id))
//     }
// }
