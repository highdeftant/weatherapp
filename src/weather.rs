use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherResponse {
    pub current: CurrentWeather,
    pub hourly: HourlyWeather,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentWeather {
    pub time: String,
    pub temperature_2m: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HourlyWeather {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f64>,
}
