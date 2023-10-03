use gedcomx_model::common::IriRef;
use hashbrown::HashMap;
use std::sync::{Arc, RwLock};

use crate::component::person::domain::Person;
use crate::port::repository::{PersonRepository, SharedPersonRepository};
use crate::shared_kernel::component::person::domain::PersonId;

pub struct InMemoryPersonRepo {
    storage: Arc<RwLock<HashMap<IriRef, Person>>>,
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
    fn get(&self, id: &PersonId) -> Result<Option<Person>, ()> {
        Ok(self.storage.read().expect("").get(&id.value).cloned())
    }

    fn save(&self, person: Person) -> Result<(), ()> {
        // todo!("implement save")
        self.storage
            .write()
            .unwrap()
            .insert(person.id().clone(), person.clone());
        Ok(())
    }
}