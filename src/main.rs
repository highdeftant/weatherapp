use chrono::{Local, DateTime};
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct WeatherResponse {
    latitude: f64,
    longitude: f64,
    hourly: CurrentWeather,
}

#[derive(Serialize, Deserialize, Debug)]
struct CurrentWeather {
    time: Vec<String>,
    temperature_2m: Vec<f64>,
}


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let endpoint = String::from("https://api.open-meteo.com/v1/forecast?latitude=38.8951&longitude=-77.0364&hourly=temperature_2m&temperature_unit=fahrenheit");
    let req: WeatherResponse = reqwest::Client::new()
        .get(endpoint)
        .send()
        .await?
        .json()
        .await?;

    let time = req.hourly.time;
    let temp = req.hourly.temperature_2m;

    print_weather(&time, &temp);


    //println!("{0:?}", req.hourly.temperature_2m);
    Ok(()) 
}


fn print_weather(current: &Vec<String>, temp: &Vec<f64>) {

    let dt: chrono::DateTime<Local> = chrono::Local::now();
    let utc: chono::DateTime<
    println!("{}", current[2]);
    println!("{dt}")

}
