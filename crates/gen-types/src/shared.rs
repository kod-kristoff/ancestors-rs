mod id_reference;

pub use id_reference::IdReference;

pub type Iri = oxiri::Iri<String>;
pub type IriRef = oxiri::IriRef<String>;
pub type IriParseError = oxiri::IriParseError;
