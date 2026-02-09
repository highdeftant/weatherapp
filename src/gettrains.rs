use reqwest; 
#[derive(Serialize, Deserialize, Debug)]
pub struct Trains {
    Trains: TrainInfo
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

pub async fn get_trains() -> Result<(), reqwest::error::Error> {
    let mut endpoint = "http://api.wmata.com/StationPrediction.svc/json/GetPrediction/{}";

    traininfo: Trains = reqwest::Client::new()
        .get(endpoint)
        .await?
        .json::<TrainInfo>()
        .await?;


}
