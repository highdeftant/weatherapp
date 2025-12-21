use std::io;
use std::collections::HashMap;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};


#[derive(Debug, Default)]
pub struct App {
    pub hourlyinfo: HashMap<String, String>, 
    pub currentinfo: String,
    pub opmstatus: String,
    exit: bool,
}


impl App {

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(()) 
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
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        };
    }
    pub fn hourly_forecast(&mut self, forecast: HashMap<String, String>) {
        for (temp, hour) in &forecast {
            self.hourlyinfo.insert(temp.to_string(), hour.to_string());
        }
    }
    fn exit(&mut self) {
        self.exit = true;
    }
}

// ANCHOR: Widget for App
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Weather App v0.1.0b ").bold();
        let instructions = Line::from(vec![
            " Quit ".into(),
            " <Q> ".blue().bold(),
            " Refresh ".into(),
            " <R> ".blue().into(),
    ]);

        // Create Border line
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
        
        let weather_text = Text::from(vec![Line::from(vec![
            "Current Weather: ".into(),
            self.hourlyinfo.into_string().yellow()
    ])]);

        Paragraph::new(weather_text)
            .centered()
            .block(block)
            .render(area,buf);
    }
}
