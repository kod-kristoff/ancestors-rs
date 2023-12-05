use ancestors_core::domain::Person;
use bonsaidb::{
    core::schema::Collection,
    local::{
        config::{Builder, StorageConfiguration},
        Database,
    },
};

pub struct BonsaiPersonRepo {
    db: Database,
}

#[derive(Debug)]
pub enum BonsaiRepoError {
    BonsaiError(bonsaidb::local::Error),
}

impl From<bonsaidb::local::Error> for BonsaiRepoError {
    fn from(value: bonsaidb::local::Error) -> Self {
        BonsaiRepoError::BonsaiError(value)
    }
}

impl BonsaiPersonRepo {
    pub fn new(dbpath: &str) -> Result<Self, BonsaiRepoError> {
        let db = Database::open::<DbPerson>(StorageConfiguration::new(dbpath))?;
        Ok(Self { db })
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Collection)]
#[collection(name = "persons")]
struct DbPerson {
    inner: Person,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let db_path = "data/tmp/unit_test_it_works.bonsaidb";
        let repo = BonsaiPersonRepo::new(db_path);
        assert!(repo.is_ok());
    }
}
