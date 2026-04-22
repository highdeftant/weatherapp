use super::App;
use crate::weatherconv::label_from_hourly_string;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::*,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Axis, Block, Chart, Dataset, GraphType, Paragraph, Widget, Wrap},
};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let wmata_title = Line::from("WMATA Arrivals").bold();
        let hour_title = Line::from(vec![
            Span::raw("Next "),
            Span::raw(self.appinfo.next_hours.to_string())
                .yellow()
                .bold(),
            Span::raw(" Hour(s)"),
        ]);
        let chart_title = Line::from("Hourly Temp Chart").bold();

        let current_title = Line::from("Current").bold();
        let footer_title = Line::from("Footer").bold();
        let footer_body = Line::from("spoofy ent").bold();

        let instructions = Line::from(vec![
            Span::raw("Quit "),
            Span::raw("<Q>").blue().bold(),
            Span::raw("  Refresh "),
            Span::raw("<R>").blue(),
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
            .constraints([
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ])
            .split(main_layout[0]);

        // Render multi-station WMATA data with proper grouping
        let wmata_lines = &self.appinfo.wmata_arrivals;
        let wmata_body: Vec<Line> = wmata_lines
            .iter()
            .map(|line| {
                let styled = Line::from(line.as_str().bold());
                match line.as_str() {
                    // Title line
                    "WMATA Arrivals" => styled.yellow().bold(),
                    // Station header lines
                    l if l.starts_with("Station:") => styled.yellow().italic(),
                    // Help/info lines
                    l if l.contains("Set WMATA_API_KEY") || l.contains("no arrivals") => {
                        styled.red().dim()
                    }
                    // Error lines
                    l if l.contains("Error") || l.contains("failed") => styled.red(),
                    // Regular arrival lines
                    _ => styled,
                }
            })
            .collect();

        let hour_body: Vec<Line> = self
            .appinfo
            .hourly_time
            .iter()
            .map(|time| Line::from(time.as_str()))
            .collect();

        let current_temp = match self.appinfo.current_time.first() {
            Some(s) => s.as_str(),
            None => "--°",
        };
        let current_clock = match self.appinfo.current_time.get(1) {
            Some(s) => s.as_str(),
            None => "--:--:--",
        };

        let current_body = Line::from(vec![
            Span::raw(current_temp.to_string()),
            Span::raw(" at "),
            Span::raw(current_clock.to_string()),
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

        let hourly_points: Vec<(f64, f64)> = self
            .appinfo
            .hourly_temp
            .iter()
            .take(24)
            .enumerate()
            .map(|(i, value)| (i as f64, *value))
            .collect();

        if hourly_points.is_empty() {
            Paragraph::new("no hourly chart data")
                .left_aligned()
                .block(
                    Block::bordered()
                        .title(chart_title.left_aligned())
                        .border_set(border::THICK),
                )
                .render(weather_layout[2], buf);
        } else {
            // --- Y axis bounds (temperature) ---
            let mut y_min = f64::INFINITY;
            let mut y_max = f64::NEG_INFINITY;
            for (_, value) in &hourly_points {
                y_min = y_min.min(*value);
                y_max = y_max.max(*value);
            }

            if (y_max - y_min).abs() < f64::EPSILON {
                y_min -= 1.0;
                y_max += 1.0;
            } else {
                y_min -= 2.0;
                y_max += 2.0;
            }

            // --- X axis bounds ---
            let x_max = if hourly_points.len() > 1 {
                (hourly_points.len() - 1) as f64
            } else {
                1.0
            };

            // --- Y axis tick labels: floor/mean/ceil temperatures ---
            let y_ticks: Vec<(f64, String)> = {
                let step = (y_max - y_min) / 4.0;
                (0..=4)
                    .map(|i| {
                        let val = y_min + step * i as f64;
                        (val, format!("{:.0}°", val))
                    })
                    .collect()
            };

            // --- X axis tick labels: extract hour-of-day from hourly_time strings ---
            // hourly_time contains formatted strings like "72.5° at 14:00:00"
            let x_labels: Vec<String> = self
                .appinfo
                .hourly_time
                .iter()
                .take(hourly_points.len())
                .map(|s| label_from_hourly_string(s))
                .collect();

            let dataset = Dataset::default()
                .name("temp")
                .graph_type(GraphType::Line)
                .data(&hourly_points)
                .style(Style::default().cyan());

            Chart::new(vec![dataset])
                .x_axis(
                    Axis::default()
                        .bounds([0.0, x_max])
                        .title("Hour")
                        .labels(x_labels)
                        .style(Style::default().dark_gray()),
                )
                .y_axis(
                    Axis::default()
                        .bounds([y_min, y_max])
                        .title("Temp °F")
                        .labels(y_ticks.iter().map(|(_, l)| l.clone()).collect::<Vec<_>>())
                        .style(Style::default().dark_gray()),
                )
                .block(
                    Block::bordered()
                        .title(chart_title.left_aligned())
                        .border_set(border::THICK),
                )
                .render(weather_layout[2], buf);
        }

        Paragraph::new(wmata_body)
            .wrap(Wrap { trim: true })
            .left_aligned()
            .block(
                Block::bordered()
                    .title(wmata_title.left_aligned().yellow())
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
