use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tui_input::backend::crossterm::EventHandler;

use crate::app::{App, CurrentScreen};

pub fn handle_key_event(app: &mut App, key_event: KeyEvent) {
    match app.current_screen {
        CurrentScreen::Main => match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => app.quit(),
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit()
                }
            }
            KeyCode::Right | KeyCode::Char('j') => app.increment_counter(),
            KeyCode::Left | KeyCode::Char('k') => app.decrement_counter(),
            KeyCode::Char('l') => {
                app.current_screen = CurrentScreen::Loading;
                app.filename = String::new();
            }
            _ => {}
        },
        CurrentScreen::Loading => match key_event.code {
            KeyCode::Esc => {
                app.current_screen = CurrentScreen::Main;
                // app.filename = String::new();
            }
            KeyCode::Enter => {
                app.load_file(app.file_input.to_string());
            }
            _ => app.file_input.handle_event(key_event.into()),
        },
        _ => match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => app.quit(),
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit()
                }
            }
            _ => {}
        },
    }
}
