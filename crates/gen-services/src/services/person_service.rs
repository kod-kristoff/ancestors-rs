use gen_types::{Person, PersonId};

use crate::repositories::SharedPersonRepository;

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
            id: PersonId::default(),
            name: None,
            extracted: true,
        }
    }
}

impl AddPerson {
    pub fn name<S: Into<String>>(&mut self, name: S) {
        self.name = Some(name.into());
    }
}
#[derive(Debug, Clone)]
pub struct UpsertPerson {
    pub id: Option<PersonId>,
    pub extracted: bool,
    pub name: Option<String>,
}

impl Default for UpsertPerson {
    fn default() -> Self {
        Self {
            id: None,
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
        let mut person = Person::new(cmd.id);
        if let Some(name) = &cmd.name {
            person = person.name(name.as_str());
        }
        // person.set_extracted(cmd.extracted);
        self.repo.save(person).unwrap();
        Ok(())
    }

    pub fn edit(&self, cmd: &EditPerson) -> UseCaseResult<()> {
        let person = Person::new(cmd.id);
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
            id: *value.id(),
            // name: None,
            // extracted: true,
        }
    }
}

#[cfg(test)]
mod tests;
