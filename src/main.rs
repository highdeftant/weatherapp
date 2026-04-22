mod api;
mod ui;
mod weather;
mod weatherconv;

use crate::{
    api::wmataapi::{fetch_all_stations, load_config_from_env},
    ui::App,
    weather::WeatherResponse,
    weatherconv::{get_current, get_hourly},
};
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let weatherendpoint = "https://api.open-meteo.com/v1/forecast?latitude=38.8951&longitude=-77.0364&hourly=temperature_2m&current=temperature_2m,rain&timezone=America%2FNew_York&temperature_unit=fahrenheit";

    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let mut app = App::default();

    app.upd_wmata_arrivals(vec![
        "WMATA Arrivals".to_string(),
        "Set WMATA_API_KEY to enable live arrivals".to_string(),
    ]);

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;
    let mut refresh = interval(Duration::from_secs(30));

    loop {
        tokio::select! {
            _ = refresh.tick() => {
                match client
                    .get(weatherendpoint)
                    .send()
                    .await
                {
                    Ok(resp) => match resp.json::<WeatherResponse>().await {
                        Ok(weather) => {
                            let current = get_current(&weather.current.time, &weather.current.temperature_2m);
                            let hourly = get_hourly(&weather.hourly.time, &weather.hourly.temperature_2m);
                            app.upd_current(current);
                            app.upd_hours(hourly);
                        }
                        Err(_) => {
                            // keep last good UI state on parse errors
                        }
                    },
                    Err(_) => {
                        // keep last good UI state on network errors
                    }
                }

                // fetch WMATA arrivals for all configured stations
                let config = load_config_from_env(0);
                let wmata_lines = fetch_all_stations(&client, &config).await;
                app.upd_wmata_arrivals(wmata_lines);
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
