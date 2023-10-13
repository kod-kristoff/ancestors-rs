use std::{error, fmt};

use crate::component::person::domain::Person;
use crate::shared_kernel::component::person::domain::PersonId;

pub trait PersonRepository {
    fn get(&self, id: &PersonId) -> Result<Option<Person>, PersonRepositoryError>;
    fn save(&self, person: Person) -> Result<(), PersonRepositoryError>;
}

#[derive(Debug)]
pub enum PersonRepositoryError {
    PersonNotFound(PersonId),
}

impl fmt::Display for PersonRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PersonNotFound(id) => write!(f, "Person '{id}' not found"),
        }
    }
}

impl error::Error for PersonRepositoryError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            _ => None,
        }
    }
}
