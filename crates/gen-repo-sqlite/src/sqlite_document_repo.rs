use std::sync::Arc;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection,
};
use gen_services::repositories::{DocumentRepository, SharedDocumentRepository};
use gen_types::{Document, DocumentId};

use crate::models::{DocumentInDb, NewDocument};

type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub struct SqliteDocumentRepository {
    read_pool: DbPool,
    write_pool: DbPool,
}

impl SqliteDocumentRepository {
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

    pub fn arc_new(path: &str) -> SharedDocumentRepository {
        Arc::new(Self::new(path))
    }
}

impl DocumentRepository for SqliteDocumentRepository {
    fn get_document(
        &self,
        id: &DocumentId,
    ) -> Result<Option<Document>, gen_services::repositories::DocumentRepositoryError> {
        use crate::schema::documents::dsl::documents;
        let mut conn = self.read_pool.get().unwrap();
        let document = documents
            .find(id.db_id())
            .select(DocumentInDb::as_select())
            .first(&mut conn)
            .optional();
        dbg!(&document);
        match document {
            Ok(None) => Ok(None),
            Ok(Some(document)) => Ok(serde_json::from_str(&document.body).unwrap()),
            Err(err) => {
                todo!("handle error : {:?}", err)
            }
        }
    }

    fn get_all_documents(
        &self,
    ) -> Result<Vec<Document>, gen_services::repositories::DocumentRepositoryError> {
        use crate::schema::documents::dsl::documents;
        let mut conn = self.read_pool.get().unwrap();
        let all_documents = documents
            .select(DocumentInDb::as_select())
            .load(&mut conn)
            .unwrap();
        let mut result = Vec::new();
        for document in all_documents {
            result.push(serde_json::from_str(&document.body).unwrap());
        }
        Ok(result)
    }
    fn save_document(
        &self,
        document: &Document,
    ) -> Result<(), gen_services::repositories::DocumentRepositoryError> {
        use crate::schema::documents;

        let id = document.id().db_id();
        let body = serde_json::to_string(&document).unwrap();
        let new_document = NewDocument {
            id: &id,
            body: &body,
            updated: document.updated().naive_utc(),
            updated_by: document.updated_by(),
        };

        let mut conn = self.write_pool.get().unwrap();
        let document = diesel::insert_into(documents::table)
            .values(&new_document)
            .execute(&mut conn)
            .unwrap();
        dbg!(document);
        Ok(())
    }
}
