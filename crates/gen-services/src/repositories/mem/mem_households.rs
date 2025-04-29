use gen_types::{Household, HouseholdId};
use hashbrown::HashMap;
use std::sync::{Arc, RwLock};

use crate::repositories::{
    HouseholdRepository, HouseholdRepositoryError, SharedHouseholdRepository,
};

#[derive(Default)]
pub struct InMemoryHouseholdRepo {
    storage: Arc<RwLock<HashMap<HouseholdId, Household>>>,
}

impl InMemoryHouseholdRepo {
    pub fn new() -> Self {
        let storage = Arc::new(RwLock::new(HashMap::new()));
        Self { storage }
    }

    pub fn arc_new() -> SharedHouseholdRepository {
        Arc::new(Self::new())
    }
}

impl HouseholdRepository for InMemoryHouseholdRepo {
    fn get_household(
        &self,
        id: &HouseholdId,
    ) -> Result<Option<Household>, HouseholdRepositoryError> {
        Ok(self.storage.read().expect("").get(id).cloned())
    }

    fn get_all_households(&self) -> Result<Vec<Household>, HouseholdRepositoryError> {
        Ok(self
            .storage
            .read()
            .expect("unpoisoned lock")
            .values()
            .cloned()
            .collect())
    }

    fn save_household(&self, household: &Household) -> Result<(), HouseholdRepositoryError> {
        self.storage
            .write()
            .unwrap()
            .insert(household.id(), household.clone());
        Ok(())
    }
}
