use gedcomx_model::conclusion::Person;

use super::UseCaseResult;
use crate::{application::repositories::DynPersonRepository, value_objects::Id};

#[derive(Debug, Clone)]
pub struct EditPerson {
    pub id: Id,
    pub extracted: bool,
    pub name: Option<String>,
}

impl From<Person> for EditPerson {
    fn from(value: Person) -> Self {
        Self {
            id: value.id().into(),
            name: None,
            extracted: true,
        }
    }
}

pub struct EditingPerson {
    repo: DynPersonRepository,
}

impl EditingPerson {
    pub fn new(repo: DynPersonRepository) -> Self {
        Self { repo }
    }
}

impl EditingPerson {
    pub fn execute(&self, cmd: &EditPerson) -> UseCaseResult<()> {
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
        let uc = EditingPerson::new(repo);

        let cmd = EditPerson::default();
        uc.execute(&cmd).unwrap();
    }
}
