use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use gen_services::repositories::{PersonRepository, PersonRepositoryError};
use gen_types::{Person, PersonId};

// use gedcomx_model::GedcomX;
#[derive(Clone)]
pub struct SharedMemStorage(pub Arc<RwLock<HashMap<PersonId, Person>>>);

impl Default for SharedMemStorage {
    fn default() -> Self {
        Self(Arc::new(RwLock::new(HashMap::new())))
    }
}

pub struct MemGedcomxPersonRepo {
    storage: SharedMemStorage,
}

impl MemGedcomxPersonRepo {
    pub fn new(storage: SharedMemStorage) -> Self {
        Self { storage }
    }

    pub fn arc_new(storage: SharedMemStorage) -> Arc<Self> {
        Arc::new(Self::new(storage))
    }
}

impl PersonRepository for MemGedcomxPersonRepo {
    fn get(&self, id: &PersonId) -> Result<Option<Person>, PersonRepositoryError> {
        Ok(self.storage.0.read().expect("").get(id).cloned())
    }

    fn save(&self, person: Person) -> Result<(), PersonRepositoryError> {
        self.storage.0.write().unwrap().insert(*person.id(), person);
        Ok(())
    }
}
