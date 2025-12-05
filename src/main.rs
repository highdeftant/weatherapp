use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct WeatherResponse {
    latitude: f64,
    longitude: f64,
    current: CurrentWeather,
    hourly: HourlyWeather,
}

#[derive(Serialize, Deserialize, Debug)]
struct CurrentWeather {
    time: String,
    interval: i64,
    temperature_2m: f64,
    rain: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct HourlyWeather {
    time: Vec<String>,
    temperature_2m: Vec<f64>,
}

// OPM Status
#[derive(Serialize, Deserialize, Debug)]
struct OpmStatus {
    Location: String,
    ShortStatusMessage: String,
    ExtendedInformation: String,
    StatusType: String,
}

fn showopm(location: &str, shortmessage: &str, extendedinfo: &str, status: &str) {
    println!("---- OPM Status ----");
    println!("");
    println!("Location: {}", location);
    println!("Status: {}", status);
    println!("Information: {}", shortmessage);
    println!("Extended:{}", extendedinfo);
}

fn hourlyweather(hourly: &Vec<String>, temp: &Vec<f64>, local: &DateTime<Local>) {
    let datestring = "%Y-%m-%dT%H:%M";
    let temp_iter = temp.iter();
    let mut num = 0;

    // rewrite this to use the index, which will give temp
    println!("");
    println!("Hourly Forecast");
    println!("---------------");
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
                println!("{}° @ {}", temp[num], naivelocal.time());
            }
        }
    }
    println!("---------------");
}

fn currentweather(_datetime: &str, _ctemp: &f64, _local: &DateTime<Local>) {}

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

    let local = chrono::Local::now();
    let htime = weatherinfo.hourly.time;
    let htemp = weatherinfo.hourly.temperature_2m;
    let ctime = weatherinfo.current.time;
    let ctemp = weatherinfo.current.temperature_2m;


    let stat = opm_status.StatusType;
    let location = opm_status.Location;
    let shortmessage = opm_status.ShortStatusMessage;
    let extendedinfo = opm_status.ExtendedInformation;

    //currentweather(&ctime, &ctemp, &local);
    hourlyweather(&htime, &htemp, &local);
    showopm(&location, &shortmessage, &extendedinfo, &stat);

    Ok(())
}
