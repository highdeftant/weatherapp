mod weather;
mod ui;

use reqwest;
use serde::{Deserialize, Serialize};
use tokio::time::{interval, Duration};
use weather::DataPoll;

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
    temperature_2m: f64,
    rain: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct HourlyWeather {
    time: Vec<String>,
    temperature_2m: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct WeatherWidget {
    current_weather: String,
    current_time: String,
    hourly_tempurature: Vec<f64>,
    hourly_time: Vec<String>,
}

pub async fn get_weather(endpoint: &str, int: u64) -> Result<(), reqwest::Error> {
    let mut timer = interval(Duration::from_secs(int));
    let client = reqwest::Client::new();

    loop {
        timer.tick().await;

        let weather = client
            .get(endpoint)
            .send()
            .await?
            .json::<WeatherResponse>()
            .await?;


        // set variables to send to logic functions
        let htime = weather.hourly.time;
        let htemp = weather.hourly.temperature_2m;
        let ctime = weather.current.time;
        let ctemp = weather.current.temperature_2m;
    }


    Ok(())
}
