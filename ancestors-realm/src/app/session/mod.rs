//! # Session
//!
//! A Db session
use std::sync::{Arc, RwLock};

use ancestors::{domain::GedcomX, infrastructure::SharedGedcomX};

mod action;
mod effect;

pub use self::action::Action;
pub use self::effect::Message;

#[derive(Debug)]
pub struct Session {
    db: SharedGedcomX,
    metadata: SessionMetadata,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct SessionMetadata {}

impl Default for Session {
    fn default() -> Self {
        Self {
            db: Arc::new(RwLock::new(GedcomX::new())),
            metadata: Default::default(),
        }
    }
}

impl Default for SessionMetadata {
    fn default() -> Self {
        Self {}
    }
}

impl From<SessionMetadata> for Session {
    fn from(metadata: SessionMetadata) -> Self {
        Self {
            metadata,
            ..Default::default()
        }
    }
}

impl Session {
    pub fn metadata(&self) -> &SessionMetadata {
        &self.metadata
    }

    pub fn available_actions(&self) -> Vec<Action> {
        let mut actions = Vec::new();
        actions.push(Action::AddPerson);
        actions
    }

    #[cfg(test)]
    pub fn mock() -> Self {
        Self::default()
    }
}
