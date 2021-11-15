use std::str::FromStr;
use chrono::Utc;
use crate::{Context, Error};
use crate::util::StarlightError;

pub enum NucleoidServer {
    Play,
    Build,
}

impl FromStr for NucleoidServer {
    type Err = StarlightError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_lowercase() {
            "play" => Ok(NucleoidServer::Play),
            "build" => Ok(NucleoidServer::Build),
            _ => Err(StarlightError::NotFound),
        }
    }
}

impl NucleoidServer {
    fn as_str(&self) -> &'static str {
        match *self {
            NucleoidServer::Play => "play",
            NucleoidServer::Build => "build",
        }
    }
}

/// Query the status of one of the Nucleoid servers.
#[poise::command(slash_command)]
pub async fn nucleoid_status(
    context: Context<'_>,
    #[description = "The server to query information about. Either 'play' or 'build'."] server: NucleoidServer
) -> Result<(), Error> {
    let status = context.data().nucleoid_client.get_status(server.as_str()).await?;
    context.send(|m| m.embed(|e| {
        e.title(format!("Nucleoid {} status", server.as_str()));
        e.description(format!("Join at `{}` using Minecraft `{}`", status.server_ip, status.game_version));
        if status.games.len() > 0 {
            e.field("Games open:", status.games.len(), true);
        }
        if status.players.len() > 0 {
            e.field("Players online", status.players.len(), true);
        }
        e.timestamp(Utc::now());
        e
    })).await?;

    Ok(())
}
