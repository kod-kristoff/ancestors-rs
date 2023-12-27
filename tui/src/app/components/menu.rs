use crossterm::event::KeyCode;
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Tabs,
    Frame,
};

use crate::{action::Action, config::Config, event::EventState, mode::Mode};

use super::shared::{constants::HIGHLIGHT_COLOR, container::render_container};
use crate::kx_tui::Component;

#[derive(Copy, Clone, Debug)]
pub enum MenuItem {
    Home,
    Persons,
    // Layout,
}

impl Default for MenuItem {
    fn default() -> Self {
        Self::Home
    }
}
impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Persons => 1,
            // MenuItem::Layout => 2,
        }
    }
}
pub struct MenuComponent {
    active_menu_item: MenuItem,
    config: Config,
}

impl MenuComponent {
    pub fn new(config: Config) -> Self {
        Self {
            active_menu_item: MenuItem::default(),
            config,
        }
    }
}

impl MenuComponent {
    pub fn set_active(&mut self, menu_item: MenuItem) {
        self.active_menu_item = menu_item;
    }
    pub fn get_active(&self) -> Mode {
        match self.active_menu_item {
            MenuItem::Home => Mode::Home,
            MenuItem::Persons => todo!(),
        }
    }
}

impl Component for MenuComponent {
    fn handle_key_event(
        &mut self,
        key: crossterm::event::KeyEvent,
    ) -> eyre::Result<Option<Action>> {
        // if key.code == KeyCode::Char('h') {
        //     self.set_active(MenuItem::Home);
        //     return Ok(EventState::Consumed);
        // }
        Ok(None)
    }
    fn draw(
        &self,
        f: &mut Frame<'_>,
        area: ratatui::prelude::Rect,
        in_focus: bool,
    ) -> eyre::Result<()> {
        let menu_titles = vec!["Ancestors", "Home", "Persons", "Quit"];

        let menu = menu_titles
            .iter()
            .enumerate()
            .map(|(index, t)| {
                if index == 0 {
                    Line::from(vec![Span::styled(*t, Style::default().fg(HIGHLIGHT_COLOR))])
                } else {
                    let (first, rest) = t.split_at(1);
                    Line::from(vec![
                        Span::styled(
                            first,
                            Style::default()
                                .fg(HIGHLIGHT_COLOR)
                                .add_modifier(Modifier::UNDERLINED),
                        ),
                        Span::styled(rest, Style::default().fg(Color::White)),
                    ])
                }
            })
            .collect();

        let tabs = Tabs::new(menu)
            .select(self.active_menu_item.into())
            .block(render_container("Menu", in_focus))
            .divider(Span::raw("|"));

        f.render_widget(tabs, area);
        Ok(())
    }
}
