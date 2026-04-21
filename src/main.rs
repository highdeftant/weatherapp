mod api;
mod ui;
mod weather;
mod weatherconv;

use crate::{
    api::wmataapi::status_lines_from_env,
    ui::App,
    weather::WeatherResponse,
    weatherconv::{get_chart_data, get_current, get_hourly},
};
use color_eyre;
use ratatui;
use tokio::time::{Duration, interval};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let weatherendpoint = "https://api.open-meteo.com/v1/forecast?latitude=38.8951&longitude=-77.0364&hourly=temperature_2m&current=temperature_2m,rain&timezone=America%2FNew_York&temperature_unit=fahrenheit";

    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let mut app = App::default();

    app.upd_opm(vec![
        "Station: --".to_string(),
        "Set WMATA_API_KEY to enable live arrivals".to_string(),
    ]);

    let client = reqwest::Client::new();
    let mut refresh = interval(Duration::from_secs(30));

    loop {
        tokio::select! {
            _ = refresh.tick() => {
                match client
                    .get(weatherendpoint)
                    .send()
                    .await?
                    .json::<WeatherResponse>()
                    .await
                {
                    Ok(weather) => {
                        let current = get_current(&weather.current.time, &weather.current.temperature_2m);
                        let hourly = get_hourly(&weather.hourly.time, &weather.hourly.temperature_2m);
                        let chart = get_chart_data(&weather.hourly.time, &weather.hourly.temperature_2m);
                        app.upd_current(current);
                        app.upd_hours(hourly);
                        app.upd_chart_hours(chart.hour_labels);

                        let wmata_lines = status_lines_from_env(&client, 6).await;
                        app.upd_opm(wmata_lines);
                    }
                    Err(_) => {
                        // keep last good UI state on fetch/parse errors
                    }
                }
            }
            _ = tokio::time::sleep(Duration::from_millis(75)) => {
                app.tick(&mut terminal)?;
                if app.should_exit() {
                    ratatui::restore();
                    break Ok(());
                }
            }
        }
    }
}
