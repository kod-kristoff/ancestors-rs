use tui::backend::Backend;
use tui::layout::Alignment;
use tui::style::{Color, Style};
use tui::terminal::Frame;
use tui::widgets::{Block, Borders, Paragraph};

/// Application result type.
pub type AppResult<T> = eyre::Result<T>;

/// AppState
#[derive(Debug, Copy, Clone)]
pub enum AppState {
    Main,
    AddPerson,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub state: AppState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            state: AppState::Main,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Renders the user interface widgets.
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        // This is where you add new widgets.
        // See the following resources:
        // - https://docs.rs/tui/0.16.0/tui/widgets/index.html
        // - https://github.com/fdehau/tui-rs/tree/v0.16.0/examples
        match self.state {
            AppState::Main => frame.render_widget(
                Paragraph::new("ancestors-tui")
                    .block(Block::default().borders(Borders::ALL))
                    .style(Style::default().fg(Color::White).bg(Color::Black))
                    .alignment(Alignment::Center),
                frame.size(),
            ),
            AppState::AddPerson => frame.render_widget(
                Paragraph::new("Add a person")
                    .block(Block::default().borders(Borders::ALL))
                    .style(Style::default().fg(Color::White).bg(Color::Black))
                    .alignment(Alignment::Center),
                frame.size(),
            ),
        }
    }
}
