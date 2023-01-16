mod persons;
pub mod mem;

use std::sync::Arc;

pub use persons::PersonRepository;
pub type DynPersonRepository = Arc<dyn PersonRepository>;
