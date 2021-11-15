use serde::Deserialize;
use crate::Error;
use crate::util::USER_AGENT;

const ISS_LOCATION_ENDPOINT: &str = "https://api.open-notify.org/iss-now.json";

#[derive(Deserialize)]
pub struct IssPosition {
    #[serde(rename = "latitude")]
    pub lat: String,
    #[serde(rename = "longitude")]
    pub long: String,
}

impl IssPosition {
    pub fn get_mapbox_url(&self, token: &str) -> String {
        format!(
            "https://api.mapbox.com/styles/v1/mapbox/satellite-streets-v11/static/{lat},{long},3/512x512@2x?access_token={token}",
            lat = self.lat,
            long = self.long,
            token = token,
        )
    }
}

#[derive(Deserialize)]
struct IssResponse {
    iss_position: IssPosition,
}

pub struct OpenNotifyClient {
    client: reqwest::Client,
}

impl OpenNotifyClient {
    pub async fn fetch_iss_position(&self) -> Result<IssPosition, Error> {
        let res: IssResponse = self.client.get(ISS_LOCATION_ENDPOINT)
            .send().await?
            .json().await?;

        Ok(res.iss_position)
    }
}

impl Default for OpenNotifyClient {
    fn default() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent(USER_AGENT)
                .build().expect("failed to create reqwest client")
        }
    }
}
