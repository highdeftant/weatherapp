use chrono::{Local, NaiveDateTime, TimeZone};

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

fn hour_to_label(hour: u32) -> String {
    let h12 = if hour.is_multiple_of(12) {
        12
    } else {
        hour % 12
    };
    let ampm = if hour < 12 { "AM" } else { "PM" };
    format!("{}{}", h12, ampm)
}

/// Extract hour-of-day label from a formatted hourly string like "72.5° at 14:00:00".
pub fn label_from_hourly_string(s: &str) -> String {
    let Some(pos) = s.rfind(" at ") else {
        return "?".to_string();
    };
    let time_str = &s[pos + 4..];
    let colon_pos = time_str.find(':').unwrap_or(time_str.len());
    let hour = time_str[..colon_pos].parse::<u32>().unwrap_or(0);
    hour_to_label(hour)
}

pub fn get_hourly(hourly: &[String], temp: &[f64]) -> (Vec<String>, Vec<f64>, i32) {
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

#[cfg(test)]
mod tests {
    use super::{get_hourly, label_from_hourly_string};
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
    fn label_from_hourly_string_parses_formatted() {
        assert_eq!(label_from_hourly_string("72.5° at 14:00:00"), "2PM");
        assert_eq!(label_from_hourly_string("68.0° at 0:00:00"), "12AM");
        assert_eq!(label_from_hourly_string("80.0° at 12:30:00"), "12PM");
        assert_eq!(label_from_hourly_string("75.0° at 9:45:00"), "9AM");
    }

    #[test]
    fn label_from_hourly_string_invalid_returns_question() {
        assert_eq!(label_from_hourly_string("no time here"), "?");
        assert_eq!(label_from_hourly_string(""), "?");
    }
}
