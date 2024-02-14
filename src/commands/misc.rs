use std::time::Duration;
use humantime::format_duration;
use poise::serenity_prelude::CreateEmbed;
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

/// Show this menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
    let config = poise::builtins::HelpConfiguration {
        show_subcommands: true,
        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}
