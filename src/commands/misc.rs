use std::time::Duration;
use humantime::{format_duration, parse_duration};
use poise::serenity_prelude::{CreateEmbed, Mentionable};
use poise::serenity_prelude as serenity;
use poise::CreateReply;

use crate::Error;
use crate::Context;


/// A ping command to test the responsiveness of the bot
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let ping_ = ctx.ping().await;
    let embed = CreateEmbed::default()
        .title("Pong!")
        .description(format!("
                Ping - {} ms
                Uptime - {}",
                ping_.as_millis(),
                format_duration(
                    Duration::from_secs(ctx.data()
                    .start_time
                    .elapsed()
                    .unwrap()
                    .as_secs())
                    )
                )
            );
    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}
