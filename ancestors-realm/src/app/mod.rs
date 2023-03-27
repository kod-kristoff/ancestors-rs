mod error;
pub mod session;
// pub mod model;
mod runtime;

pub use self::error::Error as AppError;
pub use self::runtime::Runtime;
pub use self::session::Session;

pub type AppResult<T> = Result<T, AppError>;
