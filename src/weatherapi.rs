use reqwest;
use serde::{Deserialize, Serialize};
use tokio::time::{interval, Duration};

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub current: CurrentWeather,
    pub hourly: HourlyWeather,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentWeather {
    pub time: String,
    pub interval: i64,
    pub temperature_2m: f64,
    pub rain: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HourlyWeather {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f64>,
}

pub async fn get_weather(endpoint: &str, interval: u64) -> Result<(), reqwest::Error> {
    let mut timer = interval(Duration::from_secs(3));
    let client = reqwest::Client::new();

    loop {
        timer.tick().await;

        let weather = client
            .get(endpoint)
            .send()
            .await?
            .json::<WeatherResponse>()
            .await?;

        let ctime = weather.current.temperature_2m;
        println!("{ctime}")

    }
    Ok(())
}
