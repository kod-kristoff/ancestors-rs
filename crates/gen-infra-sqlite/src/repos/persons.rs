use std::{fmt::Pointer, path::Path};

use gen_services::repositories::PersonRepository;
use rusqlite::Connection;

use crate::Error;

pub struct RusqlitePersonRepo {
    conn: Connection,
}

impl RusqlitePersonRepo {
    pub fn new(db_path: impl AsRef<Path>) -> Result<Self, Error> {
        let conn = Connection::open(db_path.as_ref()).map_err(|source| Error::CouldNotOpenDb {
            path: db_path.as_ref().into(),
            source,
        })?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS persons (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data TEXT
        )",
            (), // empty list of parameters.
        )
        .map_err(|source| Error::CouldNotCreateTable {
            table_name: "persons".into(),
            source,
        })?;
        Ok(Self { conn })
    }
}

impl PersonRepository for RusqlitePersonRepo {
    fn save(
        &self,
        person: gen_types::Person,
    ) -> Result<(), gen_services::repositories::PersonRepositoryError> {
        let id = person.id();
        let name = person
            .names()
            .get(0)
            .map(|n| {
                n.name_forms()
                    .get(0)
                    .map(|nf| nf.get_full_text())
                    .unwrap_or("NO NAMEFORM")
            })
            .unwrap_or("NO NAME");
        let body = serde_json::to_string(&person).unwrap();

        self.conn
            .execute(
                "INSERT INTO persons (id, name, data) VALUES (?1, ?2, ?3)",
                (&id.db_id(), name, &body),
            )
            .unwrap();
        Ok(())
    }
    fn get(
        &self,
        id: &gen_types::PersonId,
    ) -> Result<Option<gen_types::Person>, gen_services::repositories::PersonRepositoryError> {
        let mut stmt = self
            .conn
            .prepare("SELECT data FROM persons WHERE id=:id")
            .unwrap();
        let mut rows = stmt.query(&[(":id", &id.db_id())]).unwrap();
        if let Some(row) = rows.next().unwrap() {
            let data: String = row.get(0).unwrap();
            Ok(serde_json::from_str(&data).unwrap())
        } else {
            Ok(None)
        }
    }
    fn get_all(
        &self,
    ) -> Result<Vec<gen_types::Person>, gen_services::repositories::PersonRepositoryError> {
        let mut stmt = self
            .conn
            .prepare("SELECT data FROM persons WHERE id=:id")
            .unwrap();
        let mut rows = stmt.query(&[(":id", &id.db_id())]).unwrap();
        let mut out = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            let data: String = row.get(2).unwrap();
            let person = serde_json::from_str(&data).unwrap();
            out.push(person);
        }
        Ok(out)
    }
}
