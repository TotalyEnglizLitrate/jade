use std::time::SystemTime;
use anyhow::Context as _;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use shuttle_secrets::SecretStore;
use shuttle_serenity::ShuttleSerenity;

pub mod commands;
pub struct Data {
    pub start_time: std::time::SystemTime
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;


#[shuttle_runtime::main]
async fn main(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::misc::ping(),
                commands::misc::help(),
                commands::moderation::user::timeout(),
                commands::moderation::user::untimeout(),
                commands::moderation::user::ban(),
                commands::moderation::user::unban(),
                commands::moderation::user::kick(),
                commands::moderation::user::role(),
                commands::moderation::channel::lock(),
                commands::moderation::channel::unlock(),
                commands::moderation::channel::viewlock(),
                commands::moderation::channel::unviewlock(),
                commands::moderation::purge::purge()
            ],
            prefix_options: poise::PrefixFrameworkOptions { prefix: Some("j.".into()), ..Default::default()},
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {start_time: SystemTime::now()})
            })
        })
        .build();

    let client = ClientBuilder::new(discord_token, GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT)
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;
    Ok(client.into())
}
