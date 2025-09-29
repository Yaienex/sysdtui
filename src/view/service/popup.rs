
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Flex, Layout,  Rect};
use ratatui::style::{Color, Modifier, Style};
use crate::App;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

pub fn draw_popup(f: &mut Frame, app :&mut App){
    let area = center(
        f.area(),
        Constraint::Percentage(20),
        Constraint::Length(5), // top and bottom border + content
    );
    let block = Block::default().borders(Borders::ALL).title("  Popup !   ")
        .style(Style::default().fg(Color::Green)).title_alignment(Alignment::Center);
    let msg = format!(" {}\n  Succeeded to {}", app.current_service.clone(),app.current_action.clone());
    let popup = Paragraph::new(msg).block(block)
        .style(Style::default().fg(Color::Green));

    f.render_widget(Clear, area); //th
    f.render_widget(popup,area);
}

pub fn draw_sudo_popup(f: &mut Frame, app: &mut App){
    let area = center(
        f.area(),
        Constraint::Percentage(20),
        Constraint::Length(5), // top and bottom border + content
    );
    let block = Block::default().borders(Borders::ALL).title("  PASSWORD !  ")
        .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .title_alignment(Alignment::Center);

    let popup = Paragraph::new(app.cmd_status.clone())
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Red))
        .block(block);

    f.render_widget(Clear, area); //th
    f.render_widget(popup,area);

}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}