use gen_types::{Person, PersonId};

#[derive(Debug, Clone)]
pub struct AddPerson {
    pub extracted: bool,
    pub name: Option<String>,
}

impl Default for AddPerson {
    fn default() -> Self {
        Self {
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
pub trait PersonService {
    fn add_person(&self, user: &str, person: &AddPerson) -> Result<Person, AddPersonError>;
    fn add_person_raw(&self, user: &str, person: Person) -> Result<Person, AddPersonError>;
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum AddPersonError {
    #[error("Unknown error")]
    #[diagnostic(transparent)]
    Unknown(miette::Report),
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
            id: value.id(),
            // name: None,
            // extracted: true,
        }
    }
}
