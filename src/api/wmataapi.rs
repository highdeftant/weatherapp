use reqwest;
use serde::{Deserialize, Serialize};
use tokio::time::{Duration, interval};

#[derive(Serialize, Deserialize, Debug)]
pub struct Trains {
    Trains: Vec<TrainInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrainInfo {
    Car: String,
    Destination: String,
    DestinationCode: String,
    DestinationName: String,
    LocationCode: String,
    LocationName: String,
    Line: String,
    Min: String,
}

/// Fetch train prediction lines from WMATA API, returning up to `max` formatted lines.
/// Returns a fallback message if WMATA_API_KEY is not set.
pub async fn status_lines_from_env(client: &reqwest::Client, max: usize) -> Vec<String> {
    let Ok(api_key) = std::env::var("WMATA_API_KEY") else {
        return vec![
            "Station: --".to_string(),
            "Set WMATA_API_KEY to enable live arrivals".to_string(),
        ];
    };

    let endpoint = "https://api.wmata.com/StationPrediction.svc/json/GetPrediction/All";

    match client
        .get(endpoint)
        .header("api_key", &api_key)
        .send()
        .await
    {
        Ok(resp) => match resp.json::<Trains>().await {
            Ok(trains) => trains
                .Trains
                .into_iter()
                .take(max)
                .map(|t| format!("{} to {} — {} min", t.Line, t.DestinationName, t.Min))
                .collect(),
            Err(_) => vec!["Error parsing WMATA response".to_string()],
        },
        Err(_) => vec!["WMATA API request failed".to_string()],
    }
}
