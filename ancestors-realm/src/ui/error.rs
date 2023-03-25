//! # Error
//!
//! Ui error
use std::{error::Error as StdError, fmt::Display};

use tuirealm::{terminal::TerminalError, ApplicationError};

/// Ui error
#[derive(Debug)]
pub enum UiError {
    // #[error("failed to get terminal size")]
    // FailedToGetSize,
    // #[error("application error: {0}")]
    // Application(ApplicationError),
    // #[error("io error: {0}")]
    // Io(std::io::Error),
    Terminal(TerminalError),
    Unknown(String),
}

impl Display for UiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Terminal(_) => write!(f, "terminal error"),
            Self::Unknown(msg) => write!(f, "unknown: {}", msg),
        }
    }
}

impl StdError for UiError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Terminal(err) => Some(err),
            Self::Unknown(_) => None,
        }
    }
}

impl From<TerminalError> for UiError {
    fn from(value: TerminalError) -> Self {
        Self::Terminal(value)
    }
}
