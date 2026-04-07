use reqwest;
use serde::{Deserialize, Serialize};
use tokio::time::{interval, Duration};
use crate::weather::DataPoll;

#[derive(Serialize, Deserialize, Debug)]
pub struct Trains {
    Trains: TrainInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrainInfo {
    Car: String,
    Destination: String,
    DestinationCode: String,
    DestinationName: String,
    LocationCode: String,
    LocationName: String,
    Line: String,
    Min: String,
}

pub async fn get_trains(int: u64) -> Result<(), reqwest::Error> {
    let mut endpoint = "http://api.wmata.com/StationPrediction.svc/json/GetPrediction/{}";
    let mut timer = interval(Duration::from_secs(int));

    loop {

        timer.tick().await;

        let traininfo = reqwest::Client::new()
            .get(endpoint)
            .await?
            .json::<Trains>()
            .await?;
        }
    
    Ok(())
}
