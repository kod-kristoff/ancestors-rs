pub use self::error::UseCaseError;
pub type UseCaseResult<T> = Result<T, UseCaseError>;

pub use self::agent_service::{AddAgentError, AgentService};
pub use self::person_service::{AddPerson, EditPerson, PersonService, UpsertPerson};

pub mod agent_service;
mod error;
pub mod person_service;

pub trait GenService: AgentService + PersonService {}
