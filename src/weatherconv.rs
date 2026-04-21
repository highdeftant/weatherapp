use chrono::{Local, NaiveDateTime, TimeZone};

fn naive_to_local(time: &str) {
    let datestring = "%Y-%m-%dT%H:%M";
    let naive_dt = NaiveDateTime::parse_from_str(time, datestring)
        .expect("[ERR]: Could Not Parse DateTime -> :");

    let local_dt = Local
        .from_local_datetime(&naive_dt)
        .single()
        .expect("[ERR]: Invalid time input. -> :");
    let _ = local_dt;
}

pub fn get_current(datetime: &str, ctemp: &f64) -> Vec<String> {
    let datestring = "%Y-%m-%dT%H:%M";
    let naive_dt = NaiveDateTime::parse_from_str(datetime, datestring)
        .expect("[ERR]: Could Not Parse DateTime -> :");

    let local_dt = Local
        .from_local_datetime(&naive_dt)
        .single()
        .expect("[ERR]: Invalid time input. -> :");

    vec![format!("{}°", ctemp), format!("{}", local_dt.time())]
}

/// Convert a 0-23 hour integer into a 12-hour label like "12AM", "3PM".
fn hour_to_label(hour: u32) -> String {
    let h12 = if hour % 12 == 0 { 12 } else { hour % 12 };
    let ampm = if hour < 12 { "AM" } else { "PM" };
    format!("{}{}", h12, ampm)
}

/// Extract the hour (0-23) from a "%Y-%m-%dT%H:%M" timestamp string
/// using simple string slicing. Avoids chrono time-component methods.
fn hour_from_timestamp(ts: &str) -> Option<u32> {
    // Expected: "2024-01-01T14:00"
    //                      ^^^
    let t_pos = ts.find('T')?;
    let hour_str = ts.get(t_pos + 1..t_pos + 3)?;
    hour_str.parse::<u32>().ok()
}

/// Parse a timestamp string in "%Y-%m-%dT%H:%M" format into a human-readable
/// hour label like "12PM" or "3AM".
pub fn timestamp_to_hour_label(ts: &str) -> String {
    hour_from_timestamp(ts)
        .map(hour_to_label)
        .unwrap_or_else(|| "?".to_string())
}

/// Data ready for the chart: hour labels, temps, count.
pub struct ChartData {
    pub hour_labels: Vec<String>,
    pub temps: Vec<f64>,
    pub count: i32,
}

pub fn get_hourly(hourly: &Vec<String>, temp: &Vec<f64>) -> (Vec<String>, Vec<f64>, i32) {
    let datestring = "%Y-%m-%dT%H:%M";
    let local = chrono::Local::now();
    let mut next = 0;
    let mut new_hours: Vec<String> = Vec::new();
    let mut new_temp: Vec<f64> = Vec::new();

    for (index, hour) in hourly.iter().enumerate() {
        let Some(temp_value) = temp.get(index) else {
            continue;
        };

        let naivedate = NaiveDateTime::parse_from_str(hour, datestring)
            .expect("[ERR] Error parsing Vec<T> -> :");

        let naivelocal = Local
            .from_local_datetime(&naivedate)
            .single()
            .expect("[ERR]: Invalid time input. -> :");

        if naivelocal.date_naive() == local.date_naive() && naivelocal.time() >= local.time() {
            next += 1;
            new_hours.push(format!("{}° at {}", temp_value, naivelocal.time()));
            new_temp.push(*temp_value);
        }
    }

    (new_hours, new_temp, next)
}

/// Produce chart-ready data from the same inputs used by get_hourly.
/// Returns raw timestamps (not formatted) so the chart can build hour labels.
pub fn get_chart_data(hourly: &Vec<String>, temp: &Vec<f64>) -> ChartData {
    let datestring = "%Y-%m-%dT%H:%M";
    let local = chrono::Local::now();
    let mut hour_labels: Vec<String> = Vec::new();
    let mut temps: Vec<f64> = Vec::new();

    for (index, hour) in hourly.iter().enumerate() {
        let Some(temp_value) = temp.get(index) else {
            continue;
        };

        let Ok(naivedate) = NaiveDateTime::parse_from_str(hour, datestring) else {
            continue;
        };

        let Some(naivelocal) = Local.from_local_datetime(&naivedate).single() else {
            continue;
        };

        if naivelocal.date_naive() == local.date_naive() && naivelocal.time() >= local.time() {
            if let Some(h) = hour_from_timestamp(hour) {
                hour_labels.push(hour_to_label(h));
            } else {
                hour_labels.push("?".to_string());
            }
            temps.push(*temp_value);
        }
    }

    let count = hour_labels.len() as i32;
    ChartData {
        hour_labels,
        temps,
        count,
    }
}

#[cfg(test)]
mod tests {
    use super::{get_chart_data, get_hourly, hour_from_timestamp, timestamp_to_hour_label};
    use chrono::{Duration, Local};

    #[test]
    fn get_hourly_returns_matching_temp_points_for_upcoming_hours() {
        let now = Local::now();
        let past = (now - Duration::hours(1))
            .format("%Y-%m-%dT%H:%M")
            .to_string();
        let next = (now + Duration::hours(1))
            .format("%Y-%m-%dT%H:%M")
            .to_string();

        let hourly = vec![past, next];
        let temp = vec![61.0, 72.5];

        let (_labels, temps, count) = get_hourly(&hourly, &temp);

        assert_eq!(count, 1);
        assert_eq!(temps, vec![72.5]);
    }

    #[test]
    fn hour_from_timestamp_parses_valid() {
        assert_eq!(hour_from_timestamp("2024-01-01T00:00"), Some(0));
        assert_eq!(hour_from_timestamp("2024-01-01T06:00"), Some(6));
        assert_eq!(hour_from_timestamp("2024-01-01T12:00"), Some(12));
        assert_eq!(hour_from_timestamp("2024-01-01T15:00"), Some(15));
        assert_eq!(hour_from_timestamp("2024-01-01T23:00"), Some(23));
    }

    #[test]
    fn hour_from_timestamp_returns_none_on_invalid() {
        assert_eq!(hour_from_timestamp("not-a-date"), None);
        assert_eq!(hour_from_timestamp(""), None);
        assert_eq!(hour_from_timestamp("2024-01-01"), None);
    }

    #[test]
    fn timestamp_to_hour_label_24h_formats() {
        assert_eq!(timestamp_to_hour_label("2024-01-01T00:00"), "12AM");
        assert_eq!(timestamp_to_hour_label("2024-01-01T06:00"), "6AM");
        assert_eq!(timestamp_to_hour_label("2024-01-01T12:00"), "12PM");
        assert_eq!(timestamp_to_hour_label("2024-01-01T15:00"), "3PM");
        assert_eq!(timestamp_to_hour_label("2024-01-01T23:00"), "11PM");
    }

    #[test]
    fn timestamp_to_hour_label_invalid_returns_question() {
        assert_eq!(timestamp_to_hour_label("not-a-date"), "?");
        assert_eq!(timestamp_to_hour_label(""), "?");
    }

    #[test]
    fn get_chart_data_produces_hour_labels_and_temps() {
        let now = Local::now();
        let next1 = (now + Duration::hours(1))
            .format("%Y-%m-%dT%H:%M")
            .to_string();
        let next2 = (now + Duration::hours(2))
            .format("%Y-%m-%dT%H:%M")
            .to_string();

        let hourly = vec![next1, next2];
        let temp = vec![70.0, 75.0];

        let chart = get_chart_data(&hourly, &temp);

        assert_eq!(chart.count, 2);
        assert_eq!(chart.temps, vec![70.0, 75.0]);
        assert_eq!(chart.hour_labels.len(), 2);
    }
}
