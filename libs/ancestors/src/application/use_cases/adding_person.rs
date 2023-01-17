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
    pub fn execute(&self) -> Result<(), ()> {
        let id = Id::gen();
        let person = Person::with_id(id);
        Ok(())
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
    }
}
