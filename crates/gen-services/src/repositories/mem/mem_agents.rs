use gen_types::{Agent, AgentId};
use hashbrown::HashMap;
use std::sync::{Arc, RwLock};

use crate::repositories::{AgentRepository, AgentRepositoryError, SharedAgentRepository};

#[derive(Default)]
pub struct InMemoryAgentRepo {
    storage: Arc<RwLock<HashMap<AgentId, Agent>>>,
}

impl InMemoryAgentRepo {
    pub fn new() -> Self {
        let storage = Arc::new(RwLock::new(HashMap::new()));
        Self { storage }
    }

    pub fn arc_new() -> SharedAgentRepository {
        Arc::new(Self::new())
    }
}

impl AgentRepository for InMemoryAgentRepo {
    fn get_agent(&self, id: &AgentId) -> Result<Option<Agent>, AgentRepositoryError> {
        Ok(self.storage.read().expect("").get(id).cloned())
    }

    fn get_all_agents(&self) -> Result<Vec<Agent>, AgentRepositoryError> {
        Ok(self
            .storage
            .read()
            .expect("unpoisoned lock")
            .values()
            .cloned()
            .collect())
    }

    fn save_agent(&self, agent: &Agent) -> Result<(), AgentRepositoryError> {
        self.storage
            .write()
            .unwrap()
            .insert(agent.id(), agent.clone());
        Ok(())
    }
}
