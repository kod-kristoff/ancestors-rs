use std::sync::Arc;

pub use self::agent_repositry::{AgentRepository, AgentRepositoryError};
pub type SharedAgentRepository = Arc<dyn AgentRepository>;

pub use self::document_repository::{DocumentRepository, DocumentRepositoryError};
pub type SharedDocumentRepository = Arc<dyn DocumentRepository>;

pub use self::household_repository::{HouseholdRepository, HouseholdRepositoryError};
pub type SharedHouseholdRepository = Arc<dyn HouseholdRepository>;

pub use self::person_repository::{PersonRepository, PersonRepositoryError};
pub type SharedPersonRepository = Arc<dyn PersonRepository>;

mod agent_repositry;
mod document_repository;
mod household_repository;
pub mod mem;
mod person_repository;
mod place_repository;
mod relationship_repository;
mod source_repository;
