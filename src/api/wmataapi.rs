use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Trains {
    #[serde(rename = "Trains")]
    trains: Vec<TrainInfo>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
struct TrainInfo {
    #[serde(rename = "DestinationName")]
    destination_name: String,
    #[serde(rename = "Line")]
    line: String,
    #[serde(rename = "LocationName")]
    location_name: String,
    #[serde(rename = "Min")]
    min: String,
}

fn eta_rank(min: &str) -> i32 {
    match min {
        "BRD" => 0,
        "ARR" => 1,
        _ => min.parse::<i32>().unwrap_or(i32::MAX),
    }
}

fn format_train_row(train: &TrainInfo) -> String {
    format!(
        "{:<14} {:<2} {:>3}",
        train.destination_name, train.line, train.min
    )
}

fn format_station_lines(mut trains: Vec<TrainInfo>, max: usize) -> Vec<String> {
    if trains.is_empty() {
        return vec!["Station: --".to_string(), "No arrivals found".to_string()];
    }

    trains.sort_by_key(|t| eta_rank(&t.min));
    let station_name = trains[0].location_name.clone();

    let mut lines = Vec::with_capacity(max.saturating_add(1));
    lines.push(format!("Station: {}", station_name));
    lines.extend(trains.into_iter().take(max).map(|t| format_train_row(&t)));
    lines
}

/// Fetch train prediction lines from WMATA API, returning up to `max` formatted lines.
/// Requires WMATA_API_KEY and optionally uses WMATA_STATION_CODE.
pub async fn status_lines_from_env(client: &reqwest::Client, max: usize) -> Vec<String> {
    let Ok(api_key) = std::env::var("WMATA_API_KEY") else {
        return vec![
            "Station: --".to_string(),
            "Set WMATA_API_KEY to enable live arrivals".to_string(),
        ];
    };

    let station_code = std::env::var("WMATA_STATION_CODE").unwrap_or_else(|_| "A01".to_string());
    let endpoint = format!(
        "https://api.wmata.com/StationPrediction.svc/json/GetPrediction/{}",
        station_code
    );

    match client
        .get(&endpoint)
        .header("api_key", &api_key)
        .send()
        .await
    {
        Ok(resp) => match resp.json::<Trains>().await {
            Ok(trains) => format_station_lines(trains.trains, max),
            Err(_) => vec![
                "Station: --".to_string(),
                "Error parsing WMATA response".to_string(),
            ],
        },
        Err(_) => vec![
            "Station: --".to_string(),
            "WMATA API request failed".to_string(),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::{TrainInfo, eta_rank, format_station_lines, format_train_row};

    fn train(location: &str, dest: &str, line: &str, min: &str) -> TrainInfo {
        TrainInfo {
            destination_name: dest.to_string(),
            line: line.to_string(),
            location_name: location.to_string(),
            min: min.to_string(),
        }
    }

    #[test]
    fn eta_rank_orders_boarding_arriving_then_numeric() {
        assert!(eta_rank("BRD") < eta_rank("ARR"));
        assert!(eta_rank("ARR") < eta_rank("3"));
        assert!(eta_rank("3") < eta_rank("10"));
    }

    #[test]
    fn format_train_row_aligns_destination_line_and_eta() {
        let row = format_train_row(&train("Metro Center", "Shady Grove", "RD", "3"));
        assert_eq!(row, "Shady Grove    RD   3");
    }

    #[test]
    fn format_station_lines_adds_station_header_and_sorts_eta() {
        let lines = format_station_lines(
            vec![
                train("Metro Center", "Glenmont", "RD", "10"),
                train("Metro Center", "Shady Grove", "RD", "BRD"),
                train("Metro Center", "Silver Spring", "RD", "ARR"),
                train("Metro Center", "Glenmont", "RD", "3"),
            ],
            3,
        );

        assert_eq!(lines[0], "Station: Metro Center");
        assert_eq!(lines[1], "Shady Grove    RD BRD");
        assert_eq!(lines[2], "Silver Spring  RD ARR");
        assert_eq!(lines[3], "Glenmont       RD   3");
    }

    #[test]
    fn format_station_lines_handles_empty_results() {
        let lines = format_station_lines(vec![], 5);
        assert_eq!(lines, vec!["Station: --", "No arrivals found"]);
    }
}
