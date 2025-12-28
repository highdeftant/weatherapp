use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget, Wrap},
    DefaultTerminal, Frame,
};


#[derive(Debug, Default)]
pub struct App {
    hourly_time: Vec<String>, 
    hourly_temp: Vec<f64>,
    current_time: String,
    opmstatus: String,
    opm: Vec<String>,
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
            _ => {},
        };
    }

    pub fn upd_opm(&mut self, opmstatus: Vec<String>) {
        self.opm = opmstatus;
    }

    pub fn upd_current(&mut self, time: Vec<String>) {
        self.current_time = time
    }

    pub fn upd_hours(&mut self, hours: Vec<String>, temp, Vec<f64>) {
        self.hourly_temp = hours;

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
        
        let weather_text = Text::from(vec![
            Line::from(self.opm[0].to_string().bold()),
            Line::from(self.opm[1].to_string().bold()),
            Line::from(self.opm[2].to_string().bold()),
            Line::from(self.opm[3].to_string().bold()),
        ]);
                

        Paragraph::new(weather_text)
            .wrap(Wrap {trim: true})
            .left_aligned()
            .block(block)
            .render(area,buf);
    }
}
