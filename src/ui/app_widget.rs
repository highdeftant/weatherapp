use super::App;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::*,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget, Wrap},
};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let opm_title = Line::from("OPM Status").bold();
        let hour_title = Line::from(vec![
            "Next ".bold().into(),
            self.appinfo.next_hours.to_string().bold().into(),
            " Hour(s)".bold().into(),
        ]);

        let current_title = Line::from("Current").bold();
        let footer_title = Line::from("Footer").bold();
        let footer_body = Line::from("spoofy ent").bold();

        let instructions = Line::from(vec![
            " Quit ".into(),
            " <Q> ".blue().bold(),
            " Refresh ".into(),
            " <R> ".blue().into(),
        ]);

        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(3),
            ])
            .split(area);

        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
            .split(outer_layout[1]);

        let weather_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
            .split(main_layout[0]);

        let opm_body: Vec<Line> = self
            .appinfo
            .opm
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let line = Line::from(s.as_str().bold());
                if i == 0 {
                    line.italic().green()
                } else {
                    line
                }
            })
            .collect();

        let hour_body: Vec<Line> = self
            .appinfo
            .hourly_time
            .iter()
            .map(|time| Line::from(time.as_str()))
            .collect();

        let current_temp = match self.appinfo.current_time.get(0) {
            Some(s) => s.as_str(),
            None => "--°",
        };
        let current_clock = match self.appinfo.current_time.get(1) {
            Some(s) => s.as_str(),
            None => "--:--:--",
        };

        let current_body = Line::from(vec![
            current_temp.to_string().bold().into(),
            " at ".to_string().bold().into(),
            current_clock.to_string().bold().into(),
        ]);

        Paragraph::new(current_body)
            .wrap(Wrap { trim: true })
            .left_aligned()
            .block(
                Block::bordered()
                    .title(current_title.left_aligned())
                    .border_set(border::THICK),
            )
            .render(weather_layout[0], buf);

        Paragraph::new(hour_body)
            .wrap(Wrap { trim: true })
            .left_aligned()
            .block(
                Block::bordered()
                    .title(hour_title.left_aligned())
                    .border_set(border::THICK),
            )
            .render(weather_layout[1], buf);

        Paragraph::new(opm_body)
            .wrap(Wrap { trim: true })
            .left_aligned()
            .block(
                Block::bordered()
                    .title(opm_title.left_aligned().yellow())
                    .border_set(border::THICK),
            )
            .render(main_layout[1], buf);

        Paragraph::new(chrono::Local::now().date_naive().to_string())
            .wrap(Wrap { trim: true })
            .centered()
            .block(Block::bordered().border_set(border::THICK))
            .render(outer_layout[0], buf);

        Paragraph::new(footer_body)
            .wrap(Wrap { trim: true })
            .left_aligned()
            .block(
                Block::bordered()
                    .title(footer_title.centered())
                    .title_bottom(instructions.centered())
                    .border_set(border::THICK),
            )
            .render(outer_layout[2], buf);
    }
}
