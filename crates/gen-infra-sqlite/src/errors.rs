use std::path::PathBuf;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum Error {
    #[error("Could not open database at '{path}'")]
    CouldNotOpenDb {
        path: PathBuf,
        source: rusqlite::Error,
    },
    #[error("Could not create table '{table_name}'")]
    CouldNotCreateTable {
        table_name: String,
        source: rusqlite::Error,
    },
}
