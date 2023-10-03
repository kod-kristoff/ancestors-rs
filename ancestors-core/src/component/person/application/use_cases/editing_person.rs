use gedcomx_model::conclusion::Person;

use crate::{
    port::repository::SharedPersonRepository, shared_kernel::component::person::domain::PersonId,
};

use super::UseCaseResult;

#[derive(Debug, Clone)]
pub struct EditPerson {
    pub id: PersonId,
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
    repo: SharedPersonRepository,
}

impl EditingPerson {
    pub fn new(repo: SharedPersonRepository) -> Self {
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

    use crate::infrastructure::repository::in_memory::InMemoryPersonRepo;
    #[test]
    fn adding_person_succeds() {
        let repo = InMemoryPersonRepo::arc_new();
        let uc = EditingPerson::new(repo);

        let cmd = EditPerson::default();
        uc.execute(&cmd).unwrap();
    }
}
