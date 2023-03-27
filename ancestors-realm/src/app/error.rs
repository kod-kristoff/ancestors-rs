use crate::serialization::sessions::SavedSessionError;
use crate::ui::UiError;
use std::{error::Error as StdError, fmt::Display};

/// App error
#[derive(Debug)]
pub enum Error {
    // #[error("audio error: {0}")]
    // Audio(AudioError),
    SaveSession(SavedSessionError),
    Ui(UiError),
    Unknown(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SaveSession(err) => write!(f, "session save error: {0}", err),
            Self::Ui(_) => write!(f, "ui error"),
            Self::Unknown(msg) => write!(f, "unknown: {}", msg),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::SaveSession(err) => Some(err),
            Self::Ui(err) => Some(err),
            Self::Unknown(_) => None,
        }
    }
}

impl From<SavedSessionError> for Error {
    fn from(value: SavedSessionError) -> Self {
        Self::SaveSession(value)
    }
}
impl From<UiError> for Error {
    fn from(value: UiError) -> Self {
        Self::Ui(value)
    }
}
