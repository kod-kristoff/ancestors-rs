use std::io;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum ScrapeError {
    #[error(transparent)]
    Io {
        #[from]
        source: io::Error,
    },
    #[error("processing error")]
    Process {
        #[diagnostic_source]
        #[from]
        source: ProcessError,
    },
}
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum ProcessError {
    #[error("unknown field '{0}'")]
    UnknownField(String),
    #[error("unknown error: '{0}'")]
    UnknownError(String),
}
