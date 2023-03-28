//! # session files utilities
//!
//! Utilities to save and load session files

use serde_json::Error as JsonError;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::{error::Error, fmt::Display};

use crate::app::session::SessionMetadata;
use crate::app::Session;

pub type SavedSessionResult<T> = Result<T, SavedSessionError>;

/// session file error
#[derive(Debug)]
pub enum SavedSessionError {
    Io(std::io::Error),
    Json(JsonError),
}

impl Display for SavedSessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "io error: {0}", err),
            Self::Json(err) => write!(f, "serialization error: {0}", err),
        }
    }
}

impl Error for SavedSessionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::Json(err) => Some(err),
        }
    }
}

impl From<std::io::Error> for SavedSessionError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<JsonError> for SavedSessionError {
    fn from(e: JsonError) -> Self {
        Self::Json(e)
    }
}

pub struct SavedSessions;

impl SavedSessions {
    /// Save `session` at `sessions_dir/name`
    pub fn save_session(
        name: &str,
        sessions_dir: &Path,
        session: &Session,
    ) -> SavedSessionResult<()> {
        log::debug!("saving session {}", name);
        let mut path = sessions_dir.to_path_buf();
        path.push(name);
        log::debug!("opening save file {}", path.display());
        let file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&path)?;
        log::debug!("serializing JSON to file");
        serde_json::to_writer(&file, session.metadata())?;
        log::info!("session saved");
        Ok(())
    }

    /// Load session at path
    pub fn load_session(path: &Path) -> SavedSessionResult<Session> {
        log::debug!("loading session at {}", path.display());
        let file = OpenOptions::new().read(true).open(path)?;
        log::debug!("session file opened");
        let session_metadata: SessionMetadata = serde_json::from_reader(file)?;
        log::info!("session loaded");
        Ok(session_metadata.into())
    }

    /// Returns the list of available saved sessions
    pub fn saved_sessions(sessions_dir: &Path) -> SavedSessionResult<Vec<PathBuf>> {
        log::debug!("scanning content of {}", sessions_dir.display());
        let files = std::fs::read_dir(sessions_dir)?
            .flatten()
            .into_iter()
            .filter(|x| x.path().is_file())
            .map(|x| x.path())
            .collect();
        Ok(files)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use tempfile::TempDir;

    #[test]
    fn should_save_session() {
        let session = Session::mock();
        let sessions_dir = TempDir::new().unwrap();
        assert!(SavedSessions::save_session("mysession", sessions_dir.path(), &session).is_ok());
    }

    #[test]
    fn should_load_session() {
        let session = Session::mock();
        let sessions_dir = TempDir::new().unwrap();
        SavedSessions::save_session("mysession", sessions_dir.path(), &session)
            .expect("failed to save session");
        let mut path = sessions_dir.path().to_path_buf();
        path.push("mysession");
        // assert_eq!(
        //     SavedSessions::load_session(&path).unwrap().maze_seed(),
        //     session.maze_seed()
        // );
    }

    #[test]
    fn should_get_sessions_list() {
        let session = Session::mock();
        let sessions_dir = TempDir::new().unwrap();
        SavedSessions::save_session("mysession", sessions_dir.path(), &session)
            .expect("failed to save session");
        SavedSessions::save_session("mysession2", sessions_dir.path(), &session)
            .expect("failed to save session");
        assert_eq!(
            SavedSessions::saved_sessions(sessions_dir.path())
                .unwrap()
                .len(),
            2
        );
    }
}
