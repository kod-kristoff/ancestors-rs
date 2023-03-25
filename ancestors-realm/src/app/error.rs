use thiserror::Error;

/// App error
#[derive(Debug, Error)]
pub enum Error {
    // #[error("audio error: {0}")]
    // Audio(AudioError),
    // #[error("game save error: {0}")]
    // SaveGame(SavedGameError),
    // #[error("ui error: {0}")]
    // Ui(UiError),
    #[error("unknown: {0}")]
    Unknown(String),
}
