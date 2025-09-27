use std::fmt::format;
use std::time::{Duration, Instant};
use crossterm::event::KeyCode;
use crate::{ App, Screen};

pub fn menu_match(key_code: KeyCode, app: &mut App ) {
    general_match(key_code,app);
    match key_code{
        KeyCode::Enter => {
            if app.selected == app.items.len() - 1 {
                app.quit();
            } else {
                match app.items[app.selected].as_str() {
                    "Create a service" => app.screen = Screen::CreateMenu,
                    "Modify a Service" => app.screen = Screen::ModifyMenu,
                    "Change status" => app.screen = Screen::RunTimeMenu,
                    "Check Status" => app.screen = Screen::StatusMenu,
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

pub fn create_menu_match(key_code: KeyCode, app: &mut App ) {
        general_match(key_code,app);
        match key_code{
            KeyCode::Enter => {
                enter_navigation_actions(app);
            }
            _ => {}
    }
}

pub fn modify_menu_match(key_code: KeyCode, app: &mut App ) {
    general_match(key_code,app);
    match key_code{
        KeyCode::Enter => {
           if ! enter_navigation_actions(app){
               app.current_service = app.items[app.selected].clone();
               app.screen = Screen::ModifyService
           }
        }
        _ => {}
    }
}

pub fn modify_service_match(key_code: KeyCode,app:&mut App){
    general_match(key_code,app);
    match key_code{
        KeyCode::Enter => {
            enter_navigation_actions(app);
        }
        _ => {}
    }
}

pub fn status_service_match(key_code: KeyCode,app:&mut App){
    general_match(key_code,app);
    match key_code{
        KeyCode::Enter => {
            enter_navigation_actions(app);
        }
        _ => {}
    }
}

pub fn status_menu_match(key_code: KeyCode,app:&mut App){
    general_match(key_code,app);
    match key_code{
        KeyCode::Enter => {
            enter_navigation_actions(app);
        }
        _ => {}
    }
}
pub fn runtime_menu_match(key_code: KeyCode,app:&mut App){
    general_match(key_code,app);
    match key_code{
        KeyCode::Enter => {
            if ! enter_navigation_actions(app){
                app.current_service = app.items[app.selected].clone();
                app.screen = Screen::RunTimeService

            }

        }
        _ => {}
    }
}
pub fn runtime_service_match(key_code: KeyCode,app:&mut App){
    general_match(key_code,app);
    match key_code{
        KeyCode::Enter => {
            if ! enter_navigation_actions(app){
                if ! app.show_sudo_modal {
                    
                    app.current_action = app.items[app.selected].clone().to_lowercase();
                    app.screen = Screen::SudoMenu;
                }
            }

        }
        _ => {}
    }
}

fn general_match(key_code: KeyCode,app: &mut App) {
    if app.show_sudo_modal {
        sudo_actions(key_code, app);
    } else {
        match key_code {
            KeyCode::Char('q') => {
                app.screen = Screen::ExitMenu;
            },
            KeyCode::Esc => app.screen = Screen::Menu,
            KeyCode::Down => app.next(),
            KeyCode::Up => app.previous(),
            KeyCode::Backspace => app.backspace(),
            KeyCode::Left => app.move_left(),
            KeyCode::Right => app.move_right(),
            _ => {}
        }
    }
}
fn enter_navigation_actions(app: &mut App) -> bool{
    if app.selected == app.items.len() -1 {
        app.screen = Screen::ExitMenu;
        return true;
    } else if app.selected == app.items.len() -2 {
       match app.screen{
           Screen::ModifyService => app.screen = Screen::ModifyMenu,
           Screen::RunTimeService => app.screen = Screen::RunTimeMenu,
           Screen::StatusService => app.screen = Screen::StatusMenu,
           _ => app.screen = Screen::Menu,
       }
        return true;
    }
    false
}

fn sudo_actions(key_code: KeyCode,app: &mut App){
        match key_code {
            KeyCode::Esc => app.close_sudo_modal(),
            KeyCode::Char(c) => app.sudo_password.push(c),
            KeyCode::Backspace => { app.sudo_password.pop(); },
            KeyCode::Enter => {
                let password = app.sudo_password.clone();
                let (success, stdout, stderr) = app.run_sudo_command(&password);
                if success {
                    app.message = "Command Succeeded".to_string();
                } else {
                    app.message = "Wrong password".to_string();
                }
                app.close_sudo_modal();
                app.popup_close_time = Some(Instant::now() + Duration::from_secs(4)); // 3 secondes
            }
            _ => {}
        }

}