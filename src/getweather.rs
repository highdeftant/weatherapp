use chrono::{DateTime, Local, NaiveDateTime, TimeZone};

pub fn get_current(_datetime: &str, _ctemp: &f64, _local: &DateTime<Local>) {
    let _datestring = "%Y-%m-%dT%H:%M";
    let _naive_dt = NaiveDateTime::parse_from_str(&_datetime, _datestring)
        .expect("[ERR]: Could Not Parse DateTime -> :");

    let _local_dt = Local
        .from_local_datetime(&_naive_dt)
        .single()
        .expect("[ERR]: Invalid time input. -> :");

    let _now_time = Local::now();
}

pub fn get_hourly(hourly: &Vec<String>, temp: &Vec<f64>) -> String {
    let datestring = "%Y-%m-%dT%H:%M";
    let local = chrono::Local::now();
    let mut num = 0;
    let mut next = 0;
    let mut new_hours = String::new();
    
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
                new_hours.push_str(&format!("{}°", temp[num]).to_string());
            }
        }
    }
    new_hours
}

pub fn showopm(location: &str, shortmessage: &str, extendedinfo: &str, status: &str) -> Vec<String> {
    let mut opm: Vec<String> = vec![
        format!("Location: {location}").to_string(),
        format!("Status: {status}").to_string(),
        format!("Information: {shortmessage}").to_string(),
        format!("Extended: {extendedinfo}").to_string(),
    ];
    opm
}
