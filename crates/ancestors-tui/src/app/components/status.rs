use ratatui::widgets::Paragraph;

use super::shared::container::render_container;
use crate::kx_tui::Component;

pub struct StatusComponent {
    status: String,
}

impl Default for StatusComponent {
    fn default() -> Self {
        Self {
            status: Default::default(),
        }
    }
}

impl StatusComponent {
    pub fn write_status<S: Into<String>>(&mut self, status: S) {
        self.status = status.into();
    }
}

impl Component for StatusComponent {
    fn draw(
        &self,
        f: &mut ratatui::Frame<'_>,
        area: ratatui::prelude::Rect,
        in_focus: bool,
    ) -> eyre::Result<()> {
        let status =
            Paragraph::new(self.status.clone()).block(render_container("Status", in_focus));
        f.render_widget(status, area);
        Ok(())
    }
}
