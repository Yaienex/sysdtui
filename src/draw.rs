use std::rc::Rc;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use crate::{misc, App, Screen};
use figlet_rs::FIGfont;
use ratatui::style::Stylize;


pub fn draw_menu(f: &mut Frame, app: &mut App) {
    let size = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Percentage(90),
            Constraint::Percentage(10),
        ])
        .split(size);

    let items: Vec<ListItem> = misc::convert(&app);

    let list = List::new(items)
        .block(Block::default().title("  Menu principal  ").borders(Borders::ALL));

    f.render_stateful_widget(list, chunks[0], &mut app.state.clone());

    render_quit_button(f, app, chunks);
}

//Complete a basic form
pub fn draw_create_menu(f: &mut Frame, app: &mut App) {
    let size = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Percentage(90),
            Constraint::Percentage(10),
        ])
        .split(size);
    
    let items: Vec<ListItem> = misc::convert(&app);

    let list = List::new(items)
        .block(Block::default().title("  Menu principal  ").borders(Borders::ALL));

    f.render_stateful_widget(list, chunks[0], &mut app.state.clone());


    render_quit_button(f,app,chunks);

}

//Show the list of available service
pub fn draw_modify_menu(f: &mut Frame, app: &mut App) {
    let size = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Percentage(90),
            Constraint::Percentage(10),
        ])
        .split(size);

    let items: Vec<ListItem> = misc::convert(&app);

    let list = List::new(items)
        .block(Block::default().title("  Modify Menu  ").borders(Borders::ALL));

    f.render_stateful_widget(list, chunks[0], &mut app.state.clone());
    render_quit_button(f,app,chunks);
}

pub fn draw_status_menu(f: &mut Frame, app: &mut App) {

    let size = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Percentage(90),
            Constraint::Percentage(10),
        ])
        .split(size);
    let items: Vec<ListItem> = misc::convert(&app);

    let list = List::new(items)
        .block(Block::default().title("  Status Menu  ").borders(Borders::ALL));

    f.render_stateful_widget(list, chunks[0], &mut app.state.clone());
    render_quit_button(f,app,chunks);
}
pub fn draw_runtime_menu(f: &mut Frame, app: &mut App) {

    let size = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Percentage(90),
            Constraint::Percentage(10),
        ])
        .split(size);
    let items: Vec<ListItem> = misc::convert(&app);

    let list = List::new(items)
        .block(Block::default().title("  RunTime Menu  ").borders(Borders::ALL));

    f.render_stateful_widget(list, chunks[0], &mut app.state.clone());
    render_quit_button(f,app,chunks);
}


//-------------------------- SERVICES ----------------------
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
pub fn draw_create_service(f: &mut Frame, app: &App) {
    let size = f.area();
    let block = Block::default().title("Configurer le réseau").borders(Borders::ALL);
    let para = Paragraph::new("Formulaire de configuration réseau (placeholder).\n\n(Appuie sur ESC pour revenir)")
        .block(block)
        .style(Style::default().fg(Color::Green));
    f.render_widget(para, size);
}
pub fn draw_status_service(f: &mut Frame, app: &App) {
    let size = f.area();
    let block = Block::default().title(app.current_service.clone()).borders(Borders::ALL);
    let para = Paragraph::new("Formulaire de configuration réseau (placeholder).\n\n(Appuie sur ESC pour revenir)")
        .block(block)
        .style(Style::default().fg(Color::Green));
    f.render_widget(para, size);
}

pub fn draw_runtime_service(f: &mut Frame, app: &App) {
    let size = f.area();
    let block = Block::default().title("Configurer le réseau").borders(Borders::ALL);
    let para = Paragraph::new("Formulaire de configuration réseau (placeholder).\n\n(Appuie sur ESC pour revenir)")
        .block(block)
        .style(Style::default().fg(Color::Green));
    f.render_widget(para, size);
}

//---------------- Return / Quit Buttons--------------
fn render_quit_button(f: &mut Frame, app: &App, chunks: Rc<[Rect]>) {
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

//-----------------The exit frame --------------------
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