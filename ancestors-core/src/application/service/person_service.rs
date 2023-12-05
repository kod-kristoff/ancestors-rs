use crate::domain::Person;
use crate::port::repository::SharedPersonRepository;
use crate::shared_kernel::PersonId;

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

pub struct PersonService {
    repo: SharedPersonRepository,
}

impl PersonService {
    pub fn new(repo: SharedPersonRepository) -> Self {
        Self { repo }
    }
}

impl PersonService {
    pub fn add(&self, cmd: &AddPerson) -> UseCaseResult<()> {
        let mut person = Person::with_id(cmd.id.clone())?;
        if let Some(name) = &cmd.name {
            person = person.name(name.as_str());
        }
        person.set_extracted(cmd.extracted);
        self.repo.save(person).unwrap();
        Ok(())
    }

    pub fn edit(&self, cmd: &EditPerson) -> UseCaseResult<()> {
        let mut person = Person::with_id(cmd.id.clone())?;
        // if let Some(name) = &cmd.name {
        //     person = person.name(name.as_str());
        // }
        // person.set_extracted(cmd.extracted);
        self.repo.save(person).unwrap();
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct EditPerson {
    pub id: PersonId,
    // pub extracted: bool,
    // pub name: Option<String>,
}

impl From<Person> for EditPerson {
    fn from(value: Person) -> Self {
        Self {
            id: value.id().clone(),
            // name: None,
            // extracted: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::infrastructure::repository::in_memory::InMemoryPersonRepo;
    #[test]
    fn adding_person_succeeds() {
        let repo = InMemoryPersonRepo::arc_new();
        let service = PersonService::new(repo.clone());

        let cmd = AddPerson::default();
        service.add(&cmd).unwrap();

        let person = repo.get(&cmd.id).unwrap();
    }
    #[test]
    fn editing_person_succeds() {
        let repo = InMemoryPersonRepo::arc_new();
        let service = PersonService::new(repo.clone());

        let add_cmd = AddPerson::default();
        service.add(&add_cmd).unwrap();
        let cmd = EditPerson {
            id: add_cmd.id.clone(),
        };
        service.edit(&cmd).unwrap();
    }
}
