use std::io;

use app::App;
use event::Event;
use tui::Tui;
use update::handle_key_event;

/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Terminal user interface.
pub mod ui;

pub mod tui;

/// Event handler.
// pub mod handler;
pub mod update;

pub fn run_app(tui: &mut Tui, app: &mut App) -> eyre::Result<()> {
    // Start the main loop.
    while !app.should_quit {
        // Render the user interface.
        tui.draw(app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => handle_key_event(app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }
    Ok(())
}
// use crate::app::{App, AppResult};
// use crate::event::{Event, EventHandler};
// use crate::handler::handle_key_events;
// use crate::ui::Tui;
// use std::io;
// use tui::backend::CrosstermBackend;
// use tui::Terminal;

// pub fn try_main() -> AppResult<()> {
//     // Create an application.
//     let mut app = App::new();

//     // Initialize the terminal user interface.
//     let backend = CrosstermBackend::new(io::stderr());
//     let terminal = Terminal::new(backend)?;
//     let events = EventHandler::new(250);
//     let mut tui = Tui::new(terminal, events);
//     tui.init()?;

//     // Start the main loop.
//     while app.running {
//         // Render the user interface.
//         tui.draw(&mut app)?;
//         // Handle events.
//         match tui.events.next()? {
//             Event::Tick => app.tick(),
//             Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
//             Event::Mouse(_) => {}
//             Event::Resize(_, _) => {}
//         }
//     }

//     // Exit the user interface.
//     tui.exit()?;
//     Ok(())
// }
