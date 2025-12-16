
pub fn showopm(location: &str, shortmessage: &str, extendedinfo: &str, status: &str) {
    println!("");
    println!("---- OPM Status ----");
    println!("Location: {}", location);
    println!("Status: {}", status);
    println!("Information: {}", shortmessage);
    println!("Extended:{}", extendedinfo);
    println!("");
}
