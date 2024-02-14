use poise::serenity_prelude as serenity;
use poise::serenity_prelude::Mentionable;
// use poise::CreateReply;


use crate::Error;
use crate::Context;

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
    let state = state.unwrap_or_default();
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
