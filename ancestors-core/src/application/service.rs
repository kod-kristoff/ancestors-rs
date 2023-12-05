mod error;
pub mod person_service;

pub use self::error::UseCaseError;
pub type UseCaseResult<T> = Result<T, UseCaseError>;

pub use self::person_service::{AddPerson, EditPerson, PersonService};
