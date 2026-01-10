use chrono::{DateTime, Local, NaiveDateTime, TimeZone};

pub fn get_current(_datetime: &str, _ctemp: &f64, _local: &DateTime<Local>) -> Vec<String> {
    let _datestring = "%Y-%m-%dT%H:%M";
    let _naive_dt = NaiveDateTime::parse_from_str(&_datetime, _datestring)
        .expect("[ERR]: Could Not Parse DateTime -> :");

    let _local_dt = Local
        .from_local_datetime(&_naive_dt)
        .single()
        .expect("[ERR]: Invalid time input. -> :");

    let now_time = Local::now();

    let current_time = vec![format!("{}", now_time)];
    current_time
}

pub fn get_hourly(hourly: &Vec<String>, temp: &Vec<f64>) -> (Vec<String>, Vec<f64>, i32) {
    let datestring = "%Y-%m-%dT%H:%M";
    let local = chrono::Local::now();
    let mut num = 0;
    let mut next = 0;
    let mut new_hours: Vec<String> = Vec::new();
    let mut new_temp: Vec<f64> = Vec::new();
    
    for hour in hourly {
        num += 1;
        // String -> NaiveDateTime
        let naivedate = NaiveDateTime::parse_from_str(hour, datestring)
            .expect("[ERR] Error parsing Vec<T> -> :");

        // NaiveDateTime -> DateTime<Local>
        let naivelocal = Local
            .from_local_datetime(&naivedate)
            .single()
            .expect("[ERR]: Invalid time input. -> :");

        // Compares DateTime to Local
        if naivelocal.date_naive() == local.date_naive() {
            if naivelocal.time() >= local.time() {
                next += 1;
                new_hours.push(format!("{}° at {}", temp[num], naivelocal.time()).to_string());
            }
        }
    }
    (new_hours, new_temp, next)
}

pub fn showopm(location: &str, shortmessage: &str, extendedinfo: &str, status: &str) -> Vec<String> {
    let opm: Vec<String> = vec![
        format!("Status: {status}").to_string(),
        format!("Location: {location}").to_string(),
        format!("Information: {shortmessage}").to_string(),
        format!("Extended: {extendedinfo}").to_string(),
    ];
    opm
}
