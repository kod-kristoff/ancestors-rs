// use std::io;

// use action::Action;
// use app::AppComponent;
// use components::Component;
// use crossterm::event::KeyEvent;
use crate::event::Event;
// use tui::Tui;
// use update::handle_key_event;

// pub mod action;
// /// Application.
// pub mod app;
// pub mod config;
// /// Terminal events handler.
// pub mod event;
// pub mod mode;
// /// Terminal user interface.
// // pub mod ui;
// pub mod tui;

// pub mod components;
// /// Event handler.
// pub mod handler;
// pub mod update;
use crate::action::Action;

use super::{Component, Tui};

pub fn run_app(t: &mut Tui, app: &mut dyn Component) -> eyre::Result<()> {
    // Start the main loop.
    // while !app.should_quit {
    let mut should_quit = false;
    loop {
        // Render the user interface.
        t.terminal.draw(|f| {
            if let Err(err) = app.render(f) {
                todo!("handle render error {:?}", err);
            }
        })?;
        // Handle events.
        let mut maybe_action = match t.events.next()? {
            Event::Tick => Some(Action::Tick),
            Event::Input(event) => match app.handle_event(event) {
                Ok(action) => {
                    if !action.is_some() {
                        todo!("what to do here?");
                        Some(Action::Quit)
                    } else {
                        action
                    }
                }
                Err(_) => unimplemented!(),
            },
            Event::Resize(w, h) => Some(Action::Resize(w, h)),
        };

        while let Some(action) = maybe_action.take() {
            match action {
                Action::Quit => should_quit = true,
                Action::Resize(w, h) => t.terminal.resize(ratatui::prelude::Rect {
                    x: 0,
                    y: 0,
                    width: w,
                    height: h,
                })?,
                _ => {}
            }
            maybe_action = app.update(action)?;
        }

        if should_quit {
            break;
        }
    }
    Ok(())
}
