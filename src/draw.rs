use std::io::stdout;
use std::rc::Rc;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Modifier, Position, Style};
use ratatui::widgets::{Block, Borders, Clear, List, ListDirection, ListItem, Paragraph};
use crate::{misc, App, Screen};
use figlet_rs::FIGfont;
use ratatui::backend::ClearType;
use ratatui::style::Stylize;
use ratatui::text::Span;







//---------------- Return / Quit Buttons--------------

pub fn mdp(f: &mut Frame,app:&mut App) {
    let size = f.area();
    let block = Block::default()
        .title(" Sudo Password ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red));
    f.render_widget(block, size);

    let inner = Rect {
        x: size.x + 1,
        y: size.y + 1,
        width: size.width.saturating_sub(2),
        height: size.height.saturating_sub(2),
    };
    let chunks = Layout::default()
        .margin(40)
        .constraints([
            Constraint::Min(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10)
        ]).split(size);

    let pw_display = "*".repeat(app.sudo_password.chars().count());

    let lines = vec![
        Span::raw(app.message.clone()),
        Span::raw(""),
        Span::styled(
            pw_display.clone(),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
    ];

    for i in 0..lines.len(){
        let paragraph = Paragraph::new(lines[0].clone()).alignment(Alignment::Left);
        f.render_widget(paragraph, chunks[i]);
    }


    f.set_cursor_position(Position::new(inner.x + pw_display.len() as u16, inner.y + 2));
}

pub fn popup(f: &mut Frame, app : &mut App) {
    let size = f.area();
    let block = Block::default()
        .title("  Message  ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));

    let para = Paragraph::new(app.message.clone()).block(block.clone());
    f.render_widget(para, size);
    f.render_widget(block, size);


}
