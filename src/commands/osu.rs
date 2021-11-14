use crate::{Context, Error};
use rosu_v2::prelude::*;

/// Query a player's osu! statistics.
#[poise::command(slash_command, rename = "osu-stats")]
pub async fn osu_stats(
    context: Context<'_>,
    #[description = "The username or ID of the player"] username: String,
) -> Result<(), Error> {
    osu_stats_inner(context, username, GameMode::STD).await
}

/// Query a player's osu!standard statistics.
#[poise::command(slash_command, rename = "standard")]
pub async fn osu_stats_standard(
    context: Context<'_>,
    #[description = "The username or ID of the player"] username: String,
) -> Result<(), Error> {
    osu_stats_inner(context, username, GameMode::STD).await
}

/// Query a player's osu!catch statistics.
#[poise::command(slash_command, rename = "catch")]
pub async fn osu_stats_catch(
    context: Context<'_>,
    #[description = "The username or ID of the player"] username: String,
) -> Result<(), Error> {
    osu_stats_inner(context, username, GameMode::CTB).await
}

/// Query a player's osu!taiko statistics.
#[poise::command(slash_command, rename = "taiko")]
pub async fn osu_stats_taiko(
    context: Context<'_>,
    #[description = "The username or ID of the player"] username: String,
) -> Result<(), Error> {
    osu_stats_inner(context, username, GameMode::TKO).await
}

/// Query a player's osu!standard statistics.
#[poise::command(slash_command, rename = "mania")]
pub async fn osu_stats_mania(
    context: Context<'_>,
    #[description = "The username or ID of the player"] username: String,
) -> Result<(), Error> {
    osu_stats_inner(context, username, GameMode::MNA).await
}

async fn osu_stats_inner(
    context: Context<'_>,
    username: String,
    game_mode: GameMode,
) -> Result<(), Error> {
    let osu = &context.data().osu_client;
    let user = osu.user(&username)
        .mode(game_mode)
        .await;

    if let Err(OsuError::NotFound) = user {
        context.say(format!(":warning: Could not find a user with the username {}", &username))
            .await?;
    } else {
        let user = user?;

        context.send(|c| c.embed(|e| {
            e.colour(0xff66aa);
            e.title(format!("{}'s osu!{} stats", &user.username, mode_name(&user.mode)));
            e.thumbnail(&user.avatar_url);
            e.url(format!("https://osu.ppy.sh/users/{}", &user.user_id));
            e.field("Location", format!(":flag_{}: {}", &user.country_code.to_lowercase(), &user.country), true);
            if let Some(stats) = user.statistics {
                e.field("Accuracy", stats.accuracy, true);
                e.field("Is ranked?", if stats.is_ranked { "yes" } else { "no" }, true);

                if let Some(country_rank) = stats.country_rank {
                    e.field("Country rank", country_rank, true);
                } else {
                    e.field("Country rank", "Unranked", true);
                }

                if let Some(global_rank) = stats.global_rank {
                    e.field("Global rank", global_rank, true);
                } else {
                    e.field("Global rank", "Unranked", true);
                }

                e.field("Level", format!("{} ({}%)", stats.level.current, stats.level.progress), true);
                e.field("Max combo", stats.max_combo, true);
                e.field("Play count", stats.playcount, true);
                e.field("PP", stats.pp, true);
                e.field("Ranked score", stats.ranked_score, true);
                e.field("Total hits", stats.total_hits, true);
                e.field("Total score", stats.total_score, true);
            } else {
                e.field("No statistics", format!("{} has no statistics for osu!{}", &user.username, mode_name(&user.mode)), false);
            }
            e
        })).await?;
    }

    Ok(())
}

fn mode_name(mode: &GameMode) -> &'static str {
    match mode {
        GameMode::STD => "",
        GameMode::TKO => "taiko",
        GameMode::CTB => "catch",
        GameMode::MNA => "mania",
    }
}
