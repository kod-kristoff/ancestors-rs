use crate::app::{AppError, AppResult};
use crate::ui::Ui;

/// App runtime
pub struct Runtime {
    ui: Ui,
}

impl Runtime {
    /// Setup app runtime
    pub fn setup() -> AppResult<Self> {
        log::debug!("setting up app runtime");
        log::debug!("initializing ui");
        let ui = Ui::new()?;
        Ok(Self { ui })
    }

    /// Run app
    pub fn run(mut self) -> AppResult<()> {
        log::trace!("run");
        log::debug!("initializing terminal...");
        self.ui.init_terminal();
        log::debug!("finalizing terminal...");
        self.ui.finalize_terminal();
        Ok(())
    }
}
