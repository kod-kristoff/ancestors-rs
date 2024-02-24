pub mod components;

use ancestors_kernel::context::AppContext;
use ratatui::prelude::{Constraint, Direction, Layout};
use tui_input::Input;

use self::components::{
    menu::MenuComponent, person_editor::PersonEditorComponent, status::StatusComponent,
};
use crate::kx_tui::{Component, Frame};
use crate::{action::Action, config::Config, event::InputEvent, mode::Mode};

/// Application.
#[derive(Default)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    /// counter
    pub counter: u8,
    /// current screen
    // pub current_screen: CurrentScreen,
    pub filename: String,
    pub file_input: Input,
    pub ctx: AppContext,
}

pub struct AppComponent {
    config: Config,
    menu: MenuComponent,
    status: StatusComponent,
    person_editor: PersonEditorComponent,
    focus: Focus,
    mode: Mode,
    ctx: AppContext,
}

#[derive(Debug)]
pub enum Focus {
    Exiting,
    Loading,
    Main,
    Menu,
}

impl Default for Focus {
    fn default() -> Self {
        Self::Menu
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn load_file(&mut self, filename: String) {
        // self.current_screen = CurrentScreen::Main;
    }
    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}

impl AppComponent {
    pub fn new(config: Config, ctx: AppContext) -> Self {
        let menu = MenuComponent::new(config.clone());
        let person_editor = PersonEditorComponent::new(config.clone());
        Self {
            config,
            menu,
            status: StatusComponent::default(),
            person_editor,
            focus: Focus::default(),
            mode: Mode::default(),
            ctx,
        }
    }
}
impl Component for AppComponent {
    // pub fn new() -> AppComponent {
    //     Self::default()
    // }

    fn handle_event(&mut self, event: InputEvent) -> eyre::Result<Option<Action>> {
        if let Some(action) = self.components_event(event)? {
            return Ok(Some(action));
        }

        if let Some(action) = self.move_focus(event)? {
            return Ok(Some(action));
        }

        match event {
            InputEvent::Key(key) => {
                if let Some(keymap) = self.config.keybindings.get(&self.mode) {
                    if let Some(action) = keymap.get(&vec![key]) {
                        log::info!("Got action: {:?}", action);
                        return Ok(Some(action.clone()));
                    }
                }
            }
            _ => {}
        }
        // if matches!(event, InputEvent::Key(e) if e.code == KeyCode::Char('q')) {
        //     return Ok(EventState::NotConsumed);
        // }
        self.status
            .write_status(format!("Unhandled event: {:?}", event));
        Ok(None)
    }

    fn draw(
        &self,
        f: &mut Frame<'_>,
        window: ratatui::prelude::Rect,
        _in_focus: bool,
    ) -> eyre::Result<()> {
        // }
        // fn render(&self, f: &mut Frame) -> eyre::Result<()> {
        //     let window = f.size();
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Percentage(80),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(window);
        let (top, mid, bottom) = (main_chunks[0], main_chunks[1], main_chunks[2]);

        self.menu.draw(f, top, matches!(self.focus, Focus::Menu))?;
        match self.mode {
            Mode::AddPerson => {
                self.person_editor.draw(f, mid, true)?;
            }
            // Mode::Home => {
            //     self.
            // }
            _ => {}
        }
        self.status.draw(f, bottom, false)?;
        Ok(())
    }
    fn update(&mut self, action: Action) -> eyre::Result<Option<Action>> {
        match action {
            Action::AddPerson => {
                self.person_editor = PersonEditorComponent::new(self.config.clone());
                self.mode = Mode::AddPerson;
            }
            _ => {}
        }
        Ok(None)
    }
}
impl AppComponent {
    fn components_event(&mut self, event: InputEvent) -> eyre::Result<Option<Action>> {
        match self.mode {
            Mode::Menu => {
                if let Some(action) = self.menu.handle_event(event)? {
                    return Ok(Some(action));
                }
            }
            _ => {}
        }
        Ok(None)
    }

    fn move_focus(&mut self, event: InputEvent) -> eyre::Result<Option<Action>> {
        match self.mode {
            // Mode::Menu => {
            //     if matches!(event, InputEvent::Key(key) if key.code == KeyCode::Esc) {
            //         self.mode = self.menu.get_active();
            //     }
            // }
            _ => {}
        }
        Ok(None)
    }
}
#[cfg(test)]
mod tests;
