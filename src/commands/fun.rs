use poise::serenity_prelude::colours::branding::BLURPLE;
use crate::{Context, Error};

/// Get the current location of the international space station
#[poise::command(slash_command)]
pub async fn iss(context: Context<'_>) -> Result<(), Error> {
    let location = context.data().open_notify_client.fetch_iss_position().await?;

    context.send(|m| m.embed(|e| {
        e.title("Current ISS location");
        e.field("Latitude", &location.lat, true);
        e.field("Longitude", &location.long, true);
        e.image(location.get_mapbox_url(&context.data().config.mapbox.token));
        e.colour(BLURPLE);
        e
    })).await?;

    Ok(())
}
