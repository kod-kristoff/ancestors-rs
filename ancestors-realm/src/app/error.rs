use crate::ui::UiError;
use std::{error::Error as StdError, fmt::Display};

/// App error
#[derive(Debug)]
pub enum Error {
    // #[error("audio error: {0}")]
    // Audio(AudioError),
    // #[error("game save error: {0}")]
    // SaveGame(SavedGameError),
    // #[error("ui error: {0}")]
    // Ui(UiError),
    Unknown(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ui(_) => write!(f, "ui error"),
            Self::Unknown(msg) => write!(f, "unknown: {}", msg),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Ui(err) => Some(err),
            Self::Unknown(_) => None,
        }
    }
}
