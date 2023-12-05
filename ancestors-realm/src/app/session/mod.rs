//! # Session
//!
//! A Db session
use std::sync::{Arc, RwLock};

mod action;
mod effect;

use ancestors_core::port::repository::SharedPersonRepository;
use ancestors_infra_json::repository::json::mem_gedcomx_repository::{
    MemGedcomxPersonRepo, SharedMemStorage,
};

pub use self::action::Action;
pub use self::effect::Message;

// #[derive(Debug)]
pub struct Session {
    db: SharedMemStorage,
    metadata: SessionMetadata,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct SessionMetadata {}

impl Default for Session {
    fn default() -> Self {
        Self {
            db: SharedMemStorage::default(),
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

    pub fn get_person_repo(&self) -> SharedPersonRepository {
        MemGedcomxPersonRepo::arc_new(self.db.clone())
    }

    #[cfg(test)]
    pub fn mock() -> Self {
        Self::default()
    }
}
