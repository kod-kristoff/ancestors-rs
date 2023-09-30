use tui_input::Input;

/// Application.
#[derive(Debug, Default)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    /// counter
    pub counter: u8,
    /// current screen
    pub current_screen: CurrentScreen,
    pub filename: String,
    pub file_input: Input,
}

#[derive(Debug)]
pub enum CurrentScreen {
    Main,
    Loading,
    Exiting,
}

impl Default for CurrentScreen {
    fn default() -> Self {
        Self::Loading
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
        self.current_screen = CurrentScreen::Main;
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
// use tui::backend::Backend;
// use tui::layout::Alignment;
// use tui::style::{Color, Style};
// use tui::terminal::Frame;
// use tui::widgets::{Block, Borders, Paragraph};

// /// Application result type.
// pub type AppResult<T> = eyre::Result<T>;

// /// AppState
// #[derive(Debug, Copy, Clone)]
// pub enum AppState {
//     Main,
//     AddPerson,
// }

// /// Application.
// #[derive(Debug)]
// pub struct App {
//     /// Is the application running?
//     pub running: bool,
//     pub state: AppState,
// }

// impl Default for App {
//     fn default() -> Self {
//         Self {
//             running: true,
//             state: AppState::Main,
//         }
//     }
// }

// impl App {
//     /// Constructs a new instance of [`App`].
//     pub fn new() -> Self {
//         Self::default()
//     }

//     /// Handles the tick event of the terminal.
//     pub fn tick(&self) {}

//     /// Renders the user interface widgets.
//     pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
//         // This is where you add new widgets.
//         // See the following resources:
//         // - https://docs.rs/tui/0.16.0/tui/widgets/index.html
//         // - https://github.com/fdehau/tui-rs/tree/v0.16.0/examples
//         match self.state {
//             AppState::Main => frame.render_widget(
//                 Paragraph::new("ancestors-tui")
//                     .block(Block::default().borders(Borders::ALL))
//                     .style(Style::default().fg(Color::White).bg(Color::Black))
//                     .alignment(Alignment::Center),
//                 frame.size(),
//             ),
//             AppState::AddPerson => frame.render_widget(
//                 Paragraph::new("Add a person")
//                     .block(Block::default().borders(Borders::ALL))
//                     .style(Style::default().fg(Color::White).bg(Color::Black))
//                     .alignment(Alignment::Center),
//                 frame.size(),
//             ),
//         }
//     }
// }
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_app_increment_counter() {
        let mut app = App::default();
        app.increment_counter();
        assert_eq!(app.counter, 1);
    }

    #[test]
    fn test_app_decrement_counter() {
        let mut app = App::default();
        app.decrement_counter();
        assert_eq!(app.counter, 0);
    }
}
