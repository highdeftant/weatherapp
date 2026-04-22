//! WMATA API integration for fetching train arrivals.
//!
//! Supports multi-station configuration via `WMATA_STATION_CODE` environment variable
//! as a comma-separated list of station codes (e.g., "A01,C02,D04").
//! Single-station mode is preserved as the default (falls back to "A01").
//!
//! Configuration:
//! - `WMATA_API_KEY`: Required API key for WMATA predictions endpoint
//! - `WMATA_STATION_CODE`: Comma-separated list of station codes (optional, defaults to "A01")
//! - `WMATA_MAX_ROWS`: Max arrival lines per station (optional, defaults to 6, max 20)

use serde::Deserialize;

const DEFAULT_STATION_CODE: &str = "A01";
const DEFAULT_MAX_ROWS: usize = 6;
const MAX_ALLOWED_ROWS: usize = 20;
const DESTINATION_COL_WIDTH: usize = 10;

/// WMATA configuration loaded from environment variables.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WmataConfig {
    /// List of station codes to fetch arrivals for.
    station_codes: Vec<String>,
    /// Maximum arrival lines to display per station.
    max_rows: usize,
}

impl WmataConfig {
    /// Returns the first (primary) station code, for backwards compatibility.
    #[allow(dead_code)]
    pub fn primary_station_code(&self) -> &str {
        self.station_codes
            .first()
            .map(String::as_str)
            .unwrap_or(DEFAULT_STATION_CODE)
    }

    /// Returns all station codes.
    pub fn station_codes(&self) -> &[String] {
        &self.station_codes
    }

    /// Returns the configured max rows per station.
    pub fn max_rows(&self) -> usize {
        self.max_rows
    }
}

/// Parse station codes from a comma-separated string.
/// Validates each code and normalizes to uppercase.
fn parse_station_codes(raw: Option<&str>) -> Vec<String> {
    let mut codes: Vec<String> = raw
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_ascii_uppercase())
        .filter(|s| is_valid_station_code(s))
        .collect();

    if codes.is_empty() {
        codes.push(DEFAULT_STATION_CODE.to_string());
    }

    codes
}

/// Validate a WMATA station code (format: one letter followed by two digits).
fn is_valid_station_code(code: &str) -> bool {
    if code.len() != 3 {
        return false;
    }

    let bytes = code.as_bytes();
    bytes[0].is_ascii_uppercase() && bytes[1].is_ascii_digit() && bytes[2].is_ascii_digit()
}

/// Parse and clamp max rows from environment variable.
fn parse_max_rows(raw: Option<&str>, fallback: usize) -> usize {
    let fallback = fallback.clamp(1, MAX_ALLOWED_ROWS);

    let Some(raw) = raw else {
        return fallback;
    };

    match raw.trim().parse::<usize>() {
        Ok(value) => value.clamp(1, MAX_ALLOWED_ROWS),
        Err(_) => fallback,
    }
}

/// Load WMATA configuration from environment variables.
pub fn load_config_from_env(default_max_rows: usize) -> WmataConfig {
    let station_codes = parse_station_codes(std::env::var("WMATA_STATION_CODE").ok().as_deref());
    let max_rows = parse_max_rows(
        std::env::var("WMATA_MAX_ROWS").ok().as_deref(),
        if default_max_rows == 0 {
            DEFAULT_MAX_ROWS
        } else {
            default_max_rows
        },
    );

    WmataConfig {
        station_codes,
        max_rows,
    }
}

#[derive(Deserialize, Debug)]
struct Trains {
    #[serde(rename = "Trains")]
    trains: Vec<TrainInfo>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub(crate) struct TrainInfo {
    #[serde(rename = "DestinationName")]
    destination_name: String,
    #[serde(rename = "Line")]
    line: String,
    #[serde(rename = "LocationName")]
    location_name: String,
    #[serde(rename = "Min")]
    min: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum EtaState {
    Boarding,
    Arriving,
    Minutes(i32),
    Delayed,
    Unknown,
}

fn normalize_eta(min: &str) -> EtaState {
    let trimmed = min.trim();
    if trimmed.is_empty() {
        return EtaState::Unknown;
    }

    let normalized = trimmed.to_ascii_uppercase();
    match normalized.as_str() {
        "BRD" | "BOARDING" => EtaState::Boarding,
        "ARR" | "ARV" | "ARRIVING" => EtaState::Arriving,
        "DLY" | "DEL" | "DELAYED" => EtaState::Delayed,
        "-" | "--" | "---" | "NULL" | "N/A" | "NA" | "TBD" | "?" => EtaState::Unknown,
        _ => match trimmed.parse::<i32>() {
            Ok(minutes) if minutes >= 0 => EtaState::Minutes(minutes),
            _ => EtaState::Unknown,
        },
    }
}

fn eta_sort_key(min: &str) -> (u8, i32) {
    match normalize_eta(min) {
        EtaState::Boarding => (0, 0),
        EtaState::Arriving => (1, 0),
        EtaState::Minutes(minutes) => (2, minutes),
        EtaState::Delayed => (3, 0),
        EtaState::Unknown => (4, 0),
    }
}

fn eta_display(min: &str) -> String {
    match normalize_eta(min) {
        EtaState::Boarding => "BRD".to_string(),
        EtaState::Arriving => "ARR".to_string(),
        EtaState::Minutes(minutes) => minutes.to_string(),
        EtaState::Delayed => "DLY".to_string(),
        EtaState::Unknown => "--".to_string(),
    }
}

fn normalize_destination(destination: &str, width: usize) -> String {
    if width == 0 {
        return String::new();
    }

    let destination_length = destination.chars().count();
    if destination_length <= width {
        return format!("{destination:<width$}");
    }

    if width == 1 {
        return "…".to_string();
    }

    let mut truncated: String = destination.chars().take(width - 1).collect();
    truncated.push('…');
    truncated
}

fn format_train_row(train: &TrainInfo) -> String {
    let eta = eta_display(&train.min);
    let destination = normalize_destination(&train.destination_name, DESTINATION_COL_WIDTH);
    format!(
        "{:<destination_width$} {:<2} {:>3}",
        destination,
        train.line,
        eta,
        destination_width = DESTINATION_COL_WIDTH
    )
}

/// Format a single station's arrivals into display lines.
pub fn format_station_lines(
    trains: Vec<TrainInfo>,
    max: usize,
    station_name: String,
) -> Vec<String> {
    let display_name = if station_name.is_empty() {
        "Unknown Station".to_string()
    } else {
        station_name
    };

    if trains.is_empty() {
        return vec![
            format!("Station: {}", display_name),
            "No arrivals found".to_string(),
        ];
    }

    let mut sorted_trains = trains;
    sorted_trains.sort_by(|a, b| {
        eta_sort_key(&a.min)
            .cmp(&eta_sort_key(&b.min))
            .then_with(|| a.destination_name.cmp(&b.destination_name))
            .then_with(|| a.line.cmp(&b.line))
            .then_with(|| a.min.trim().cmp(b.min.trim()))
    });

    let mut lines = Vec::with_capacity(max.saturating_add(1));
    lines.push(format!("Station: {}", display_name));
    lines.extend(
        sorted_trains
            .into_iter()
            .take(max)
            .map(|t| format_train_row(&t)),
    );
    lines
}

/// Fetch arrival predictions for all configured stations.
/// Returns formatted lines grouped by station, with each station section prefixed by its name.
pub async fn fetch_all_stations(client: &reqwest::Client, config: &WmataConfig) -> Vec<String> {
    let Ok(api_key) = std::env::var("WMATA_API_KEY") else {
        return vec!["Set WMATA_API_KEY to enable live arrivals".to_string()];
    };

    let station_codes = config.station_codes();
    let mut all_lines = Vec::new();

    for (index, station_code) in station_codes.iter().enumerate() {
        let endpoint = format!(
            "https://api.wmata.com/StationPrediction.svc/json/GetPrediction/{}",
            station_code
        );

        let station_lines = match client
            .get(&endpoint)
            .header("api_key", &api_key)
            .send()
            .await
        {
            Ok(resp) => match resp.json::<Trains>().await {
                Ok(trains) => {
                    let station_name = trains
                        .trains
                        .first()
                        .map(|t| t.location_name.clone())
                        .unwrap_or_else(|| format!("Station {}", station_code));
                    format_station_lines(trains.trains, config.max_rows(), station_name)
                }
                Err(_) => {
                    vec![
                        format!("Station: {}", station_code),
                        "Error parsing response".to_string(),
                    ]
                }
            },
            Err(_) => {
                vec![
                    format!("Station: {}", station_code),
                    "API request failed".to_string(),
                ]
            }
        };

        all_lines.extend(station_lines);

        if index + 1 < station_codes.len() {
            all_lines.push(String::new());
        }
    }

    all_lines
}

/// Backwards-compatible wrapper that fetches only the primary station.
#[allow(dead_code)]
#[deprecated(
    since = "0.2.0",
    note = "Use fetch_all_stations() for multi-station support"
)]
pub async fn status_lines_from_env(client: &reqwest::Client, max: usize) -> Vec<String> {
    let config = load_config_from_env(max);
    fetch_all_stations(client, &config).await
}

#[cfg(test)]
mod tests {
    use super::{
        eta_sort_key, format_station_lines, format_train_row, normalize_destination,
        parse_max_rows, parse_station_codes, TrainInfo, DEFAULT_STATION_CODE,
        DESTINATION_COL_WIDTH, MAX_ALLOWED_ROWS,
    };

    fn train(location: &str, dest: &str, line: &str, min: &str) -> TrainInfo {
        TrainInfo {
            destination_name: dest.to_string(),
            line: line.to_string(),
            location_name: location.to_string(),
            min: min.to_string(),
        }
    }

    #[test]
    fn eta_sort_orders_boarding_arriving_then_numeric() {
        assert!(eta_sort_key("BRD") < eta_sort_key("ARR"));
        assert!(eta_sort_key("ARR") < eta_sort_key("3"));
        assert!(eta_sort_key("3") < eta_sort_key("10"));
    }

    #[test]
    fn normalize_destination_truncates_with_ellipsis() {
        let destination = normalize_destination("Largo Town Center", DESTINATION_COL_WIDTH);
        assert_eq!(destination, "Largo Tow…");
        assert_eq!(destination.chars().count(), DESTINATION_COL_WIDTH);
    }

    #[test]
    fn normalize_destination_pads_short_destination() {
        let destination = normalize_destination("NoMa", DESTINATION_COL_WIDTH);
        assert_eq!(destination, "NoMa      ");
        assert_eq!(destination.chars().count(), DESTINATION_COL_WIDTH);
    }

    #[test]
    fn format_train_row_aligns_destination_line_and_eta() {
        let row = format_train_row(&train("Metro Center", "Shady Grove", "RD", "3"));
        assert_eq!(row, "Shady Gro… RD   3");
    }

    #[test]
    fn format_train_row_keeps_line_and_eta_visible_in_fixed_width() {
        let row = format_train_row(&train("Metro Center", "Largo Town Center", "BL", "10"));
        assert_eq!(row, "Largo Tow… BL  10");
        assert_eq!(
            row.chars().count(),
            DESTINATION_COL_WIDTH + 1 + 2 + 1 + 3,
            "destination + spacer + line + spacer + eta"
        );
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
            "Metro Center".to_string(),
        );

        assert_eq!(lines[0], "Station: Metro Center");
        assert_eq!(lines[1], "Shady Gro… RD BRD");
        assert_eq!(lines[2], "Silver Sp… RD ARR");
        assert_eq!(lines[3], "Glenmont   RD   3");
    }

    #[test]
    fn format_station_lines_handles_empty_results() {
        let lines = format_station_lines(vec![], 5, "Test Station".to_string());
        assert_eq!(lines, vec!["Station: Test Station", "No arrivals found"]);
    }

    #[test]
    fn format_station_lines_uses_default_name_when_empty() {
        let lines = format_station_lines(vec![], 5, "".to_string());
        assert_eq!(lines, vec!["Station: Unknown Station", "No arrivals found"]);
    }

    #[test]
    fn parse_station_codes_uses_default_for_missing_or_invalid_values() {
        assert_eq!(
            parse_station_codes(None),
            vec![DEFAULT_STATION_CODE.to_string()]
        );
        assert_eq!(
            parse_station_codes(Some("foo,123")),
            vec![DEFAULT_STATION_CODE.to_string()]
        );
    }

    #[test]
    fn parse_station_codes_accepts_valid_codes_and_normalizes_case() {
        assert_eq!(
            parse_station_codes(Some("a01, c02,invalid")),
            vec!["A01".to_string(), "C02".to_string()]
        );
    }

    #[test]
    fn parse_station_codes_filters_empty_and_invalid_codes() {
        let codes = parse_station_codes(Some("A01,,C02,  ,invalid"));
        assert_eq!(codes, vec!["A01".to_string(), "C02".to_string()]);
    }

    #[test]
    fn parse_max_rows_uses_fallback_for_missing_or_invalid_values() {
        assert_eq!(parse_max_rows(None, 6), 6);
        assert_eq!(parse_max_rows(Some("nope"), 6), 6);
    }

    #[test]
    fn parse_max_rows_clamps_to_allowed_bounds() {
        assert_eq!(parse_max_rows(Some("0"), 6), 1);
        assert_eq!(parse_max_rows(Some("999"), 6), MAX_ALLOWED_ROWS);
    }

    #[test]
    fn parse_station_codes_handles_single_station() {
        assert_eq!(parse_station_codes(Some("A01")), vec!["A01".to_string()]);
    }

    #[test]
    fn parse_station_codes_handles_multiple_stations() {
        let codes = parse_station_codes(Some("A01,C02,D04"));
        assert_eq!(
            codes,
            vec!["A01".to_string(), "C02".to_string(), "D04".to_string()]
        );
    }
}
