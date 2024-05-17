mod person_repository;
pub mod mem;

use std::sync::Arc;

pub use self::person_repository::{PersonRepository, PersonRepositoryError};
pub type SharedPersonRepository = Arc<dyn PersonRepository>;
