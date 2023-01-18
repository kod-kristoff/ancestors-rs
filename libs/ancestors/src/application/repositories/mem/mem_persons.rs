use std::{
    sync::{Arc, RwLock},
};
use hashbrown::HashMap;
use crate::{
    application::repositories::PersonRepository,
    domain::{conclusion::Person, value_objects::Id},
};
use ulid::Ulid;

pub struct InMemoryPersonRepo {
    storage: Arc<RwLock<HashMap<Ulid, Person>>>,
}

impl InMemoryPersonRepo {
    pub fn new() -> Self {
        let storage = Arc::new(RwLock::new(HashMap::new()));
        Self { storage }
    }

    pub fn arc_new() -> Arc<Self> {
        Arc::new(Self::new())
    }
}

impl PersonRepository for InMemoryPersonRepo {
    fn get(&self, id: &Id) -> Result<Option<Person>, ()> {
        Ok(self.storage.read().expect("").get(&id.value).cloned())
    }
    fn save(&self, person: Person) -> Result<(), ()> {
        // todo!("implement save")
        self.storage.write().unwrap().insert(person.id(), person);
        Ok(())
    }
}
