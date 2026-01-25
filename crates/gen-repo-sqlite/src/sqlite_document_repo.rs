use std::sync::Arc;

use diesel::{OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use gen_services::repositories::{DocumentRepository, SharedDocumentRepository};
use gen_types::{Document, DocumentId};

use crate::{
    models::{DocumentInDb, NewDocument},
    pool::DbPool,
};

pub struct SqliteDocumentRepository {
    db_pool: DbPool,
}

impl SqliteDocumentRepository {
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }

    pub fn arc_new(db_pool: DbPool) -> SharedDocumentRepository {
        Arc::new(Self::new(db_pool))
    }
}

impl DocumentRepository for SqliteDocumentRepository {
    fn get_document(
        &self,
        id: &DocumentId,
    ) -> Result<Option<Document>, gen_services::repositories::DocumentRepositoryError> {
        use crate::schema::documents::dsl::documents;
        let mut conn = self.db_pool.read().unwrap();
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
        let mut conn = self.db_pool.read().unwrap();
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

        let mut conn = self.db_pool.write().unwrap();
        let document = diesel::insert_into(documents::table)
            .values(&new_document)
            .execute(&mut conn)
            .unwrap();
        dbg!(document);
        Ok(())
    }
}
