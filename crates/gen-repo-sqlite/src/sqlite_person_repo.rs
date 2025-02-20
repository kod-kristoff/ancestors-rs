use std::sync::Arc;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection,
};
use gen_services::repositories::{PersonRepository, SharedPersonRepository};
use gen_types::{Person, PersonId};

use crate::models::{NewPerson, PersonInDb};

type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub struct SqlitePersonRepository {
    read_pool: DbPool,
    write_pool: DbPool,
}

impl SqlitePersonRepository {
    pub fn new(path: &str) -> Self {
        let manager = ConnectionManager::new(path);

        let read_pool = Pool::builder()
            .max_size(5)
            .build(manager)
            .expect("sqlite_repo: build read_pool");

        let manager = ConnectionManager::new(path);

        let write_pool = Pool::builder()
            .max_size(1)
            .build(manager)
            .expect("sqlite_repo: build write_pool");

        Self {
            read_pool,
            write_pool,
        }
    }

    pub fn arc_new(path: &str) -> SharedPersonRepository {
        Arc::new(Self::new(path))
    }
}

impl PersonRepository for SqlitePersonRepository {
    fn get_person(
        &self,
        id: &PersonId,
    ) -> Result<Option<Person>, gen_services::repositories::PersonRepositoryError> {
        use crate::schema::persons::dsl::persons;
        let mut conn = self.read_pool.get().unwrap();
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
        let mut conn = self.read_pool.get().unwrap();
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
            updated: person.updated(),
            updated_by: person.updated_by(),
        };

        let mut conn = self.write_pool.get().unwrap();
        let person = diesel::insert_into(persons::table)
            .values(&new_person)
            .execute(&mut conn)
            .unwrap();
        dbg!(person);
        Ok(())
    }
}
