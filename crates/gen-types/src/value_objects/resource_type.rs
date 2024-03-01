use crate::shared::Iri;

/// Enumeration of high-level genealogical resource types.
#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ResourceType {
    /// A historical record.
    Record,

    /// A collection.
    Collection,

    /// A digital artifact, such as a digital image or video.
    DigitalArtifact,

    /// A physical artifact.
    PhysicalArtifact,

    /// A person.
    Person,
}

impl ResourceType {
    fn as_qname_uri(&self) -> &str {
        match self {
            Self::Record => "http://gedcomx.org/Record",
            Self::Collection => "http://gedcomx.org/Collection",
            Self::DigitalArtifact => "http://gedcomx.org/DigitalArtifact",
            Self::PhysicalArtifact => "http://gedcomx.org/PhysicalArtifact",
            Self::Person => "http://gedcomx.org/Person",
        }
    }
}

impl From<ResourceType> for Iri {
    fn from(r: ResourceType) -> Self {
        Iri::parse(r.as_qname_uri().into()).unwrap()
    }
}
