use serde::{Deserialize, Serialize};

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

// OPM Status
#[derive(Serialize, Deserialize, Debug)]
pub struct OpmStatus {
   pub Location: String,
   pub ShortStatusMessage: String,
   pub ExtendedInformation: String,
   pub StatusType: String,
}

