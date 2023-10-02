use std::{fmt::Display, error::Error};

#[derive(Debug)]
pub enum UseCaseError {
    GedcomxError(gedcomx_model::Error),
}

impl From<gedcomx_model::Error> for UseCaseError {
    fn from(value: gedcomx_model::Error) -> Self {
        Self::GedcomxError(value)
    }
}

impl Display for UseCaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GedcomxError(err) => write!(f, "gedcomx model error: {}", err),
        }
    }
}

impl Error for UseCaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::GedcomxError(err) => Some(err),
        }
    }
}
