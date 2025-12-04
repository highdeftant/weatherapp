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

fn hourlyweather(hourly: &Vec<String>, temp: &Vec<f64>, local: &DateTime<Local>) {
    let datestring = "%Y-%m-%dT%H:%M";
    let temp_iter = temp.iter();

    for temps in temp {
        println!("{}", temps);
    }

    // rewrite this to use the index, which will give temp
    for hour in hourly {
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
                println!("{}", naivelocal.time());
            }
        }
    }
}

fn currentweather(_datetime: &str, _ctemp: &f64, _local: &DateTime<Local>) {}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let endpoint = String::from("https://api.open-meteo.com/v1/forecast?latitude=38.8951&longitude=-77.0364&hourly=temperature_2m&current=temperature_2m,rain&timezone=America%2FNew_York&temperature_unit=fahrenheit");

    let req: WeatherResponse = reqwest::Client::new()
        .get(endpoint)
        .send()
        .await?
        .json()
        .await?;

    let local = chrono::Local::now();
    let htime = req.hourly.time;
    let htemp = req.hourly.temperature_2m;
    let ctime = req.current.time;
    let ctemp = req.current.temperature_2m;

    //currentweather(&ctime, &ctemp, &local);
    hourlyweather(&htime, &htemp, &local);

    Ok(())
}
