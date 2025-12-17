mod weather;
mod opmstatus;
mod app;

use app
use opmstatus::showopm;
use weather::{WeatherResponse, CurrentWeather, HourlyWeather, OpmStatus};
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use reqwest;

fn hourlyweather(hourly: &Vec<String>, temp: &Vec<f64>, local: &DateTime<Local>) {
    let datestring = "%Y-%m-%dT%H:%M";
    let temp_iter = temp.iter();
    let mut num = 0;
    let mut next = 0;
    // rewrite this to use the index, which will give temp
    println!("");
    println!("Hourly Forecast");
    println!("---------------");
    println!("");

    for hour in hourly {
        num += 1;
        // String -> NaiveDateTime
        let naivedate = NaiveDateTime::parse_from_str(hour, datestring)
            .expect("[ERR] Error parsing Vec<T> -> :");

        // NaiveDateTime -> DateTime<Local>
        let naivelocal = Local
            .from_local_datetime(&naivedate)
            .single()
            .expect("[ERR]: Invalid time input. -> :"); 

        // Compares DateTime to Local
        if naivelocal.date_naive() == local.date_naive() {
            if naivelocal.time() >= local.time() {
                next += 1;
                println!("{}° at {}", temp[num], naivelocal.time());
            }
        }
    }
    println!("");
    println!("---------------");
}

fn currentweather(_datetime: &str, _ctemp: &f64, _local: &DateTime<Local>) {}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    color_eyre::install()?;
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
    let local = chrono::Local::now();
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
    hourlyweather(&htime, &htemp, &local);
    showopm(&location, &shortmessage, &extendedinfo, &stat);

    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result

    Ok(())
}
