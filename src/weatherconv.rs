use chrono::{DateTime, Local, NaiveDateTime, TimeZone};


fn naive_to_local(time: String) {


}
pub fn get_current(datetime: &str, ctemp: &f64) -> Vec<String> {

    let datestring = "%Y-%m-%dT%H:%M";
    let naive_dt = NaiveDateTime::parse_from_str(&datetime, datestring)
        .expect("[ERR]: Could Not Parse DateTime -> :");

    let local_dt = Local
        .from_local_datetime(&naive_dt)
        .single()
        .expect("[ERR]: Invalid time input. -> :");


    let current_time = vec![
        format!("{}°", ctemp),
        format!("{}", local_dt.time()),
        
    ];
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
