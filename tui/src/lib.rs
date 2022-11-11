pub mod app;
pub mod event;
pub mod person;
pub mod ui;
// mod repl;

use crate::app::App;
use crate::event::Key;

// use repl::{get_command_type, get_config};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{backend::CrosstermBackend, Terminal};

// use crate::repl::CommandType;

pub fn try_main() -> eyre::Result<()> {
    log::trace!("called try_main");
    // let config = get_config();
    // let mut repl = rustyline::Editor::<()>::with_config(config)?;
    // loop {
    //     let readline = repl.readline(">> ");
    //     match readline {
    //         Ok(command) => {
    //             log::trace!("Command: {:?}", command);
    //             match get_command_type(&command) {
    //                 CommandType::PersonCommand(_) => todo!(),
    //             }
    //         }
    //         Err(ReadlineError::Interrupted) => {
    //             break;
    //         }
    //         Err(ReadlineError::Eof) => {
    //             break;
    //         }
    //         Err(err) => {
    //             log::error!("An error occured: {:?}", err);
    //             break;
    //         }
    //     }
    // }
    let app = App::new();
    start_ui(app)?;
    Ok(())
}

fn start_ui(mut app: App) -> eyre::Result<()> {
    log::trace!("starting ui ...");
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    let mut backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    let events = event::EventHandler::new(250);
    loop {
        terminal.draw(|mut f| ui::draw_main_layout(&mut f, &app));
        match events.next()? {
            event::Event::Input(key) => {
                if key == Key::Ctrl('c') {
                    break;
                }
            }
            event::Event::Tick => {
                app.update_on_tick();
            }
        }
    }
    close_application()?;
    Ok(())
}

fn close_application() -> eyre::Result<()> {
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    log::trace!("closing ui ...");
    Ok(())
}
