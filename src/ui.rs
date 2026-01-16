use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use chrono::Local;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::*,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget, Wrap, LineGauge},
    DefaultTerminal, Frame,
};
use std::io;

#[derive(Debug, Default)]
pub struct App {
    next_hours: i32,
    hourly_time: Vec<String>, 
    hourly_temp: Vec<f64>,
    current_time: Vec<String>,
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
            _ => {}
        };
    }

    pub fn upd_opm(&mut self, opmstatus: Vec<String>) {
        self.opm = opmstatus;
    }

    pub fn upd_current(&mut self, time: Vec<String>) {
        self.current_time = time
    }

    pub fn upd_hours(&mut self, hour_temp: (Vec<String>, Vec<f64>, i32)) { 
        self.hourly_time = hour_temp.0;
        self.hourly_temp = hour_temp.1;
        self.next_hours = hour_temp.2;

    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

// ANCHOR: Widget for App
impl Widget for &App {

    fn render(self, area: Rect, buf: &mut Buffer) {
        let opm_title = Line::from("OPM Status").bold();
        let next_hours = Line::from(format!("Next {} hours", self.next_hours));
        let hour_title = Line::from("Hourly").bold();
        let current_title = Line::from("Current").bold();
        let news_title = Line::from("News").bold();
        let header_title = Line::from("Header").bold();
        let dashboard_title = Line::from("Dashboard").bold();
        let footer_title = Line::from("Footer").bold();

        let footer_body = Line::from("spoofy ent").bold();

        let instructions = Line::from(vec![
            " Quit ".into(),
            " <Q> ".blue().bold(),
            " Refresh ".into(),
            " <R> ".blue().into(),
        ]);

        // Main Layout
        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(10), // Main Container
                Constraint::Length(3), // Footer
            ])
            .split(area);

        // Inner Layout
        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Ratio(1, 2), // Left Side
                Constraint::Ratio(1, 2), // Right Side
            ])
            .split(outer_layout[1]);

        let weather_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Ratio(1, 2), // Top
                Constraint::Ratio(1, 2), // Bottom
            ])
            .split(main_layout[0]);
        //
        // OPM Status Information
        let opm_body = Text::from(vec![
            Line::from(self.opm[0].to_string().bold()).italic().green(),
            Line::from(self.opm[1].to_string().bold()),
            Line::from(self.opm[2].to_string().bold()),
            Line::from(self.opm[3].to_string().bold()),
        ]);

        let hour_body: Vec<Line> = self.hourly_time
            .iter()
            .map(|time| Line::from(time.as_str()))
            .collect();

        let current_body = Line::from(vec![
            self.current_time[0].to_string().bold().into(),
            " at ".to_string().bold().into(),
            self.current_time[1].to_string().bold().into(),]);


        // Layout Information
        //
        // Body
        let currentweather = Paragraph::new(current_body)
            .wrap(Wrap {trim: true})
            .left_aligned()
            .block(
                Block::bordered()
                .title(current_title.left_aligned())
                .border_set(border::THICK)
            )
            .render(weather_layout[0], buf);

        let hourlyweather = Paragraph::new(hour_body)
            .wrap(Wrap {trim: true})
            .left_aligned()
            .block(
                Block::bordered()
                .title(hour_title.left_aligned())
                .border_set(border::THICK)
            )
            .render(weather_layout[1], buf);

        let opmblock = Paragraph::new(opm_body)
            .wrap(Wrap {trim: true})
            .left_aligned()
            .block(
                Block::bordered()
                .title(opm_title.left_aligned().yellow())
                .border_set(border::THICK)
            )
            .render(main_layout[1], buf);

        
        // Header, Footer
        
        let header = Paragraph::new(chrono::Local::now().date_naive().to_string())
            .wrap(Wrap {trim: true})
            .centered()
            .block(
                Block::bordered()
                .border_set(border::THICK)
            )
            .render(outer_layout[0], buf);
        
        let footer = Paragraph::new(footer_body)
            .wrap(Wrap {trim: true})
            .left_aligned()
            .block(
                Block::bordered()
                .title(footer_title.centered())
                .title_bottom(instructions.centered())
                .border_set(border::THICK)
            )
            .render(outer_layout[2], buf);
    }
}
