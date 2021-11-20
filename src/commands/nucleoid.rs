use std::str::FromStr;
use chrono::Utc;
use crate::{Context, Error};
use crate::util::{pluralise, StarlightError};

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

/// Commands for querying information from Nucleoid's public API
#[poise::command(slash_command)]
pub async fn nucleoid(context: Context<'_>) -> Result<(), Error> {
    context.say("this is not an actual command").await?;
    Ok(())
}

/// Query the status of one of the Nucleoid servers.
#[poise::command(slash_command, rename = "status")]
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

/// List the 5 most recently played games on Nucleoid
#[poise::command(slash_command, rename = "recent-games")]
pub async fn nucleoid_recent_games(
    context: Context<'_>,
) -> Result<(), Error> {
    let recent_games = context.data().nucleoid_client.get_recent_games(5).await?;
    context.send(|m| m.embed(|e| {
        e.title("Recent games:");
        for game in recent_games {
            e.field(game.namespace, format!(
                "{}\nPlayed at: `{}` on `{}`",
                pluralise(game.players.len(), "player", "players"),
                game.date_played, game.server,
            ), false);
        }
        e.timestamp(Utc::now());
        e
    })).await?;

    Ok(())
}
