mod getweather;
use crate::{
    getweather::{showopm};
}
use reqwest;
use serde::{Deserialize, Serialize};
use tokio::time::{interval, Duration};
use weather::DataPoll;

#[derive(Serialize, Deserialize, Debug)]
pub struct OpmStatus {
   pub Location: String,
   pub ShortStatusMessage: String,
   pub ExtendedInformation: String,
   pub StatusType: String,

pub async fn get_opm(endpoint: &str, int: u64) -> Result<(), reqwest::Error> {
    let mut timer = interval(Duration::from_secs(int));
    let client = reqwest::Client::new();

    loop {
        timer.tick().await;

        let opm = client
            .get(endpoint)
            .send()
            .await?
            .json::<OpmStatus>()
            .await?;

        // set variables to send to logic functions
         let location = opm.Location;
         let status = opm.StatusType;
         let shortmsg = opm.ShortStatusMessage;
         let extendedinfo = opm.ExtendedInformation;
         
    }

    Ok(())
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
