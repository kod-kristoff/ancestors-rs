mod error;
pub mod service;

pub use self::error::UseCaseError;
pub type UseCaseResult<T> = Result<T, UseCaseError>;
