use gen_types::{Document, DocumentId};

pub trait DocumentRepository {
    fn get_document(&self, id: &DocumentId) -> Result<Option<Document>, DocumentRepositoryError>;
    fn get_all_documents(&self) -> Result<Vec<Document>, DocumentRepositoryError>;
    fn save_document(&self, document: &Document) -> Result<(), DocumentRepositoryError>;
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum DocumentRepositoryError {
    #[error("Unknown error")]
    #[diagnostic(transparent)]
    Unknown(miette::Report),
}
