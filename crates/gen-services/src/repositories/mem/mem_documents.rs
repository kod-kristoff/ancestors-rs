use gen_types::{Document, DocumentId};
use hashbrown::HashMap;
use std::sync::{Arc, RwLock};

use crate::repositories::{DocumentRepository, DocumentRepositoryError, SharedDocumentRepository};

#[derive(Default)]
pub struct InMemoryDocumentRepo {
    storage: Arc<RwLock<HashMap<DocumentId, Document>>>,
}

impl InMemoryDocumentRepo {
    pub fn new() -> Self {
        let storage = Arc::new(RwLock::new(HashMap::new()));
        Self { storage }
    }

    pub fn arc_new() -> SharedDocumentRepository {
        Arc::new(Self::new())
    }
}

impl DocumentRepository for InMemoryDocumentRepo {
    fn get_document(&self, id: &DocumentId) -> Result<Option<Document>, DocumentRepositoryError> {
        Ok(self.storage.read().expect("").get(id).cloned())
    }

    fn get_all_documents(&self) -> Result<Vec<Document>, DocumentRepositoryError> {
        Ok(self
            .storage
            .read()
            .expect("unpoisoned lock")
            .values()
            .cloned()
            .collect())
    }

    fn save_document(&self, document: &Document) -> Result<(), DocumentRepositoryError> {
        self.storage
            .write()
            .unwrap()
            .insert(document.id(), document.clone());
        Ok(())
    }
}
