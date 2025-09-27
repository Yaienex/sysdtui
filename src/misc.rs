use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::{List, ListItem};
use crate::{App, Screen};

pub fn convert(app: &App) -> Vec<ListItem>{
    let mut items: Vec<ListItem> = app.items
        .iter()
        .enumerate()
        .map(|(i, text)| {
            if app.selected == i {
                ListItem::new(format!(">> {}", text))
                    .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            } else {
                ListItem::new(text.as_str())
            }
        })
        .collect();
    if app.screen != Screen::Menu{
        items.pop();
    }
    items.pop();
    items
}
