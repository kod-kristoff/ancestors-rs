use ancestors::domain::conclusion::Person;
use bonsaidb::{
    core::schema::Collection,
    local::{config::StorageConfiguration, Database},
};

pub struct BonsaiPersonRepo {
    db: Database,
}

pub struct BonsaiRepoError {}

impl BonsaiPersonRepo {
    pub fn new(dbpath: &str) -> Result<Self, BonsaiRepoError> {
        Ok(Self {
            db: Database::open::<DbPerson>(StorageConfiguration::new(dbpath)),
        })
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
        let repo = BonsaiPersonRepo::new();
        assert!(repo.is_ok());
    }
}
