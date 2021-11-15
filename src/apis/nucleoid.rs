use serde::Deserialize;
use crate::Error;
use crate::util::{StarlightError, USER_AGENT};

const NUCLEOID_API_BASE: &str = "https://api.nucleoid.xyz";

pub struct NucleoidClient {
    client: reqwest::Client,
}

impl NucleoidClient {
    pub async fn get_status(&self, server: &str) -> Result<ServerStatus, Error> {
        let res = self.client.get(format!("{}/status/{}", NUCLEOID_API_BASE, server))
            .send().await?;

        if res.status().is_client_error() {
            Err(StarlightError::NotFound.into())
        } else {
            Ok(res.json().await?)
        }
    }
}

impl Default for NucleoidClient {
    fn default() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent(USER_AGENT)
                .build().expect("failed to create reqwest client")
        }
    }
}

// Responses

#[derive(Deserialize)]
pub struct ServerStatus {
    pub game_version: String,
    pub server_ip: String,
    pub games: Vec<GameStatus>,
    pub players: Vec<StatusPlayer>,
}

#[derive(Deserialize)]
pub struct GameStatus {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub player_count: u16,
}

#[derive(Deserialize)]
pub struct StatusPlayer {
    pub name: String,
    pub id: uuid::Uuid,
}
