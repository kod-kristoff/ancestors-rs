use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
};
use ratatui::{layout::Alignment, widgets::BorderType};

use crate::{
    app::{App, CurrentScreen},
    tui::Frame,
};
pub mod components;
use self::components::centered_rect;

pub fn ui(app: &mut App, f: &mut Frame) {
    // Create the layout sections.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Create New Json",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);
    f.render_widget(title, chunks[0]);

    f.render_widget(
        Paragraph::new(format!(
            "
        Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
        Press `j` and `k` to increment and decrement the counter respectively.\n\
        Counter: {}
      ",
            app.counter
        ))
        .block(
            Block::default()
                .title("Counter App")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center),
        chunks[1],
    );

    let current_navigation_text = vec![
        // The first half of the text
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
            CurrentScreen::Loading => {
                Span::styled("Loading Mode", Style::default().fg(Color::Yellow))
            }
            CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
        }
        .to_owned(),
        // A white divider bar to separate the two sections
        Span::styled(" | ", Style::default().fg(Color::White)),
        // The final section of the text, with hints on what the user is editing
        {
            // if let Some(editing) = &app.currently_editing {
            //     match editing {
            //         CurrentlyEditing::Key => {
            //             Span::styled("Editing Json Key", Style::default().fg(Color::Green))
            //         }
            //         CurrentlyEditing::Value => {
            //             Span::styled("Editing Json Value", Style::default().fg(Color::LightGreen))
            //         }
            //     }
            // } else {
            Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))
            // }
        },
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "(q) to quit / (e) to make new pair",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Loading => Span::styled(
                "(ESC) to cancel/(Tab) to switch boxes/enter to complete",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exiting => Span::styled(
                "(q) to quit / (e) to make new pair",
                Style::default().fg(Color::Red),
            ),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[2]);

    f.render_widget(mode_footer, footer_chunks[0]);
    f.render_widget(key_notes_footer, footer_chunks[1]);

    if let CurrentScreen::Loading = &app.current_screen {
        let popup_block = Block::default()
            .title("Load a file")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let area = centered_rect(60, 25, f.size());
        f.render_widget(popup_block, area);

        let popup_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let mut key_block = Block::default().title("File").borders(Borders::ALL);
        let mut filename_block = Block::default().title("Filename").borders(Borders::ALL);

        let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

        // match editing {
        //     CurrentlyEditing::Key => key_block = key_block.style(active_style),
        //     CurrentlyEditing::Value => value_block = value_block.style(active_style),
        // };

        // let key_text = Paragraph::new(app.key_input.clone()).block(key_block);
        // f.render_widget(key_text, popup_chunks[0]);

        let filename_text = Paragraph::new(app.filename.clone()).block(filename_block);

        f.render_widget(filename_text, popup_chunks[1]);
    }
}
// use crate::app::{App, AppResult};
// use crate::event::EventHandler;
// use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
// use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
// use std::io;
// use tui::backend::Backend;
// use tui::Terminal;

// /// Representation of a terminal user interface.
// ///
// /// It is responsible for setting up the terminal,
// /// initializing the interface and handling the draw events.
// #[derive(Debug)]
// pub struct Tui<B: Backend> {
//     /// Interface to the Terminal.
//     terminal: Terminal<B>,
//     /// Terminal event handler.
//     pub events: EventHandler,
// }

// impl<B: Backend> Tui<B> {
//     /// Constructs a new instance of [`Tui`].
//     pub fn new(terminal: Terminal<B>, events: EventHandler) -> Self {
//         Self { terminal, events }
//     }

//     /// Initializes the terminal interface.
//     ///
//     /// It enables the raw mode and sets terminal properties.
//     pub fn init(&mut self) -> AppResult<()> {
//         terminal::enable_raw_mode()?;
//         crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;
//         self.terminal.hide_cursor()?;
//         self.terminal.clear()?;
//         Ok(())
//     }

//     /// [`Draw`] the terminal interface by [`rendering`] the widgets.
//     ///
//     /// [`Draw`]: tui::Terminal::draw
//     /// [`rendering`]: crate::app::App::render
//     pub fn draw(&mut self, app: &mut App) -> AppResult<()> {
//         self.terminal.draw(|frame| app.render(frame))?;
//         Ok(())
//     }

//     /// Exits the terminal interface.
//     ///
//     /// It disables the raw mode and reverts back the terminal properties.
//     pub fn exit(&mut self) -> AppResult<()> {
//         terminal::disable_raw_mode()?;
//         crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
//         self.terminal.show_cursor()?;
//         Ok(())
//     }
// }
