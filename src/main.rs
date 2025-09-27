mod matching_list;
mod misc;
mod draw;

use draw::*;
use std::io;
use std::io::stdout;
use std::process::{exit, Command};
use std::thread::sleep;
use std::time::Duration;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use crossterm::terminal::{Clear, ClearType};
use ratatui::{backend::CrosstermBackend, Terminal};
use ratatui::widgets::ListState;

#[derive(Debug, Clone, Copy,PartialEq)]
enum Screen {
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

struct App {
    items: Vec<String>,
    selected: usize,
    screen: Screen,
    prev_screen: Screen,
    close: bool,
    state: ListState,
    cursor_pos: usize,
    input: String,
    current_service: String,
}

impl App {
    fn new(items: Vec<String>) -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        App {
            items,
            selected: 0,
            screen: Screen::Menu,
            prev_screen: Screen::Menu,
            close: false,
            state,
            cursor_pos: 0,
            input :"".to_string(),
            current_service: "".to_string(),
        }

    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 { 0 } else { i + 1 }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.selected = i;
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 { self.items.len() - 1 } else { i - 1 }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.selected = i;
    }


    fn insert_char(&mut self, c: char) {
        self.input.insert(self.cursor_pos, c);
        self.cursor_pos += 1;
    }

    fn backspace(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.input.remove(self.cursor_pos);
        }
    }

    fn move_left(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }
    }

    fn move_right(&mut self) {
        if self.cursor_pos < self.input.len() {
            self.cursor_pos += 1;
        }
    }

    fn quit(&mut self) {
        sleep(Duration::from_secs(2));
        execute!(stdout(), Clear(ClearType::All)).unwrap();
        exit(0);
    }

    fn change_items(&mut self, items: Vec<String>) {
        self.items = items;
        if self.prev_screen != self.screen{
            self.selected =0;
            self.prev_screen = self.screen;
        }
    }

}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

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
                    draw_menu(f, &mut app)
                },
                Screen::CreateMenu => {
                    app.change_items(create_items.to_owned());
                    draw_create_menu(f, &mut app);
                },
                Screen::ModifyMenu => {
                    app.change_items(services_items.to_owned());
                    draw_modify_menu(f, &mut app);
                },
                Screen::ModifyService => {
                    app.change_items(modify_items.to_owned());
                    draw_modify_service(f, &mut app);
                },
                Screen::RunTimeMenu => {
                    app.change_items(services_items.to_owned());
                    draw_runtime_menu(f, &mut app);
                },
                Screen::RunTimeService => {
                    app.change_items(runtime_items.to_owned());
                    draw_runtime_service(f, &mut app);
                },
                Screen::StatusMenu => {
                    app.change_items(services_items.to_owned());
                    draw_status_menu(f, &mut app);
                },
                Screen::StatusService => {
                    app.change_items(status_items.to_owned());
                    draw_status_service(f, &mut app); },
                Screen::ExitMenu => draw_exit_menu(f, &mut app),
            }
        }).unwrap();

        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                match app.screen {
                    Screen::Menu => matching_list::menu_match(key.code,&mut app),
                    Screen::CreateMenu => matching_list::create_menu_match(key.code,&mut app),
                    Screen::ModifyMenu=> matching_list::modify_menu_match(key.code,&mut app),
                    Screen::ModifyService => matching_list::modify_service_match(key.code,&mut app),
                    Screen::RunTimeMenu => matching_list::runtime_menu_match(key.code,&mut app),
                    Screen::StatusMenu =>matching_list::status_menu_match(key.code,&mut app),
                    _ => {}
                }
            }
        }
        if app.close {
            app.quit()
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
            vec!["Return".to_string(), "Exit".to_string()]
        },

        _ => vec!["".to_string()],
    }
}

