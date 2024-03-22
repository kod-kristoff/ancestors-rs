use gen_types::{Person, PersonId};
use hashbrown::HashMap;
use std::sync::{Arc, RwLock};

use crate::port::repository::{PersonRepository, PersonRepositoryError, SharedPersonRepository};

pub struct InMemoryPersonRepo {
    storage: Arc<RwLock<HashMap<PersonId, Person>>>,
}

impl InMemoryPersonRepo {
    pub fn new() -> Self {
        let storage = Arc::new(RwLock::new(HashMap::new()));
        Self { storage }
    }

    pub fn arc_new() -> SharedPersonRepository {
        Arc::new(Self::new())
    }
}

impl PersonRepository for InMemoryPersonRepo {
    fn get(&self, id: &PersonId) -> Result<Option<Person>, PersonRepositoryError> {
        Ok(self.storage.read().expect("").get(id).cloned())
    }

    fn save(&self, person: Person) -> Result<(), PersonRepositoryError> {
        self.storage
            .write()
            .unwrap()
            .insert(person.id().clone(), person.clone());
        Ok(())
    }
}
