use chrono::{DateTime, Local, NaiveDateTime, TimeZone};

pub fn get_currentweather(_datetime: &str, _ctemp: &f64, _local: &DateTime<Local>) {
    todo!();
}

pub fn get_hourlyweather(hourly: &Vec<String>, temp: &Vec<f64>) {
    let datestring = "%Y-%m-%dT%H:%M";
    let local = chrono::Local::now();
    let temp_iter = temp.iter();
    let mut num = 0;
    let mut next = 0;
    // rewrite this to use the index, which will give temp
    println!("");
    println!("Hourly Forecast");
    println!("---------------");
    println!("");

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
                println!("{}° at {}", temp[num], naivelocal.time());
            }
        }
    }
    println!("");
    println!("---------------");
}

pub fn showopm(location: &str, shortmessage: &str, extendedinfo: &str, status: &str) {
    println!("");
    println!("---- OPM Status ----");
    println!("Location: {}", location);
    println!("Status: {}", status);
    println!("Information: {}", shortmessage);
    println!("Extended:{}", extendedinfo);
    println!("");
}
