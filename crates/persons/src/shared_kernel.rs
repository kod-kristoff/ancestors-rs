mod person_id;
pub use person_id::PersonId;

pub type IriRef = oxiri::IriRef<String>;
pub use oxiri::IriParseError;
// pub use gedcomx_model::common::{IriParseError, IriRef};
