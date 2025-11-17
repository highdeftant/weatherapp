use chrono::{Local, DateTime, Utc};
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

fn hourlyweather(_hourly: &Vec<String>, _temp: &Vec<f64>) {
    let _local_now: DateTime<Local> = chrono::Local::now();
    let _utc_now =  Utc::now();
}

fn currentweather(datetime: &str, temp: &f64) {
    let date = &datetime[0..10];
    let time = &datetime[11..];

    println!("Todays Date: {}", date);
    println!("Current Weather: {}°F", temp);
    println!("Last checked at {}", time);
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    let endpoint = String::from("https://api.open-meteo.com/v1/forecast?latitude=38.8951&longitude=-77.0364&hourly=temperature_2m&current=temperature_2m,rain&timezone=America%2FNew_York&temperature_unit=fahrenheit");

    let req: WeatherResponse = reqwest::Client::new()
        .get(endpoint)
        .send()
        .await?
        .json()
        .await?;

    let htime = req.hourly.time;
    let htemp = req.hourly.temperature_2m;
    let ctime = req.current.time;
    let ctemp = req.current.temperature_2m;

    currentweather(&ctime, &ctemp);
    hourlyweather(&htime, &htemp);

    Ok(()) 
}
