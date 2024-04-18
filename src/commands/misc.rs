use std::time::Duration;
use humantime::format_duration;
use poise::serenity_prelude::CreateEmbed;
use poise::CreateReply;
use poise::serenity_prelude as serenity;

use crate::Error;
use crate::Context;


/// A ping command to test the responsiveness of the bot
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let ping_ = ctx.ping().await;
    let embed = CreateEmbed::default()
        .title("Pong!")
        .description(format!("Ping - {} ms\nUptime - {}",
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

/// Show this menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
    let config = poise::builtins::PrettyHelpConfiguration {
        ..Default::default()
    };
    poise::builtins::pretty_help(ctx, command.as_deref(), config).await?;
    Ok(())
}



/// DM anyone anonymously
#[poise::command(slash_command)]
pub async fn dm(
    ctx: Context<'_>,
    #[description = "member to send dm to"] user: serenity::User,
    #[description = "message to send"] msg: String
) -> Result<(), Error> {
    if ctx.author().id != serenity::UserId::new(925033181990756392) && ctx.author().id != serenity::UserId::new(1031158144795156511) {
        ctx.say("Unauthorized").await?;
        return Ok(());
    }
    user.dm(ctx.http(), serenity::CreateMessage::new().content(msg)).await?;
    ctx.send(CreateReply::default().content("sent hehe").ephemeral(true)).await?;
    Ok(())
}
