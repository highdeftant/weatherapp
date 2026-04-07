mod weatherconv;
mod ui;
mod weather;
mod weatherapi;
mod opmapi;
mod wmataapi;

use crate::{
    weatherconv::{get_current, get_hourly, showopm},
    ui::App,
};
use color_eyre;
use ratatui;
use reqwest;
use tokio;
use weather::{CurrentWeather, HourlyWeather, OpmStatus, WeatherResponse};
use weatherapi::get_weather;

pub struct Wmata {
    DestinationName: String,
    Line: String,
    Min: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let weatherendpoint = String::from("https://api.open-meteo.com/v1/forecast?latitude=38.8951&longitude=-77.0364&hourly=temperature_2m&current=temperature_2m,rain&timezone=America%2FNew_York&temperature_unit=fahrenheit");
    let opmendpoint = String::from("https://www.opm.gov/json/operatingstatus.json");

    let response = tokio::spawn(async move {
        get_weather(&weatherendpoint, 30).await;

    });

    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let mut app = App::default();

    loop {
   //     app.upd_current(current);
   //     app.upd_opm(opm);
   //     app.upd_hours(hours);
        let result = app.run(&mut terminal);
        ratatui::restore();
        result?;
        break Ok(())
    }

}
