use std::time::Duration;
use tuirealm::{
    props::Shape, terminal::TerminalBridge, Application, EventListenerCfg, NoUserEvent,
};

mod error;

pub use error::UiError;

pub type UiResult<T> = Result<T, UiError>;

/// Application ID
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Id {
    // Menu(MenuId),
    None,
}

/// Application MSG
#[derive(PartialEq, Eq)]
pub enum Msg {
    None,
}
/// Current UI view
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum View {
    None,
}

/// Ancestors TUI
pub struct Ui {
    application: Application<Id, Msg, NoUserEvent>,
    terminal: TerminalBridge,
    view: View,
}

impl Ui {
    /// Create Ui
    pub fn new() -> UiResult<Self> {
        log::trace!("creating ui");
        let application = Application::init(
            EventListenerCfg::default().default_input_listener(Duration::from_millis(10)),
        );
        Ok(Self {
            application,
            terminal: TerminalBridge::new()?,
            view: View::None,
        })
    }

    /// Init terminal
    pub fn init_terminal(&mut self) {
        let _ = self.terminal.enable_raw_mode();
        let _ = self.terminal.enter_alternate_screen();
        let _ = self.terminal.clear_screen();
    }

    /// Finalize terminal
    pub fn finalize_terminal(&mut self) {
        let _ = self.terminal.disable_raw_mode();
        let _ = self.terminal.leave_alternate_screen();
        let _ = self.terminal.clear_screen();
    }
}
