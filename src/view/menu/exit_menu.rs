use figlet_rs::FIGfont;
use ratatui::Frame;
use ratatui::layout::Alignment;
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::app::App;

pub fn draw_exit_menu(f: &mut Frame, app: &mut App) {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("See Ya");
    let banner = figure.unwrap().to_string();

    let block = Block::default().borders(Borders::ALL).title("  Exiting sysdtui  ");
    let title = Paragraph::new(banner)
        .block(block)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Blue)).add_modifier(Modifier::BOLD);

    let size = f.area();

    f.render_widget(title, size);

    //after the last render we call the application
    app.close = true;

}
