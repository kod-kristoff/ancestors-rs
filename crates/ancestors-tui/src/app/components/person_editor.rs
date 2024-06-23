use ancestors_kernel::gen_services::services::UpsertPerson;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Cell, Row, Table};

use crate::app::components::shared::container::render_container;
use crate::config::Config;

use crate::kx_tui::widgets::StatefulTable;
use crate::kx_tui::Component;

use super::shared::constants::HIGHLIGHT_COLOR;

pub struct PersonEditorComponent {
    person_info: UpsertPerson,
    config: Config,
    table: StatefulTable,
}

impl PersonEditorComponent {
    pub fn new(config: Config) -> PersonEditorComponent {
        let person_info = UpsertPerson::default();
        let table = build_table(&person_info);
        Self {
            person_info,
            config,
            table,
        }
    }

    fn generate_label(&self) -> String {
        "New person".into()
    }
}
fn build_table(pi: &UpsertPerson) -> StatefulTable {
    let name = if let Some(name) = pi.name.as_ref() {
        name.clone()
    } else {
        "".into()
    };
    let fields = vec![vec!["name:".into(), name]];
    StatefulTable::default()
        .with_items(fields)
        .with_headers(vec!["Field", "Value"])
        .build()
}
impl Component for PersonEditorComponent {
    fn draw(
        &self,
        f: &mut ratatui::prelude::Frame<'_>,
        area: ratatui::prelude::Rect,
        in_focus: bool,
    ) -> eyre::Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100), Constraint::Percentage(0)])
            .split(area);
        let header_cells = self
            .table
            .headers
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::Black)));
        let normal_style = Style::default().bg(Color::DarkGray);
        let header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);
        let rows = self.table.items.iter().map(|item| {
            let height = item
                .iter()
                .map(|content| content.chars().filter(|c| *c == '\n').count())
                .max()
                .unwrap_or(0)
                + 1;
            let cells = item.iter().map(|c| Cell::from(c.clone()));

            Row::new(cells).height(height as u16).bottom_margin(1)
        });
        let label = self.generate_label();
        let widths = &[
            // Constraint::Percentage(5),
            Constraint::Percentage(35),
            Constraint::Percentage(60),
        ];
        let table = Table::new(rows, widths)
            .header(header)
            .block(render_container(&label, in_focus))
            .highlight_style(
                Style::default()
                    .bg(HIGHLIGHT_COLOR)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            );
        f.render_stateful_widget(table, chunks[0], &mut self.table.state.clone());
        // todo!()
        Ok(())
    }
}
