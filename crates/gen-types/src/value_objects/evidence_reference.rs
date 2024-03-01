// use crate::common::IriRef;

use crate::shared::IriRef;

use super::Reference;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct EvidenceReference {
    #[serde(rename = "resource", flatten)]
    reference: Reference,
}

impl EvidenceReference {
    pub fn new(iri: IriRef) -> Self {
        Self {
            reference: Reference::new(iri),
        }
    }
    // pub fn with_resource(resource: String) -> Self {
    //     Self {
    //         reference: Reference::with_resource(resource),
    //     }
    // }
}

impl EvidenceReference {
    pub fn resource(&self) -> &str {
        self.reference.resource()
    }
}
