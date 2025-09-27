use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders};
use crate::app::App;
use super::super::render_quit_button;

pub fn draw_status_service(f: &mut Frame, app: &App) {
    let size = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Percentage(45),
            Constraint::Percentage(45),
            Constraint::Percentage(10),
        ])
        .split(size);
    let up_block = Block::default().title(app.current_service.clone()).borders(Borders::LEFT | Borders::RIGHT | Borders::TOP);
    let bot_block = Block::default().borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM);

    let subchunk = Layout::default()
        .direction(Direction::Horizontal)
        .margin(4)
        .constraints([Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(30)])
        .split(chunks[1]);

    f.render_widget(up_block, chunks[0]);
    render_quit_button(f,app, chunks);
}
