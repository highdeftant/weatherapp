mod ui;
mod getweather;
mod weather;

use crate::{ui::App, getweather::{get_hourly, get_current, showopm}};
use color_eyre;
use ratatui;
use reqwest;
use weather::{CurrentWeather, HourlyWeather, OpmStatus, WeatherResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let weatherendpoint = String::from("https://api.open-meteo.com/v1/forecast?latitude=38.8951&longitude=-77.0364&hourly=temperature_2m&current=temperature_2m,rain&timezone=America%2FNew_York&temperature_unit=fahrenheit");
    let opmendpoint = String::from("https://www.opm.gov/json/operatingstatus.json");

    let weatherinfo: WeatherResponse = reqwest::Client::new()
        .get(weatherendpoint)
        .send()
        .await?
        .json()
        .await?;

    let opm_status: OpmStatus = reqwest::Client::new()
        .get(opmendpoint)
        .send()
        .await?
        .json()
        .await?;

    // Weather info
    let htime = weatherinfo.hourly.time;
    let htemp = weatherinfo.hourly.temperature_2m;
    let ctime = weatherinfo.current.time;
    let ctemp = weatherinfo.current.temperature_2m;

    // OPM Status
    let stat = opm_status.StatusType;
    let location = opm_status.Location;
    let shortmessage = opm_status.ShortStatusMessage;
    let extendedinfo = opm_status.ExtendedInformation;
    let opm = showopm(&location, &shortmessage, &extendedinfo, &stat);
    let hours = get_hourly(&htime, &htemp);
    //currentweather(&ctime, &ctemp, &local);

    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let mut app = App::default();
    app.upd_opm(opm);
    app.upd_hours(hours);
    let result = app.run(&mut terminal);
    ratatui::restore();
    result?;
    Ok(())
}
