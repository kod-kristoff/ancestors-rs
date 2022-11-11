use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders},
    Frame,
};

use crate::app::App;

pub fn draw_main_layout<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    // let margin = 5;

    let parent_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(6)].as_ref())
        // .margin(margin)
        .split(f.size());

    draw_routes(f, app, parent_layout[0]);
    draw_canvas(f, app, parent_layout[1]);
}

pub fn draw_routes<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let block = Block::default().title("Block").borders(Borders::ALL);
    f.render_widget(block, layout_chunk);
}

pub fn draw_canvas<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let block = Block::default().title("Canvas").borders(Borders::ALL);
    f.render_widget(block, layout_chunk);
}
