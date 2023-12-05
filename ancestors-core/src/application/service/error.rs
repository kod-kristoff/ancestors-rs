use std::{error::Error, fmt::Display};

use crate::shared_kernel;

#[derive(Debug)]
pub enum UseCaseError {
    GedcomxError(String),
    IdParseError(shared_kernel::IriParseError),
}

// impl From<gedcomx_model::Error> for UseCaseError {
//     fn from(value: gedcomx_model::Error) -> Self {
//         Self::GedcomxError(value)
//     }
// }

impl From<shared_kernel::IriParseError> for UseCaseError {
    fn from(value: shared_kernel::IriParseError) -> Self {
        Self::IdParseError(value)
    }
}

impl Display for UseCaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GedcomxError(err) => write!(f, "gedcomx model error: {}", err),
            Self::IdParseError(err) => f.write_fmt(format_args!("failed to parse id '{}'", err)),
        }
    }
}

impl Error for UseCaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::IdParseError(err) => Some(err),
            _ => None,
        }
    }
}
