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

#[cfg(test)]
mod tests {
    use super::get_hourly;
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
}
