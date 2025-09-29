use std::rc::Rc;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::Paragraph;
use crate::app::App;

pub mod menu;
pub mod service;

pub(crate) fn render_quit_button(f: &mut Frame, app: &App, chunks: Rc<[Rect]>) {
    let mut quit_label = "Quitter   ".to_string();
    let mut quit_style = Style::default().fg(Color::White);
    if app.selected != app.items.len() -1{
        quit_label = " < Quitter >".to_string()
    } else {
        quit_style = Style::default().fg(Color::Red).add_modifier(Modifier::BOLD);
    };
    let mut rtn_label = "   Return".to_string();
    let mut rtn_style = Style::default().fg(Color::White);
    if app.selected != app.items.len() -2 {
        rtn_label = " < Return >".to_string();
    } else {
        rtn_style = Style::default().fg(Color::Green).add_modifier(Modifier::BOLD);
    }
    let quit = Paragraph::new(quit_label)
        .alignment(Alignment::Right)
        .style(quit_style);
    let rtn = Paragraph::new(rtn_label)
        .alignment(Alignment::Left)
        .style(rtn_style);
    let subchunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(50),Constraint::Fill(50)])
        .split(chunks[chunks.len() -1]);

    if app.screen != Screen::Menu{
        f.render_widget(rtn, subchunks[0]);
    }
    f.render_widget(quit, subchunks[1]);
}

#[derive(Debug, Clone, Copy,PartialEq)]
pub enum Screen {
    Menu,
    CreateMenu, // create a service
    ModifyMenu,
    ModifyService, // enable / disable + writing
    RunTimeMenu,
    RunTimeService,// start / stop / restart
    StatusMenu,
    StatusService, // status
    ExitMenu
}