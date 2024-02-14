use humantime::parse_duration;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::Mentionable;
// use poise::CreateReply;


use crate::Error;
use crate::Context;

/// Timeout a User
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MODERATE_MEMBERS",
    guild_only = true
    )
]
pub async fn timeout(
    ctx: Context<'_>,
    #[description = "User to timeout"] user: serenity::User,
    #[description = "Time to timeout the user for"] time: Option<String>
) -> Result<(), Error> {
    let time = time.unwrap_or_else(|| "1h".to_string());
    let mut member = ctx.http().get_member(ctx.guild_id().unwrap(), user.id).await?;
    let timeout_ts = serenity::Timestamp::from_millis(
        parse_duration(&time)
        .unwrap_or_default()
        .as_millis() as i64 + serenity::Timestamp::now().timestamp_millis()
    ).unwrap();

    if member.communication_disabled_until.unwrap_or_else(|| serenity::Timestamp::from_millis(0).unwrap()) <= serenity::Timestamp::now() {
        let _ = &member.disable_communication_until_datetime(ctx.http(), timeout_ts).await?;
        ctx.say(format!("{} was timed out for {}", user.name, time)).await?;
    }
    else {
        ctx.say(format!("User {} already under timeout", user.name)).await?;
    }
    Ok(())
}


/// Untimeout a user
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MODERATE_MEMBERS",
    guild_only = true
    )
]
pub async fn untimeout(
    ctx: Context<'_>,
    #[description = "User to untimeout"] user:serenity::User
) -> Result<(), Error> {
    let mut member = ctx.http().get_member(ctx.guild_id().unwrap(), user.id).await?;
    if member
        .communication_disabled_until
        .unwrap_or_else(|| serenity::Timestamp::from_millis(0).unwrap())
        >= serenity::Timestamp::now() {
            member.enable_communication(ctx.http()).await?;
            ctx.say(format!("Successfully removed timeout from {}", user.name)).await?;
    }
    else {
        ctx.say(format!("User {} not under timeout!", user.name)).await?;
    }
    Ok(())
}


/// Ban a member
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "BAN_MEMBERS",
    guild_only = true
    )
]
pub async fn ban(
    ctx: Context<'_>,
    #[description = "Member to ban"] user: serenity::User,
    #[description = "Reason"] mut reason: Option<String>
) -> Result<(), Error> {
    reason = Some(reason.unwrap_or_else(|| format!("Requested by {}", &ctx.author().name)));
    ctx.http()
        .ban_user(
            ctx.guild_id().unwrap(),
            user.id,
            0,
            reason.as_deref()
            ).await?;
    ctx.say(format!("Successfully banned {}", user.mention())).await?;
    Ok(())
}


/// Unban a member
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "BAN_MEMBERS",
    guild_only = true
    )
]
pub async fn unban(
    ctx: Context<'_>,
    #[description = "UID of User to unban"] user: serenity::UserId,
    #[description = "Reason"] mut reason: Option<String>
) -> Result<(), Error> {
    reason = Some(reason.unwrap_or_else(|| format!("Requested by {}", ctx.author().name)));
    ctx.http()
        .remove_ban(
            ctx.guild_id().unwrap(),
            user,
            reason.as_deref()
            ).await?;
    ctx.say(format!("Successfully unbanned {}", user.mention())).await?;
    Ok(())
}


/// Kick a member
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "KICK_MEMBERS",
    guild_only = true
    )
]
pub async fn kick(
    ctx: Context<'_>,
    #[description = "User to kick"] user: serenity::User,
    #[description = "Reason"] mut reason: Option<String>
) -> Result<(), Error> {
    reason = Some(reason.unwrap_or_else(|| format!("Requested by {}", ctx.author().name)));
    ctx.http()
        .kick_member(
            ctx.guild_id().unwrap(),
            user.id,
            reason.as_deref()
            ).await?;
    ctx.say(format!("Successfully kicked {}", user.mention())).await?;
    Ok(())
}
