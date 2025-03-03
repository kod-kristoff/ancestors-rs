mod agent_repositry;
pub mod mem;
mod person_repository;

use std::sync::Arc;

pub use self::agent_repositry::{AgentRepository, AgentRepositoryError};
pub type SharedAgentRepository = Arc<dyn AgentRepository>;

pub use self::person_repository::{PersonRepository, PersonRepositoryError};
pub type SharedPersonRepository = Arc<dyn PersonRepository>;
