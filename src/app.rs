use std::io::{stdout, Write};
use std::process::{exit, Command, Stdio};
use std::thread::sleep;
use std::time::{Duration, Instant};
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use ratatui::widgets::ListState;
use crate::Screen;

pub struct App {
    pub(crate) items: Vec<String>,
    pub(crate) selected: usize,
    pub(crate) screen: Screen,
    pub(crate) prev_screen: Screen,
    pub(crate) close: bool,
    pub(crate) state: ListState,


    pub(crate) current_action: String,
    pub(crate) current_service: String,
    // modal sudo
    pub(crate) sudo_password: String,
    pub(crate) cmd_status: String,

    //Popups
    pub(crate) popup: bool,
    pub(crate) sudo_popup: bool,
    //Timers
    pub(crate) popup_close_time: core::option::Option<Instant>,

}

impl App {
    pub fn new(items: Vec<String>) -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        App {
            items,
            selected: 0,
            screen: Screen::Menu,
            prev_screen: Screen::Menu,
            close: false,
            state,
            current_service: "".to_string(),
            current_action: "".to_string(),
            sudo_password: String::new(),
            cmd_status: String::new(),
            popup: false,
            sudo_popup: false,
            popup_close_time : Some(Instant::now() + Duration::from_secs(3)),

        }

    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 { 0 } else { i + 1 }
            }
            None => 0,
        };
        self.state.select(Some(i));

        self.selected = i;
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 { self.items.len() - 1 } else { i - 1 }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.selected = i;
    }

    pub fn quit(&mut self) {
        sleep(Duration::from_secs(2));
        execute!(stdout(), Clear(ClearType::All)).unwrap();
        exit(0);
    }


    pub fn change_items(&mut self, items: Vec<String>) {
        self.items = items;
        if self.prev_screen != self.screen{
            self.selected =0;
            self.prev_screen = self.screen;
        }
    }

    pub fn run_sudo_command(&mut self, password: &str) -> (bool, String, String) {
         let mut child = match Command::new("sudo")
            .arg("-S")
            .arg("systemctl")
            .arg(self.items[self.selected].as_str().to_lowercase())
            .arg(self.current_service.clone())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(c) => c,
            Err(e) => {
                return (false, "".to_string(), format!("Error spawn sudo: {}", e));
            }
        };

        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(format!("{}\n", password).as_bytes());
            let _ = stdin.flush();
            drop(stdin);
        } else {
            return (false, "".to_string(), "Can't open stdin".to_string());
        }

        match child.wait_with_output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                (output.status.success(), stdout, stderr)
            }
            Err(e) => (false, "".to_string(), format!("Error waiting sudo: {}", e)),
        }
    }

    pub fn open_popup(&mut self){
        self.popup_close_time = Option::from(Instant::now() + Duration::from_secs(4));
        self.popup = true;
    }

    pub fn close_popup(&mut self){
        self.popup = false;
        self.popup_close_time = None;
    }

    pub fn close_sudo(&mut self){
        self.sudo_popup = false;
    }

    pub fn open_sudo(&mut self){
        self.sudo_popup = true;
    }
}