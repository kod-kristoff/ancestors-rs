use crate::component::person::domain::Person;
use crate::port::repository::SharedPersonRepository;
use crate::shared_kernel::component::person::domain::PersonId;

use super::UseCaseResult;

#[derive(Debug, Clone)]
pub struct AddPerson {
    pub id: PersonId,
    pub extracted: bool,
    pub name: Option<String>,
}

impl Default for AddPerson {
    fn default() -> Self {
        Self {
            id: PersonId::gen(),
            name: None,
            extracted: true,
        }
    }
}

pub struct AddingPerson {
    repo: SharedPersonRepository,
}

impl AddingPerson {
    pub fn new(repo: SharedPersonRepository) -> Self {
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

    use crate::infrastructure::repository::in_memory::InMemoryPersonRepo;
    #[test]
    fn adding_person_succeds() {
        let repo = InMemoryPersonRepo::arc_new();
        let uc = AddingPerson::new(repo);

        let cmd = AddPerson::default();
        uc.execute(&cmd).unwrap();
    }
}
