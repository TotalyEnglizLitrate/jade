use poise::serenity_prelude as serenity;
use regex;
// use poise::CreateReply;


use crate::Error;
use crate::Context;


/// Base command for purging messages
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_MESSAGES",
    subcommands("all", "bot", "contains", "embeds", "files", "human", "links", "mentions", "reactions", "user"),
    guild_only = true
    )
]
pub async fn purge(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("This is the base command and is not to be run independently").await?;
    Ok(())
}


/// Purge the last n messages - defaults to maximum 100
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_MESSAGES",
    guild_only = true
    )
]
pub async fn all(
    ctx: Context<'_>,
    #[description = "No. of Messages to purge"] amount: Option<u8>
) -> Result<(), Error> {
    let amount = amount.unwrap_or_else(|| 100);
    let channel = ctx.guild_channel().await.unwrap();

    let messages: Vec<serenity::MessageId> = channel.messages(
        &ctx.http(),
        serenity::GetMessages::new().limit(amount)
    ).await?
    .into_iter()
    .map(|msg| msg.id)
    .collect();

    ctx.say(format!("Purging {} messages", amount)).await?;
    channel.delete_messages(
        &ctx.http(),
        messages
    ).await?;
    Ok(())
}


/// Purges all messages sent by a bot in the last n messages - defaults to maximum of 100
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_MESSAGES",
    guild_only = true
    )
]
pub async fn bot(
    ctx: Context<'_>,
    #[description = "No. of Messages to search"] amount: Option<u8>
) -> Result<(), Error> {
    let amount = amount.unwrap_or_else(|| 100);
    let channel = ctx.guild_channel().await.unwrap();

    let messages: Vec<serenity::MessageId> = channel.messages(
        &ctx.http(),
        serenity::GetMessages::new().limit(amount)
    ).await?
    .into_iter()
    .filter(|msg| msg.author.bot)
    .map(|msg| msg.id)
    .collect();

    ctx.say(format!("Purging {} messages", messages.len())).await?;
    channel.delete_messages(
        &ctx.http(),
        messages
    ).await?;
    Ok(())
}


/// Purges all messages with the given word/phrase in the last n messages - defaults to maximum of 100
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_MESSAGES",
    guild_only = true
    )
]
pub async fn contains(
    ctx: Context<'_>,
    #[description = "No. of Messages to search"] amount: Option<u8>,
    #[description = "Word/Phrase to search for"] search: String
) -> Result<(), Error> {
    let amount = amount.unwrap_or_else(|| 100);
    let channel = ctx.guild_channel().await.unwrap();

    let messages: Vec<serenity::MessageId> = channel.messages(
        &ctx.http(),
        serenity::GetMessages::new().limit(amount)
    ).await?
    .into_iter()
    .filter(|msg| msg.content.contains(&search))
    .map(|msg| msg.id)
    .collect();

    ctx.say(format!("Purging {} messages", messages.len())).await?;
    channel.delete_messages(
        &ctx.http(),
        messages
    ).await?;
    Ok(())
}

/// Purges all messages with an embed in the last n messages - defaults to maximum of 100
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_MESSAGES",
    guild_only = true
    )
]
pub async fn embeds(
    ctx: Context<'_>,
    #[description = "No. of Messages to search"] amount: Option<u8>
) -> Result<(), Error> {
    let amount = amount.unwrap_or_else(|| 100);
    let channel = ctx.guild_channel().await.unwrap();

    let messages: Vec<serenity::MessageId> = channel.messages(
        &ctx.http(),
        serenity::GetMessages::new().limit(amount)
    ).await?
    .into_iter()
    .filter(|msg| msg.embeds.len() != 0)
    .map(|msg| msg.id)
    .collect();

    ctx.say(format!("Purging {} messages", messages.len())).await?;
    channel.delete_messages(
        &ctx.http(),
        messages
    ).await?;
    Ok(())
}


/// Purges all messages sent by a non bot in the last n messages - defaults to maximum of 100
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_MESSAGES",
    guild_only = true
    )
]
pub async fn human(
    ctx: Context<'_>,
    #[description = "No. of Messages to search"] amount: Option<u8>
) -> Result<(), Error> {
    let amount = amount.unwrap_or_else(|| 100);
    let channel = ctx.guild_channel().await.unwrap();

    let messages: Vec<serenity::MessageId> = channel.messages(
        &ctx.http(),
        serenity::GetMessages::new().limit(amount)
    ).await?
    .into_iter()
    .filter(|msg| !msg.author.bot)
    .map(|msg| msg.id)
    .collect();

    ctx.say(format!("Purging {} messages", messages.len())).await?;
    channel.delete_messages(
        &ctx.http(),
        messages
    ).await?;
    Ok(())
}

/// Purges all messages with attachments in the last n messages - defaults to maximum of 100
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_MESSAGES",
    guild_only = true
    )
]
pub async fn files(
    ctx: Context<'_>,
    #[description = "No. of Messages to search"] amount: Option<u8>
) -> Result<(), Error> {
    let amount = amount.unwrap_or_else(|| 100);
    let channel = ctx.guild_channel().await.unwrap();

    let messages: Vec<serenity::MessageId> = channel.messages(
        &ctx.http(),
        serenity::GetMessages::new().limit(amount)
    ).await?
    .into_iter()
    .filter(|msg| msg.attachments.len() != 0)
    .map(|msg| msg.id)
    .collect();

    ctx.say(format!("Purging {} messages", messages.len())).await?;
    channel.delete_messages(
        &ctx.http(),
        messages
    ).await?;
    Ok(())
}


/// Purges all reactions in the last n messages - defaults to maximum of 100
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_MESSAGES",
    guild_only = true
    )
]
pub async fn reactions(
    ctx: Context<'_>,
    #[description = "No. of Messages to search"] amount: Option<u8>,
) -> Result<(), Error> {
    let amount = amount.unwrap_or_else(|| 100);
    let channel = ctx.guild_channel().await.unwrap();

    let messages: Vec<serenity::MessageId> = channel.messages(
        &ctx.http(),
        serenity::GetMessages::new().limit(amount)
    ).await?
    .into_iter()
    .map(|msg| msg.id)
    .collect();

    ctx.say(format!("Purging reactions in {} messages", amount)).await?;
    for msgid in &messages {
        channel.delete_reactions(&ctx.http(), msgid).await?;
    }
    Ok(())
}


/// Purges all messages sent by a user in the last n messages - defaults to maximum of 100
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_MESSAGES",
    guild_only = true
    )
]
pub async fn user(
    ctx: Context<'_>,
    #[description = "No. of Messages to search"] amount: Option<u8>,
    #[description = "User whose messages are to be purged"] user: serenity::User
) -> Result<(), Error> {
    let amount = amount.unwrap_or_else(|| 100);
    let channel = ctx.guild_channel().await.unwrap();

    let messages: Vec<serenity::MessageId> = channel.messages(
        &ctx.http(),
        serenity::GetMessages::new().limit(amount)
    ).await?
    .into_iter()
    .filter(|msg| msg.author == user)
    .map(|msg| msg.id)
    .collect();

    ctx.say(format!("Purging {} messages", messages.len())).await?;
    channel.delete_messages(
        &ctx.http(),
        messages
    ).await?;
    Ok(())
}


/// Purges all messages containing user/role pings in the last n messages - defaults to maximum of 100
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_MESSAGES",
    guild_only = true
    )
]
pub async fn mentions(
    ctx: Context<'_>,
    #[description = "No. of Messages to search"] amount: Option<u8>
) -> Result<(), Error> {
    let amount = amount.unwrap_or_else(|| 100);
    let channel = ctx.guild_channel().await.unwrap();

    let messages: Vec<serenity::MessageId> = channel.messages(
        &ctx.http(),
        serenity::GetMessages::new().limit(amount)
    ).await?
    .into_iter()
    .filter(|msg| msg.mention_roles.len() != 0 || msg.mentions.len() != 0)
    .map(|msg| msg.id)
    .collect();

    ctx.say(format!("Purging {} messages", messages.len())).await?;
    channel.delete_messages(
        &ctx.http(),
        messages
    ).await?;
    Ok(())
}


/// Purges all messages containing links in the last n messages - defaults to maximum of 100
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_MESSAGES",
    guild_only = true
    )
]
pub async fn links(
    ctx: Context<'_>,
    #[description = "No. of Messages to search"] amount: Option<u8>
) -> Result<(), Error> {
    let amount = amount.unwrap_or_else(|| 100);
    let channel = ctx.guild_channel().await.unwrap();

    let messages: Vec<serenity::MessageId> = channel.messages(
        &ctx.http(),
        serenity::GetMessages::new().limit(amount)
    ).await?
    .into_iter()
    .filter(|msg| regex::Regex::new("https?://S+").unwrap().is_match(&msg.content))
    .map(|msg| msg.id)
    .collect();

    ctx.say(format!("Purging {} messages", messages.len())).await?;
    channel.delete_messages(
        &ctx.http(),
        messages
    ).await?;
    Ok(())
}
