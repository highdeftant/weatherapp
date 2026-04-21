use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Trains {
    #[serde(rename = "Trains")]
    trains: Vec<TrainInfo>,
}

#[derive(Deserialize, Debug)]
struct TrainInfo {
    #[serde(rename = "DestinationName")]
    destination_name: String,
    #[serde(rename = "Line")]
    line: String,
    #[serde(rename = "Min")]
    min: String,
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
                .trains
                .into_iter()
                .take(max)
                .map(|t| format!("{} to {} — {} min", t.line, t.destination_name, t.min))
                .collect(),
            Err(_) => vec!["Error parsing WMATA response".to_string()],
        },
        Err(_) => vec!["WMATA API request failed".to_string()],
    }
}
