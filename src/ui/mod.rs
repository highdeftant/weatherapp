mod app_widget;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use std::{io, time::Duration};

#[derive(Debug, Default)]
pub struct App {
    appinfo: AppInfo,
    exit: bool,
}

#[derive(Debug, Default)]
pub struct AppInfo {
    next_hours: i32,
    hourly_time: Vec<String>,
    hourly_temp: Vec<f64>,
    current_time: Vec<String>,
    wmata_arrivals: Vec<String>,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn tick(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        terminal.draw(|frame| self.draw(frame))?;
        self.handle_events_nonblocking()?;
        Ok(())
    }

    pub fn should_exit(&self) -> bool {
        self.exit
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_events_nonblocking(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    self.handle_key_event(key_event);
                }
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if let KeyCode::Char('q') = key_event.code {
            self.exit();
        }
    }

    pub fn upd_wmata_arrivals(&mut self, wmatastatus: Vec<String>) {
        self.appinfo.wmata_arrivals = wmatastatus;
    }

    pub fn upd_current(&mut self, time: Vec<String>) {
        self.appinfo.current_time = time
    }

    pub fn upd_hours(&mut self, hour_temp: (Vec<String>, Vec<f64>, i32)) {
        self.appinfo.hourly_time = hour_temp.0;
        self.appinfo.hourly_temp = hour_temp.1;
        self.appinfo.next_hours = hour_temp.2;
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
