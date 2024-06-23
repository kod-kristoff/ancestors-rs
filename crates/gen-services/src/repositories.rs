pub mod mem;
mod person_repository;

use std::sync::Arc;

pub use self::person_repository::{PersonRepository, PersonRepositoryError};
pub type SharedPersonRepository = Arc<dyn PersonRepository>;
