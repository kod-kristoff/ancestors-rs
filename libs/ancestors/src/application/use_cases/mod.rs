mod adding_person;
mod editing_person;
mod error;

pub use self::adding_person::{AddPerson, AddingPerson};
pub use self::editing_person::{EditPerson, EditingPerson};
pub use self::error::UseCaseError;

pub type UseCaseResult<T> = Result<T, UseCaseError>;
