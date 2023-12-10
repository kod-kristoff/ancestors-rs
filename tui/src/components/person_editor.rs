use ratatui::layout::{Layout, Direction, Constraint};

use crate::config::Config;

use super::Component;

pub struct PersonEditorComponent {
    config: Config,
}

impl PersonEditorComponent {
    pub fn new(config: Config) -> PersonEditorComponent {
        Self { config }
    }
}
impl Component for PersonEditorComponent {
    fn draw(
        &self,
        f: &mut ratatui::prelude::Frame<'_>,
        area: ratatui::prelude::Rect,
        in_focus: bool,
    ) -> eyre::Result<()> {
        let mut chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100),Constraint::Percentage(0)]).split(rect);
        todo!()
    }
}
