use std::sync::Arc;

use diesel::{OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use gen_services::repositories::{PersonRepository, SharedPersonRepository};
use gen_types::{Person, PersonId};

use crate::{
    models::{NewPerson, PersonInDb},
    pool::DbPool,
};

pub struct SqlitePersonRepository {
    db_pool: DbPool,
}

impl SqlitePersonRepository {
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }

    pub fn arc_new(db_pool: DbPool) -> SharedPersonRepository {
        Arc::new(Self::new(db_pool))
    }
}

impl PersonRepository for SqlitePersonRepository {
    fn get_person(
        &self,
        id: &PersonId,
    ) -> Result<Option<Person>, gen_services::repositories::PersonRepositoryError> {
        use crate::schema::persons::dsl::persons;
        let mut conn = self.db_pool.read().unwrap();
        let person = persons
            .find(id.db_id())
            .select(PersonInDb::as_select())
            .first(&mut conn)
            .optional();
        dbg!(&person);
        match person {
            Ok(None) => Ok(None),
            Ok(Some(person)) => Ok(serde_json::from_str(&person.body).unwrap()),
            Err(err) => {
                todo!("handle error : {:?}", err)
            }
        }
    }

    fn get_all_persons(
        &self,
    ) -> Result<Vec<Person>, gen_services::repositories::PersonRepositoryError> {
        use crate::schema::persons::dsl::persons;
        let mut conn = self.db_pool.read().unwrap();
        let all_persons = persons
            .select(PersonInDb::as_select())
            .load(&mut conn)
            .unwrap();
        let mut result = Vec::new();
        for person in all_persons {
            result.push(serde_json::from_str(&person.body).unwrap());
        }
        Ok(result)
    }
    fn save_person(
        &self,
        person: &Person,
    ) -> Result<(), gen_services::repositories::PersonRepositoryError> {
        use crate::schema::persons;

        let id = person.id().db_id();
        let body = serde_json::to_string(&person).unwrap();
        let new_person = NewPerson {
            id: &id,
            extracted: person.body().is_extracted(),
            body: &body,
            updated: person.updated().naive_utc(),
            updated_by: person.updated_by(),
        };

        let mut conn = self.db_pool.write().unwrap();
        let person = diesel::insert_into(persons::table)
            .values(&new_person)
            .execute(&mut conn)
            .unwrap();
        dbg!(person);
        Ok(())
    }
}
