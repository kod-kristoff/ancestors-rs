pub mod mem;
mod persons;
mod repository;

use std::sync::Arc;

pub use persons::PersonRepository;
pub type DynPersonRepository = Arc<dyn PersonRepository>;
