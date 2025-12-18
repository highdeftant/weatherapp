mod ui;
mod getweather;
mod opmstatus;
mod weather;

use crate::{ui::App, getweather::{get_hourlyweather, get_currentweather, showopm}};
use color_eyre;
use ratatui;
use reqwest;
use std::io;
use weather::{CurrentWeather, HourlyWeather, OpmStatus, WeatherResponse};

//fn runterm() -> Result<()> {
//    color_eyre::install()?;
//    let mut terminal = ratatui::init();
//    let app_result = App::default().run(&mut terminal);
//    ratatui::restore();
//    app_result;
//    Ok(())
//}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
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

    //currentweather(&ctime, &ctemp, &local);
    get_hourlyweather(&htime, &htemp);
    showopm(&location, &shortmessage, &extendedinfo, &stat);
    Ok(())
}
