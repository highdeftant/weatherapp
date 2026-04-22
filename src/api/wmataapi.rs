use serde::Deserialize;

const DEFAULT_STATION_CODE: &str = "A01";
const DEFAULT_MAX_ROWS: usize = 6;
const MAX_ALLOWED_ROWS: usize = 20;
const DESTINATION_COL_WIDTH: usize = 10;

#[derive(Debug, Clone, PartialEq, Eq)]
struct WmataConfig {
    station_codes: Vec<String>,
    max_rows: usize,
}

impl WmataConfig {
    fn primary_station_code(&self) -> &str {
        self.station_codes
            .first()
            .map(String::as_str)
            .unwrap_or(DEFAULT_STATION_CODE)
    }
}

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

fn is_valid_station_code(code: &str) -> bool {
    if code.len() != 3 {
        return false;
    }

    let bytes = code.as_bytes();
    bytes[0].is_ascii_uppercase() && bytes[1].is_ascii_digit() && bytes[2].is_ascii_digit()
}

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

fn load_config_from_env(default_max_rows: usize) -> WmataConfig {
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

fn format_station_lines(mut trains: Vec<TrainInfo>, max: usize) -> Vec<String> {
    if trains.is_empty() {
        return vec!["Station: --".to_string(), "No arrivals found".to_string()];
    }

    trains.sort_by(|a, b| {
        eta_sort_key(&a.min)
            .cmp(&eta_sort_key(&b.min))
            .then_with(|| a.destination_name.cmp(&b.destination_name))
            .then_with(|| a.line.cmp(&b.line))
            .then_with(|| a.min.trim().cmp(b.min.trim()))
    });
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

    let config = load_config_from_env(max);
    let endpoint = format!(
        "https://api.wmata.com/StationPrediction.svc/json/GetPrediction/{}",
        config.primary_station_code()
    );

    match client
        .get(&endpoint)
        .header("api_key", &api_key)
        .send()
        .await
    {
        Ok(resp) => match resp.json::<Trains>().await {
            Ok(trains) => format_station_lines(trains.trains, config.max_rows),
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
    use super::{
        DEFAULT_STATION_CODE, DESTINATION_COL_WIDTH, MAX_ALLOWED_ROWS, TrainInfo, eta_sort_key,
        format_station_lines, format_train_row, normalize_destination, parse_max_rows,
        parse_station_codes,
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
        );

        assert_eq!(lines[0], "Station: Metro Center");
        assert_eq!(lines[1], "Shady Gro… RD BRD");
        assert_eq!(lines[2], "Silver Sp… RD ARR");
        assert_eq!(lines[3], "Glenmont   RD   3");
    }

    #[test]
    fn format_station_lines_handles_empty_results() {
        let lines = format_station_lines(vec![], 5);
        assert_eq!(lines, vec!["Station: --", "No arrivals found"]);
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
    fn parse_max_rows_uses_fallback_for_missing_or_invalid_values() {
        assert_eq!(parse_max_rows(None, 6), 6);
        assert_eq!(parse_max_rows(Some("nope"), 6), 6);
    }

    #[test]
    fn parse_max_rows_clamps_to_allowed_bounds() {
        assert_eq!(parse_max_rows(Some("0"), 6), 1);
        assert_eq!(parse_max_rows(Some("999"), 6), MAX_ALLOWED_ROWS);
    }
}
