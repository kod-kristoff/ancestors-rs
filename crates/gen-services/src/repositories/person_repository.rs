use gen_types::{Person, PersonId};

pub trait PersonRepository {
    fn get_person(&self, id: &PersonId) -> Result<Option<Person>, PersonRepositoryError>;
    fn get_all_persons(&self) -> Result<Vec<Person>, PersonRepositoryError>;
    fn save_person(&self, person: &Person) -> Result<(), PersonRepositoryError>;
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum PersonRepositoryError {
    #[error("Unknown error")]
    #[diagnostic(transparent)]
    Unknown(miette::Report),
}
