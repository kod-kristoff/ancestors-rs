use gen_types::Document;

pub trait DocumentService {
    // fn add_document(&self, user: &str, document: &AddDocument) -> Result<Document, AddDocumentError>;
    fn add_document_raw(
        &self,
        user: &str,
        document: Document,
    ) -> Result<Document, AddDocumentError>;
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum AddDocumentError {
    #[error("Unknown error")]
    #[diagnostic(transparent)]
    Unknown(miette::Report),
}
