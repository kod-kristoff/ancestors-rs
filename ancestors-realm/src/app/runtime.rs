use crate::app::{AppError, AppResult};

/// App runtime
pub struct Runtime {}

impl Runtime {
    /// Setup app runtime
    pub fn setup() -> AppResult<Self> {
        log::trace!("setup Runtime");
        Ok(Self {})
    }

    /// Run app
    pub fn run(mut self) -> AppResult<()> {
        log::trace!("run");
        Ok(())
    }
}
