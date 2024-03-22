use std::{error::Error as StdError, fmt, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IdError(id_ulid::IdError),
    IriParseError(oxiri::IriParseError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IdError(_) => f.write_str("id error"),
            Self::IriParseError(_) => write!(f, "Failed parse Iri"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::IdError(err) => Some(err),
            Self::IriParseError(err) => Some(err),
        }
    }
}

impl From<oxiri::IriParseError> for Error {
    fn from(err: oxiri::IriParseError) -> Self {
        Self::IriParseError(err)
    }
}

impl From<id_ulid::IdError> for Error {
    fn from(value: id_ulid::IdError) -> Self {
        Self::IdError(value)
    }
}
