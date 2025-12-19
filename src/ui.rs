use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use reqwest;
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
    weatherinfo: String,
    exit: bool,
}


impl App {

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Box<dyn std::error::Error> {
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
            " <R> ".into(),
    ]);

        // Create Border line
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
        
        let weather_text = Text::from(vec![Line::from(vec![
            "Current Weather: ".into(),
            self.weatherinfo.to_string().yellow(),
    ])]);

        Paragraph::new(weather_text)
            .centered()
            .block(block)
            .render(area,buf);
    }
}
