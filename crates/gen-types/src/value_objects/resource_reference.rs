use crate::Result;
use crate::{shared::IriRef, value_objects::Reference};

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ResourceReference(Reference);

impl ResourceReference {
    pub fn new(iri: IriRef) -> Self {
        Self(Reference::new(iri))
    }
    pub fn try_new(iri: String) -> Result<Self> {
        Ok(Self(Reference::try_new(iri)?))
    }
    // pub fn with_resource(resource: String) -> Self {
    //     Self(Reference::with_resource(resource))
    // }
}

impl ResourceReference {
    pub fn resource(&self) -> &str {
        self.0.resource()
    }
}
