use gedcomx_model::conclusion::Person;

use crate::{application::repositories::DynPersonRepository, domain::value_objects::Id};

pub struct AddingPerson {
    repo: DynPersonRepository,
}

impl AddingPerson {
    pub fn new(repo: DynPersonRepository) -> Self {
        Self { repo }
    }
}

impl AddingPerson {
    pub fn execute(&self) -> Result<(), UseCaseError> {
        let id = Id::gen();
        let person = Person::with_id(id)?;
        self.repo.save(person).unwrap();
        Ok(())
    }
}

#[derive(Debug)]
pub enum UseCaseError {
    GedcomxError(gedcomx_model::Error),
}

impl From<gedcomx_model::Error> for UseCaseError {
    fn from(value: gedcomx_model::Error) -> Self {
        Self::GedcomxError(value)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    use crate::application::repositories::mem::InMemoryPersonRepo;
    #[test]
    fn adding_person_succeds() {
        let repo = InMemoryPersonRepo::arc_new();
        let uc = AddingPerson::new(repo);

        uc.execute().unwrap();
    }
}
