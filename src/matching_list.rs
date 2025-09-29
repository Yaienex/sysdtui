use crossterm::event::KeyCode;
use crate::{ App, Screen};

pub fn menu_match(key_code: KeyCode, app: &mut App ) {
    general_match(key_code,app);
    match key_code{
        KeyCode::Char('p') => {
          app.open_popup();
        },
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
                app.current_action = app.items[app.selected].clone().to_lowercase();
                app.open_sudo();

            }

        }
        _ => {}
    }
}

fn general_match(key_code: KeyCode,app: &mut App) {
    match key_code {
        KeyCode::Char('q') => {
          app.screen = Screen::ExitMenu;
        },
        KeyCode::Esc => app.screen = Screen::Menu,
        KeyCode::Down => app.next(),
        KeyCode::Up => app.previous(),
        _ => {}
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


pub(crate) fn sudo_menu(key_code: KeyCode, app: &mut App) {
    match key_code{
        KeyCode::Esc => {
            app.close_sudo();
        }
        KeyCode::Enter => {
            let password = app.sudo_password.clone();
            app.sudo_password.clear();
            let (success, stdout,_stderr) = app.run_sudo_command(&password);
            if success{
                app.cmd_status = "Well done".to_string();
                app.open_popup();
                app.close_sudo();
                println!("{stdout}");
            } else {
                app.cmd_status = "Try Again".to_string();
            }
        },
        KeyCode::Char(c) => {
            app.cmd_status.clear();
            let s = c.to_string();
            app.sudo_password.push_str(s.as_str());
        },
        KeyCode::Backspace => {
            app.sudo_password.pop();
        }

        _ => {}
    }
}