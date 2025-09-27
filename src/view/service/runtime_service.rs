use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::app::App;
use super::super::render_quit_button;

pub fn draw_runtime_service(f: &mut Frame, app: &App) {
    let size = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Min(5),
            Constraint::Percentage(10),
        ])
        .split(size);
    let up_block = Block::default().title(app.current_service.clone()).borders(Borders::LEFT | Borders::RIGHT | Borders::TOP);
    let bot_block = Block::default().borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM);

    let subchunk = Layout::default()
        .direction(Direction::Horizontal)
        .margin(4)
        .spacing(10)
        .constraints([Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25)])
        .split(chunks[1]);

    let mut labels :Vec<String>= vec![
        "  Start  ".to_string(),
        "  Stop  ".to_string(),
        "  Enable  ".to_string(),
        "  Disable  ".to_string()];
    match app.items[app.selected].as_str() {
        "Start" => {labels[0] = "< Start >".to_string()},
        "Stop" => {labels[1] = "< Stop >".to_string()},
        "Enable" => {labels[2] = "< Enable >".to_string()},
        "Disable" => {labels[3] = "< Disable >".to_string()},
        _ => {},
    }
    for i in 0..labels.len(){
        let para:Paragraph;
        if labels[i].clone().contains("<"){
            para = Paragraph::new(labels[i].clone())
                .style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Blue));
        }else {
            para = Paragraph::new(labels[i].clone());
        }
        f.render_widget(para, subchunk[i]);
    }

    //Borders rendering
    f.render_widget(bot_block, chunks[1]);
    f.render_widget(up_block, chunks[0]);
    render_quit_button(f,app, chunks);
}
