mod matching_list;
mod misc;
mod app;

mod view;

use view::{menu::*,service::*,Screen};
use app::App;
use std::io;
use std::process::Command;
use std::time::{Duration, Instant};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>)  {
    let main_menu_items = get_lists("menu");
    let mut app = App::new(main_menu_items.clone());

    let services_items = get_lists("service");
    let modify_items = get_lists("modify");
    let create_items = get_lists("create");
    let status_items = get_lists("status");
    let runtime_items = get_lists("runtime");
    loop {
        terminal.draw(|f| {
            match app.screen {
                    Screen::Menu =>{ app.change_items(main_menu_items.to_owned());
                        main_menu::draw_menu(f, &mut app)
                    },
                    Screen::CreateMenu => {
                        app.change_items(create_items.to_owned());
                        create_menu::draw_create_menu(f, &mut app);
                    },
                    Screen::ModifyMenu => {
                        app.change_items(services_items.to_owned());
                        modify_menu::draw_modify_menu(f, &mut app);
                    },
                    Screen::ModifyService => {
                        app.change_items(modify_items.to_owned());
                        modify_service::draw_modify_service(f, &mut app);
                    },
                    Screen::RunTimeMenu => {
                        app.change_items(services_items.to_owned());
                        runtime_menu::draw_runtime_menu(f, &mut app);
                    },
                    Screen::RunTimeService => {
                        app.change_items(runtime_items.to_owned());
                        runtime_service::draw_runtime_service(f, &mut app);
                    },
                    Screen::StatusMenu => {
                        app.change_items(services_items.to_owned());
                        status_menu::draw_status_menu(f, &mut app);
                    },
                    Screen::StatusService => {
                        app.change_items(status_items.to_owned());
                        status_service::draw_status_service(f, &mut app);
                    },
                    Screen::ExitMenu => exit_menu::draw_exit_menu(f, &mut app),
                }
            if app.popup{
                popup::draw_popup(f,&mut app);
            }
            else if app.sudo_popup{
                popup::draw_sudo_popup(f,&mut app);
            }
        }).unwrap();


        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                if app.sudo_popup{
                    matching_list::sudo_menu(key.code, &mut app);
                } else {
                    match app.screen {
                        Screen::Menu => matching_list::menu_match(key.code, &mut app),
                        Screen::CreateMenu => matching_list::create_menu_match(key.code, &mut app),
                        Screen::ModifyMenu => matching_list::modify_menu_match(key.code, &mut app),
                        Screen::ModifyService => matching_list::modify_service_match(key.code, &mut app),
                        Screen::RunTimeMenu => matching_list::runtime_menu_match(key.code, &mut app),
                        Screen::RunTimeService => matching_list::runtime_service_match(key.code, &mut app),
                        Screen::StatusMenu => matching_list::status_menu_match(key.code, &mut app),
                        Screen::StatusService => matching_list::status_service_match(key.code, &mut app),
                        _ => {}
                    }
                }
            }
        }
        if app.close {
            app.quit()
        }
        if let Some(close_time) = app.popup_close_time {
            if Instant::now() >= close_time {
               app.close_popup();
            }
        }
    }
}

fn get_lists(list: &str) -> Vec<String> {
    match list {
        "menu" =>   vec![
            "Create a service".to_string(),
            "Modify a Service".to_string(),
            "Change status".to_string(),
            "Check Status".to_string(),
            "Exit".to_string()
        ],
        "service" => {
            let cmd = Command::new("ls")
                .args(["/etc/systemd/system"])
                .output();
            let p = cmd.unwrap().stdout;
            let mut services = String::from_utf8_lossy(&p).to_string()
                .split('\n')
                .collect::<Vec<&str>>()
                .iter()
                .map(|s| {
                    let mut r = String::new();
                    if s.contains(".service") && !s.contains(".service.") {
                        r = s.to_string();
                    }
                    r
                }).filter(|s| s.len() > 0).collect::<Vec<String>>();
            services.append(&mut vec!["Return".to_string(), "Exit".to_string()]);
            services
        },
        "modify" => {
            vec!["Return".to_string(), "Exit".to_string()]
        },
        "create" => {
            vec!["Return".to_string(), "Exit".to_string()]
        },
        "status" => {
            vec!["Return".to_string(), "Exit".to_string()]
        },
        "runtime" => {
            vec!["Start".to_string(),
                 "Stop".to_string(),
                 "Enable".to_string(),
                 "Disable".to_string(),
                 "Return".to_string(),
                 "Exit".to_string()]
        },

        _ => vec!["".to_string()],
    }
}

