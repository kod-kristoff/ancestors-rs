use id_ulid::{Id, Identifiable};

use crate::{
    shared::IdReference,
    value_objects::{Fact, ResourceReference},
};

use super::{
    shared::{Entity, Subject},
    DocumentReference, PlaceReference,
};

pub type PlaceRelationshipId = Id<PlaceRelationshipTag>;
pub type PlaceRelationshipReference = IdReference<PlaceRelationshipId>;
pub type PlaceRelationship = Entity<PlaceRelationshipTag, PlaceRelationshipBody>;

pub struct PlaceRelationshipTag;

impl Identifiable for PlaceRelationshipTag {
    const PREFIX: &'static str = "PRel";
}

/// Enumeration of standard relationship types.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlaceRelationshipType {
    SuperplaceSubplace,
    Other,
}
const QNAME_MAP: [(PlaceRelationshipType, &str); 1] = [(
    PlaceRelationshipType::SuperplaceSubplace,
    "http://kristoff.casa/SuperplaceSubplace",
)];
impl PlaceRelationshipType {
    pub fn as_qname_uri(&self) -> &str {
        for (relationship, name) in QNAME_MAP {
            if *self == relationship {
                return name;
            }
        }
        "OTHER"
    }
    pub fn from_qname_uri(qname_uri: &str) -> Self {
        for (relationship, qname) in QNAME_MAP {
            if qname == qname_uri {
                return relationship;
            }
        }
        Self::Other
    }
}
mod place_relationship_type {
    use super::PlaceRelationshipType;

    impl serde::Serialize for PlaceRelationshipType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(self.as_qname_uri())
        }
    }

    impl<'de> serde::Deserialize<'de> for PlaceRelationshipType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_str(PlaceRelationshipTypeVisitor)
        }
    }

    struct PlaceRelationshipTypeVisitor;

    impl<'de> serde::de::Visitor<'de> for PlaceRelationshipTypeVisitor {
        type Value = PlaceRelationshipType;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an uri")
        }
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(PlaceRelationshipType::from_qname_uri(v))
        }
    }
}
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PlaceRelationshipBody {
    r#type: PlaceRelationshipType,
    #[serde(flatten)]
    subject: Subject<PlaceRelationshipReference>,
    place1: Option<PlaceReference>,
    place2: Option<PlaceReference>,
    source: Option<ResourceReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    facts: Vec<Fact>,
}

impl PlaceRelationshipBody {
    pub fn new(r#type: PlaceRelationshipType) -> Self {
        Self {
            r#type,
            subject: Subject::default(),
            place1: None,
            place2: None,
            source: None,
            facts: Vec::default(),
        }
    }
}

// Builder lite
impl PlaceRelationshipBody {
    pub fn source<S: Into<ResourceReference>>(mut self, source: S) -> Self {
        self.source = Some(source.into());
        self
    }
    pub fn analysis<S: Into<DocumentReference>>(mut self, analysis: S) -> Self {
        self.set_analysis(analysis.into());
        self
    }
    pub fn evidence<S: Into<PlaceRelationshipReference>>(mut self, evidence: S) -> Self {
        self.add_evidence(evidence.into());
        self
    }
    pub fn place1<R: Into<PlaceReference>>(mut self, place1: R) -> Self {
        self.set_place1(place1.into());
        self
    }

    pub fn place2<R: Into<PlaceReference>>(mut self, place2: R) -> Self {
        self.set_place2(place2.into());
        self
    }

    pub fn fact(mut self, fact: Fact) -> Self {
        self.facts.push(fact);
        self
    }
}

impl PlaceRelationshipBody {
    // pub fn set_id(&mut self, id: String) {
    //     self.id = Some(id);
    // }
    pub fn set_place1(&mut self, place1: PlaceReference) {
        self.place1 = Some(place1);
    }
    pub fn set_place2(&mut self, place2: PlaceReference) {
        self.place2 = Some(place2);
    }
    pub fn is_extracted(&self) -> bool {
        self.subject.is_extracted()
    }
    pub fn set_analysis(&mut self, analysis: DocumentReference) {
        self.subject.set_analysis(analysis);
    }
    pub fn add_evidence(&mut self, evidence: PlaceRelationshipReference) {
        self.subject.add_evidence(evidence);
    }
}

// impl From<&Relationship> for EvidenceReference {
//     fn from(p: &Relationship) -> Self {
//         EvidenceReference::with_resource(format!("#{}", p.id))
//     }
// }
