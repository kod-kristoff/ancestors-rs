use std::sync::{Arc, RwLock};

use ancestors_core::{
    port::repository::{PersonRepository, PersonRepositoryError},
    shared_kernel::component::person::domain::PersonId,
};

use gedcomx_model::GedcomX;
#[derive(Clone)]
pub struct SharedGedcomX(pub Arc<RwLock<GedcomX>>);

impl Default for SharedGedcomX {
    fn default() -> Self {
        Self(Arc::new(RwLock::new(GedcomX::new())))
    }
}

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
        id: &PersonId,
    ) -> Result<Option<gedcomx_model::conclusion::Person>, PersonRepositoryError> {
        Ok(self
            .storage
            .0
            .read()
            .expect("")
            .persons()
            .iter()
            .find(|p| p.id() == &id.value)
            .cloned())
    }

    fn save(&self, person: gedcomx_model::conclusion::Person) -> Result<(), PersonRepositoryError> {
        self.storage.0.write().unwrap().add_person(person);
        Ok(())
    }
}
