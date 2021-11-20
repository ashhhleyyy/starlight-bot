use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::ButtonStyle;
use crate::util::{GIT_BUILD_HASH, GIT_LOG, VERSION};

/// Send the current shard latency.
#[poise::command(slash_command, track_edits)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let shard_id = ctx.discord().shard_id;

    let ws_latency = {
        let shard_manager = ctx.framework().shard_manager();

        let manager = shard_manager.lock().await;
        let runners = manager.runners.lock().await;

        let runner = runners.get(&serenity::ShardId(shard_id)).unwrap();

        if let Some(duration) = runner.latency {
            format!("{:.2}ms", duration.as_millis())
        } else {
            "?ms".to_string()
        }
    };

    ctx.say(format!("The shard latency for **Shard {}** is **{}**", shard_id, ws_latency)).await?;

    Ok(())
}


/// Register application commands in this guild or globally
///
/// Run with no arguments to register in guild, run with argument "global" to register globally.
#[poise::command(prefix_command, hide_in_help, check = "crate::checks::only_owners")]
pub async fn register_commands(ctx: Context<'_>, #[flag] global: bool) -> Result<(), Error> {
    poise::builtins::register_application_commands(ctx, global).await?;
    Ok(())
}

/// Show information about the bot
#[poise::command(slash_command)]
pub async fn about(context: Context<'_>) -> Result<(), Error> {
    context.send(|m| m.embed(|e| {
        e.title("Starlight");
        e.description("Starlight is a general purpose Discord bot featuring commands for fun, games and other random things!");
        e.field("Open source <3", "Starlight is open-source and licensed under the Mozilla Public License 2.0!", false);
        e.field("Latest changes:", GIT_LOG.replace("\\n", "\n"), false);
        e.footer(|f| {
            f.text(format!("Starlight {} ({}) | Created with ❤️ by Ash!", VERSION, GIT_BUILD_HASH));
            f
        });
        e
    }).components(|c| c.create_action_row(|row| row.create_button(|b| {
        b.url("https://github.com/ashisbored/starlight-bot");
        b.label("Source code!");
        b.style(ButtonStyle::Link);
        b
    })))).await?;
    Ok(())
}
