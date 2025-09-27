use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, List, ListItem};
use crate::app::App;
use crate::misc;
use super::super::render_quit_button;

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
     render_quit_button(f, app, chunks);
}
