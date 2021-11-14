use crate::{Context, Error};

pub async fn only_owners(ctx: Context<'_>) -> Result<bool, Error> {
    Ok(ctx.framework().options().owners.contains(&ctx.author().id))
}
