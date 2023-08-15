//! # Error
//!
//! Ui error
use std::{error::Error as StdError, fmt::Display};

use tuirealm::{terminal::TerminalError, ApplicationError};

/// Ui error
#[derive(Debug)]
pub enum UiError {
    FailedToGetSize,
    Application(ApplicationError),
    Io(std::io::Error),
    Terminal(TerminalError),
    // Unknown(String),
}

impl Display for UiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Application(_) => write!(f, "application error"),
            Self::Io(_) => write!(f, "io error"),
            Self::Terminal(_) => write!(f, "terminal error"),
            // Self::Unknown(msg) => write!(f, "unknown: {}", msg),
            Self::FailedToGetSize => write!(f, "failed to get terminal size"),
        }
    }
}

impl StdError for UiError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Application(err) => Some(err),
            Self::FailedToGetSize => None,
            Self::Io(err) => Some(err),
            Self::Terminal(err) => Some(err),
            // Self::Unknown(_) => None,
        }
    }
}

impl From<TerminalError> for UiError {
    fn from(value: TerminalError) -> Self {
        Self::Terminal(value)
    }
}

impl From<ApplicationError> for UiError {
    fn from(value: ApplicationError) -> Self {
        Self::Application(value)
    }
}

impl From<std::io::Error> for UiError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
