use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use std::collections::HashMap;

pub fn get_current(_datetime: &str, _ctemp: &f64, _local: &DateTime<Local>) {
    todo!();
}

pub fn get_hourly(hourly: &Vec<String>, temp: &Vec<f64>) -> HashMap<String, String> {
    let datestring = "%Y-%m-%dT%H:%M";
    let local = chrono::Local::now();
    let temp_iter = temp.iter();
    let mut num = 0;
    let mut next = 0;
    let mut next_hours = HashMap::new();
    // rewrite this to use the index, which will give temp
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
                next_hours.insert(temp[num].to_string(), naivelocal.time().to_string());
            }
        }
    }
    next_hours
}

pub fn showopm(location: &str, shortmessage: &str, extendedinfo: &str, status: &str) -> String {
    let opm = format!(
        "
        ---- OPM Status ----
        Location: {location}
        Status: {status}
        Information: {shortmessage}
        Extended: {extendedinfo}"
    );
    opm
}
