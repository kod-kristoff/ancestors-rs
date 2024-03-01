// use crate::{
//     common::{EvidenceReference, ResourceReference},
//     conclusion::Conclusion,
//     ser::{SerError, SerializeXml},
//     source::SourceReference,
// };

use serde::Deserializer;
use std::borrow::Cow;

use crate::{value_objects::Identifier, DocumentReference, SourceReference};
use serde::Serialize;

use super::Conclusion;

pub fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}
pub fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::Deserialize;

    let s: Cow<String> = Deserialize::deserialize(deserializer)?;

    match s.as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(serde::de::Error::unknown_variant(
            s.as_str(),
            &["true", "false"],
        )),
    }
}
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Subject<T: 'static> {
    #[serde(default, skip_serializing_if = "is_default")]
    extracted: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    evidence: Vec<T>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    identifiers: Vec<Identifier>,
    #[serde(flatten)]
    conclusion: Conclusion,
    // gender: Option<Gender>,
    // names: Vec<Name>,
    // facts: Vec<Fact>,
}

impl<T: Serialize> Subject<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: Serialize> Default for Subject<T> {
    fn default() -> Self {
        Self {
            extracted: true,
            evidence: Default::default(),
            identifiers: Default::default(),
            conclusion: Default::default(),
        }
    }
}

// Builder lite
impl<T: Serialize> Subject<T> {
    pub fn extracted(mut self, yes: bool) -> Self {
        self.extracted = yes;
        self
    }

    // pub fn id<S: Into<String>>(mut self, id: S) -> Self {
    //     self.set_id(id.into());
    //     self
    // }
    pub fn evidence<S: Into<T>>(mut self, evidence: S) -> Self {
        self.add_evidence(evidence.into());
        self
    }
    // pub fn gender(mut self, gender: Gender) -> Self {
    //     self.gender = Some(gender);
    //     self
    // }

    // pub fn name<N: Into<Name>>(mut self, name: N) -> Self {
    //     self.add_name(name.into());
    //     self
    // }

    // pub fn fact(mut self, fact: Fact) -> Self {
    //     self.facts.push(fact);
    //     self
    // }
}

impl<T: Serialize> Subject<T> {
    pub fn evidences(&self) -> &[T] {
        &self.evidence
    }
    pub fn identifiers(&self) -> &[Identifier] {
        &self.identifiers
    }
    pub fn set_extracted(&mut self, yes: bool) {
        self.extracted = yes;
    }

    // pub fn set_id(&mut self, id: String) {
    //     self.id = id;
    // }
    pub fn set_analysis(&mut self, analysis: DocumentReference) {
        self.conclusion.set_analysis(analysis);
    }
    pub fn add_source(&mut self, source: SourceReference) {
        self.conclusion.add_source(source);
    }
    pub fn add_evidence(&mut self, evidence: T) {
        self.evidence.push(evidence);
        self.set_extracted(false);
    }
    pub fn extracted_as_str(&self) -> &'static str {
        bool_as_str(self.extracted)
    }
    pub fn is_extracted(&self) -> bool {
        self.extracted
    }
    pub fn add_identifier(&mut self, identifier: Identifier) {
        self.identifiers.push(identifier);
    }
}
pub fn bool_as_str(b: bool) -> &'static str {
    if b {
        "true"
    } else {
        "false"
    }
}
