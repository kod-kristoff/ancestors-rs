mod error;
// pub mod model;
mod runtime;

pub use error::Error as AppError;
pub use runtime::Runtime;

pub type AppResult<T> = Result<T, AppError>;
