use humantime::{format_duration, parse_duration};
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::Mentionable;
use poise::CreateReply;


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


/// Lock a channel - If both user and role mentioned only locks only for role
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_CHANNELS | MANAGE_ROLES",
    guild_only = true
    )
]
pub async fn lock(
    ctx: Context<'_>,
    #[description = "Channel to lock - defaults to current channel"] channel: Option<serenity::GuildChannel>,
    #[description = "Role to lock channel for - defaults to everyone"] role: Option<serenity::Role>,
    #[description = "User to lock channel for"] user: Option<serenity::User>
) -> Result<(), Error> {
    let channel_ = ctx.guild_channel().await.unwrap();
    let channel = channel.unwrap_or_else(|| channel_);
    let user = user.unwrap_or_default();let role = role.unwrap_or_default();
    let overwrites = channel.permission_overwrites.clone();
    let ovrwrt: serenity::PermissionOverwriteType;
    if role != serenity::Role::default() {
        ovrwrt = serenity::PermissionOverwriteType::Role(role.id);
        ctx.say(format!("Locking {} for role **{}**", channel.mention(), role.name)).await?;
    }
    else if user != serenity::User::default() {
        ovrwrt = serenity::PermissionOverwriteType::Member(user.id);
        ctx.say(format!("Locking {} for user **{}**", channel.mention(), user.name)).await?;
    }
    else {
        ovrwrt = serenity::PermissionOverwriteType::Role(ctx.guild_id().unwrap().everyone_role());
        ctx.say(format!("Locking {} for everyone", channel.mention())).await?;
    }
    let mut found = false;
    for idx in 0..overwrites.len() {
        if overwrites[idx].kind == ovrwrt {
            found = true;
            channel.create_permission(
                ctx.http(),
                serenity::PermissionOverwrite {
                    allow: (overwrites[idx].allow & !serenity::Permissions::SEND_MESSAGES) & !serenity::Permissions::SEND_MESSAGES_IN_THREADS,
                    deny: overwrites[idx].deny | serenity::Permissions::SEND_MESSAGES | serenity::Permissions::SEND_MESSAGES_IN_THREADS,
                    kind: overwrites[idx].kind
                }
                ).await?;
        }
    }
    if !found {
        channel.create_permission(
            ctx.http(),
            serenity::PermissionOverwrite {
                allow: serenity::Permissions::empty(),
                deny: serenity::Permissions::SEND_MESSAGES,
                kind: ovrwrt
            }
            ).await?;
    }
    Ok(())
} 


/// Unlock a channel - If both user and role mentioned only unlocks only for role
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_CHANNELS | MANAGE_ROLES",
    guild_only = true
    )
]
pub async fn unlock(
    ctx: Context<'_>,
    #[description = "Channel to lock - defaults to current channel"] channel: Option<serenity::GuildChannel>,
    #[description = "Role to lock channel for - defaults to everyone"] role: Option<serenity::Role>,
    #[description = "User to lock channel for"] user: Option<serenity::User>,
    #[description = "State to set SEND_MESSAGES to, true for true, false for default - defaults to true"] state: Option<bool>
) -> Result<(), Error> {
    let channel_ = ctx.guild_channel().await.unwrap();
    let channel = channel.unwrap_or_else(|| channel_);
    let user = user.unwrap_or_default();let role = role.unwrap_or_default();
    let state = state.unwrap_or_else(|| true);
    let overwrites = channel.permission_overwrites.clone();
    let ovrwrt: serenity::PermissionOverwriteType;
    let allow: serenity::Permissions;
    let mut state_str = String::new();
    if state {
        allow = serenity::Permissions::SEND_MESSAGES | serenity::Permissions::SEND_MESSAGES_IN_THREADS;
        state_str.push_str("True");
    } 
    else {
        allow = serenity::Permissions::empty();
        state_str.push_str("Default");
    }
    if role != serenity::Role::default() {
        ovrwrt = serenity::PermissionOverwriteType::Role(role.id);
        ctx.say(format!("Unlocking {} for role **{}** with state {}", channel.mention(), role.name, state_str)).await?;
    }
    else if user != serenity::User::default() {
        ovrwrt = serenity::PermissionOverwriteType::Member(user.id);
        ctx.say(format!("Unlocking {} for user **{}** with state {}", channel.mention(), user.name, state_str)).await?;
    }
    else {
        ovrwrt = serenity::PermissionOverwriteType::Role(ctx.guild_id().unwrap().everyone_role());
        ctx.say(format!("Unlocking {} for everyone with state {}", channel.mention(), state_str)).await?;
    }
    let mut found = false;
    for idx in 0..overwrites.len() {
        if overwrites[idx].kind == ovrwrt {
            found = true;
            channel.create_permission(
                ctx.http(),
                serenity::PermissionOverwrite {
                    allow: allow | ((overwrites[idx].allow & !serenity::Permissions::SEND_MESSAGES) & !serenity::Permissions::SEND_MESSAGES_IN_THREADS),
                    deny: overwrites[idx].deny & !serenity::Permissions::SEND_MESSAGES & !serenity::Permissions::SEND_MESSAGES_IN_THREADS,
                    kind: overwrites[idx].kind
                }
                ).await?;
        }
    }
    if !found {
        channel.create_permission(
            ctx.http(),
            serenity::PermissionOverwrite {
                allow,
                deny: serenity::Permissions::empty(),
                kind: ovrwrt
            }
            ).await?;
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


/// Give a role to a member
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MODERATE_MEMBERS",
    guild_only = true
    )
]
pub async fn role(
    ctx: Context<'_>,
    #[description = "Member to add role to"] user: serenity::User,
    #[description = "Role to add"] role: serenity::Role
) -> Result<(), Error> {
    let member = ctx.http().get_member(ctx.guild_id().unwrap(), user.id).await?;
    let highest_role = member.highest_role_info(ctx.cache())
        .unwrap_or_default().0
        .to_role_cached(ctx.cache())
        .unwrap_or_else(
            || ctx.guild_id().unwrap().everyone_role().to_role_cached(ctx.cache()
        ).unwrap());
    if role.position >= highest_role.position {
        ctx.say("Cannot assign role higher than your highest role").await?;
        return Ok(())
    }
    if user.has_role(ctx.http(), ctx.guild_id().unwrap(), role.id).await? {
        member.remove_role(ctx.http(), role.id).await?;
        ctx.say(format!("Successfully removed role **{}** from **{}**", role.name, user.name)).await?;
    }
    else {
        member.add_role(ctx.http(), role.id).await?;
        ctx.say(format!("Successfully added role **{}** to **{}**", role.name, user.name)).await?;
    }
    Ok(())
}


/// Viewlock a channel - If both user and role mentioned only locks only for role
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_CHANNELS | MANAGE_ROLES",
    guild_only = true
    )
]
pub async fn viewlock(
    ctx: Context<'_>,
    #[description = "Channel to viewlock - defaults to current channel"] channel: Option<serenity::GuildChannel>,
    #[description = "Role to viewlock channel for - defaults to everyone"] role: Option<serenity::Role>,
    #[description = "User to viewlock channel for"] user: Option<serenity::User>
) -> Result<(), Error> {
    let channel_ = ctx.guild_channel().await.unwrap();
    let channel = channel.unwrap_or_else(|| channel_);
    let user = user.unwrap_or_default();let role = role.unwrap_or_default();
    let overwrites = channel.permission_overwrites.clone();
    let ovrwrt: serenity::PermissionOverwriteType;
    if role != serenity::Role::default() {
        ovrwrt = serenity::PermissionOverwriteType::Role(role.id);
        ctx.say(format!("Viewlocking {} for role **{}**", channel.mention(), role.name)).await?;
    }
    else if user != serenity::User::default() {
        ovrwrt = serenity::PermissionOverwriteType::Member(user.id);
        ctx.say(format!("Viewlocking {} for user **{}**", channel.mention(), user.name)).await?;
    }
    else {
        ovrwrt = serenity::PermissionOverwriteType::Role(ctx.guild_id().unwrap().everyone_role());
        ctx.say(format!("Viewlocking {} for everyone", channel.mention())).await?;
    }
    let mut found = false;
    for idx in 0..overwrites.len() {
        if overwrites[idx].kind == ovrwrt {
            found = true;
            channel.create_permission(
                ctx.http(),
                serenity::PermissionOverwrite {
                    allow: overwrites[idx].allow & !serenity::Permissions::VIEW_CHANNEL,
                    deny: overwrites[idx].deny | serenity::Permissions::VIEW_CHANNEL,
                    kind: overwrites[idx].kind
                }
                ).await?;
        }
    }
    if !found {
        channel.create_permission(
            ctx.http(),
            serenity::PermissionOverwrite {
                allow: serenity::Permissions::empty(),
                deny: serenity::Permissions::VIEW_CHANNEL,
                kind: ovrwrt
            }
            ).await?;
    }
    Ok(())
} 


/// Unviewlock a channel - If both user and role mentioned only unlocks only for role
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_CHANNELS | MANAGE_ROLES",
    guild_only = true
    )
]
pub async fn unviewlock(
    ctx: Context<'_>,
    #[description = "Channel to unviewlock - defaults to current channel"] channel: Option<serenity::GuildChannel>,
    #[description = "Role to unviewlock channel for - defaults to everyone"] role: Option<serenity::Role>,
    #[description = "User to unviewlock channel for"] user: Option<serenity::User>,
    #[description = "State to set VIEW_CHANNEL to, true for true, false for default - defaults to true"] state: Option<bool>
) -> Result<(), Error> {
    let channel_ = ctx.guild_channel().await.unwrap();
    let channel = channel.unwrap_or_else(|| channel_);
    let user = user.unwrap_or_default();let role = role.unwrap_or_default();
    let state = state.unwrap_or_else(|| true);
    let overwrites = channel.permission_overwrites.clone();
    let ovrwrt: serenity::PermissionOverwriteType;
    let allow: serenity::Permissions;
    let mut state_str = String::new();
    if state {
        allow = serenity::Permissions::VIEW_CHANNEL;
        state_str.push_str("True");
    } 
    else {
        allow = serenity::Permissions::empty();
        state_str.push_str("Default")
    }
    if role != serenity::Role::default() {
        ovrwrt = serenity::PermissionOverwriteType::Role(role.id);
        ctx.say(format!("Unviewlocking {} for role **{}** with state {}", channel.mention(), role.name, state_str)).await?;
    }
    else if user != serenity::User::default() {
        ovrwrt = serenity::PermissionOverwriteType::Member(user.id);
        ctx.say(format!("Unviewlocking {} for user **{}** with state {}", channel.mention(), user.name, state_str)).await?;
    }
    else {
        ovrwrt = serenity::PermissionOverwriteType::Role(ctx.guild_id().unwrap().everyone_role());
        ctx.say(format!("Unviewlocking {} for everyone with state {}", channel.mention(), state_str)).await?;
    }
    let mut found = false;
    for idx in 0..overwrites.len() {
        if overwrites[idx].kind == ovrwrt {
            found = true;
            channel.create_permission(
                ctx.http(),
                serenity::PermissionOverwrite {
                    allow: allow | (overwrites[idx].allow & !serenity::Permissions::VIEW_CHANNEL),
                    deny: overwrites[idx].deny & !serenity::Permissions::VIEW_CHANNEL,
                    kind: overwrites[idx].kind
                }
                ).await?;
        }
    }
    if !found {
        channel.create_permission(
            ctx.http(),
            serenity::PermissionOverwrite {
                allow,
                deny: serenity::Permissions::empty(),
                kind: ovrwrt
            }
            ).await?;
    }
    Ok(())
}

