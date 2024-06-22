use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum UseCaseError {
    GedcomxError(String),
    IdParseError(gen_types::shared::IriParseError),
    IdError(gen_types::Error),
}

// impl From<gedcomx_model::Error> for UseCaseError {
//     fn from(value: gedcomx_model::Error) -> Self {
//         Self::GedcomxError(value)
//     }
// }

impl From<gen_types::shared::IriParseError> for UseCaseError {
    fn from(value: gen_types::shared::IriParseError) -> Self {
        Self::IdParseError(value)
    }
}

impl Display for UseCaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GedcomxError(err) => write!(f, "gedcomx model error: {}", err),
            Self::IdError(err) => f.write_fmt(format_args!("failed to parse id '{}'", err)),
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

impl From<gen_types::Error> for UseCaseError {
    fn from(value: gen_types::Error) -> Self {
        Self::IdError(value)
    }
}
