mod adding_person;
mod error;

pub use self::adding_person::{AddPerson, AddingPerson};
pub use self::error::UseCaseError;

pub type UseCaseResult<T> = Result<T, UseCaseError>;
