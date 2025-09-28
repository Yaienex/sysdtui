use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Position};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::app::App;

pub fn draw_sudo_menu(f:&mut Frame, app: &mut App){
    let size = f.area();
    let chunks = Layout::default()
        .margin(2)
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .split(size);
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Red))
        .title("  Enter your sudo password  ");
    let subchunk = Layout::default()
        .margin(2)
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(40)])
        .split(chunks[0]);
    let para = Paragraph::new(app.cmd_status.clone());
    f.render_widget(para,subchunk[0]);


    f.render_widget(block,chunks[0]);
    f.set_cursor_position(Position::new(size.width/2, size.height/2));

}