use std::path::Path;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct StarlightConfig {
    pub discord: DiscordConfig,
    pub osu: OsuConfig,
    pub mapbox: Option<MapboxConfig>,
}

#[derive(Deserialize, Clone)]
pub struct DiscordConfig {
    pub token: String,
}

#[derive(Deserialize, Clone)]
pub struct OsuConfig {
    pub client_id: u64,
    pub client_secret: String,
}

#[derive(Deserialize, Clone)]
pub struct MapboxConfig {
    pub token: String,
}

pub fn load() -> StarlightConfig {
    let path = Path::new("Starlight.toml");
    if !path.exists() {
        error!("Missing Starlight.toml");
        std::process::exit(1);
    } else {
        let config = std::fs::read_to_string(path).expect("failed to read config file Starlight.toml");
        toml::from_str(&config).expect("failed to parge Starlight.toml")
    }
}
