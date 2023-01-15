use crate::app::{App, AppResult, AppState};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match (app.state, key_event.code) {
        // exit application on ESC
        (AppState::Main, KeyCode::Esc) => {
            app.running = false;
        }
        // exit application on Ctrl-D
        (_, KeyCode::Char('d') | KeyCode::Char('D')) => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.running = false;
            }
        }
        (AppState::Main, KeyCode::Char('p') | KeyCode::Char('P')) => {
            if key_event.modifiers == KeyModifiers::NONE {
                app.state = AppState::AddPerson;
                log::info!("setting state to AddPerson");
            }
        }
        _ => {}
    }
    Ok(())
}
