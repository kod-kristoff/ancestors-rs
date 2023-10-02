use gedcomx_model::conclusion::Person;

use super::UseCaseResult;
use crate::{application::repositories::DynPersonRepository, value_objects::Id};

#[derive(Debug, Clone)]
pub struct AddPerson {
    pub id: Id,
    pub extracted: bool,
    pub name: Option<String>,
}

impl Default for AddPerson {
    fn default() -> Self {
        Self {
            id: Id::gen(),
            name: None,
            extracted: true,
        }
    }
}

pub struct AddingPerson {
    repo: DynPersonRepository,
}

impl AddingPerson {
    pub fn new(repo: DynPersonRepository) -> Self {
        Self { repo }
    }
}

impl AddingPerson {
    pub fn execute(&self, cmd: &AddPerson) -> UseCaseResult<()> {
        let mut person = Person::with_id(cmd.id.clone())?;
        if let Some(name) = &cmd.name {
            person = person.name(name.as_str());
        }
        person.set_extracted(cmd.extracted);
        self.repo.save(person).unwrap();
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

        let cmd = AddPerson::default();
        uc.execute(&cmd).unwrap();
    }
}