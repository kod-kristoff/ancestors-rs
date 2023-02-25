use std::sync::{Arc, RwLock};

use crate::{application::repositories::PersonRepository, domain::GedcomX};

pub type SharedGedcomX = Arc<RwLock<GedcomX>>;

pub struct MemGedcomxPersonRepo {
    storage: SharedGedcomX,
}

impl MemGedcomxPersonRepo {
    pub fn new(storage: SharedGedcomX) -> Self {
        Self { storage }
    }

    pub fn arc_new(storage: SharedGedcomX) -> Arc<Self> {
        Arc::new(Self::new(storage))
    }
}

impl PersonRepository for MemGedcomxPersonRepo {
    fn get(
        &self,
        id: &crate::value_objects::Id,
    ) -> Result<Option<gedcomx_model::conclusion::Person>, ()> {
        Ok(self
            .storage
            .read()
            .expect("")
            .persons()
            .iter()
            .find(|p| p.id() == &id.value)
            .cloned())
    }

    fn save(&self, person: gedcomx_model::conclusion::Person) -> Result<(), ()> {
        self.storage.write().unwrap().add_person(person);
        Ok(())
    }
}
