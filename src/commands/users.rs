use poise::serenity_prelude as serenity;
use crate::{Context, Error};

/// Display your account age
#[poise::command(slash_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "User to query the account age for, exclude to query yourself"] user: Option<serenity::User>
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());
    ctx.say(format!(":clock4: **@{}**'s account was created at **{}**", user.tag(), user.created_at())).await?;

    Ok(())
}
