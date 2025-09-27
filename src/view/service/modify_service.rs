use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders};
use crate::app::App;
use super::super::render_quit_button;

pub fn draw_modify_service(f: &mut Frame, app: &mut App) {
    let size = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Percentage(90),
            Constraint::Percentage(10),
        ])
        .split(size);
    let block = Block::default().title(app.current_service.clone()).borders(Borders::ALL);

    f.render_widget(block, chunks[0]);
    render_quit_button(f,app, chunks);
}