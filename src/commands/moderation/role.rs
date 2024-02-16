use humantime::{format_duration, parse_rfc3339};
use poise;
use poise::serenity_prelude as serenity;
use std::time::Duration;
use std::vec;

use crate::Context;
use crate::Error;

fn highest_role(ctx: &Context<'_>, member: &serenity::Member) -> serenity::Role {
    member
        .highest_role_info(ctx.cache())
        .unwrap_or_default()
        .0
        .to_role_cached(ctx.cache())
        .unwrap_or_else(|| {
            ctx.guild_id()
                .unwrap()
                .everyone_role()
                .to_role_cached(ctx.cache())
                .unwrap()
        })
}

/// Base command for role management - Give a role to/remove a role from a member
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_ROLES",
    guild_only = true,
    subcommands("create", "add", "remove", "info", "rall", "edit")
)]
pub async fn role(
    ctx: Context<'_>,
    #[description = "Member to add/remove role to/from"] user: serenity::User,
    #[description = "Role to add/remove"] role: serenity::Role,
) -> Result<(), Error> {
    let member = ctx
        .http()
        .get_member(ctx.guild_id().unwrap(), user.id)
        .await?;
    let highest_role = highest_role(&ctx, &ctx.author_member().await.unwrap());
    if role.position >= highest_role.position {
        ctx.say("Cannot assign role higher than your highest role")
            .await?;
        return Ok(());
    }
    if user
        .has_role(ctx.http(), ctx.guild_id().unwrap(), role.id)
        .await?
    {
        member.remove_role(ctx.http(), role.id).await?;
        ctx.say(format!(
            "Successfully removed role **{}** from **{}**",
            role.name, user.name
        ))
        .await?;
    } else {
        member.add_role(ctx.http(), role.id).await?;
        ctx.say(format!(
            "Successfully added role **{}** to **{}**",
            role.name, user.name
        ))
        .await?;
    }
    Ok(())
}

/// Create a new role with the given parameters
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_ROLES",
    guild_only = true
)]
pub async fn create(
    ctx: Context<'_>,
    #[description = "Name of the role"] name: String,
    #[description = "Colo(u)r of the role"] colour: Option<String>,
    #[description = "Whether the role should be pingable - defaults to false"] mentionable: Option<
        bool,
    >,
    #[description = "Whether the role should be hoisted - defaults to false"] hoisted: Option<bool>,
) -> Result<(), Error> {
    let colour = colour.unwrap_or_else(|| String::from("000000"));
    let hex: serenity::Colour;
    if regex::Regex::new("^#?[0-9a-fA-F]{6}$")
        .unwrap()
        .is_match(&colour)
    {
        if colour.len() == 7 {
            let colour = &colour[1..6];
        }
        hex = serenity::Colour(u32::from_str_radix(&colour, 16).unwrap());
    } else {
        hex = serenity::Colour(u32::from_str_radix(&colour, 16).unwrap());
    }
    let mentionable = mentionable.unwrap_or_else(|| false);
    let hoisted = hoisted.unwrap_or_else(|| false);
    let role = serenity::EditRole::new()
        .name(&name)
        .colour(hex)
        .mentionable(mentionable)
        .hoist(hoisted);
    ctx.guild_id()
        .unwrap()
        .create_role(ctx.http(), role)
        .await?;
    ctx.say(format!("Successfully created role {}", &name))
        .await?;
    Ok(())
}

/// Add a role to a member
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_ROLES",
    guild_only = true
)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "Member to add role to"] user: serenity::User,
    #[description = "Role to add"] role: serenity::Role,
) -> Result<(), Error> {
    let member = ctx
        .http()
        .get_member(ctx.guild_id().unwrap(), user.id)
        .await?;
    let highest_role = highest_role(&ctx, &ctx.author_member().await.unwrap());
    if role.position >= highest_role.position {
        ctx.say("Cannot assign role higher than your highest role")
            .await?;
        return Ok(());
    }
    if user
        .has_role(ctx.http(), ctx.guild_id().unwrap(), role.id)
        .await?
    {
        ctx.say(format!(
            "**{}** already has role **{}**",
            user.name, role.name
        ))
        .await?;
    } else {
        member.add_role(ctx.http(), role.id).await?;
        ctx.say(format!(
            "Successfully added role **{}** to **{}**",
            role.name, user.name
        ))
        .await?;
    }
    Ok(())
}

/// Remove a role from a member
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_ROLES",
    guild_only = true
)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "Member to remove role from"] user: serenity::User,
    #[description = "Role to remove"] role: serenity::Role,
) -> Result<(), Error> {
    let member = ctx
        .http()
        .get_member(ctx.guild_id().unwrap(), user.id)
        .await?;
    let highest_role = highest_role(&ctx, &ctx.author_member().await.unwrap());
    if role.position >= highest_role.position {
        ctx.say("Cannot assign role higher than your highest role")
            .await?;
        return Ok(());
    }
    if user
        .has_role(ctx.http(), ctx.guild_id().unwrap(), role.id)
        .await?
    {
        member.remove_role(ctx.http(), role.id).await?;
        ctx.say(format!(
            "Successfully removed role **{}** from **{}**",
            role.name, user.name
        ))
        .await?;
    } else {
        ctx.say(format!(
            "**{}** already has role **{}**",
            user.name, role.name
        ))
        .await?;
    }
    Ok(())
}

/// Get info about a given role
#[poise::command(slash_command, prefix_command, guild_only = true)]
pub async fn info(
    ctx: Context<'_>,
    #[description = "Role whose info you want to get"] role: serenity::Role,
) -> Result<(), Error> {
    let guild = &ctx.cache().guild(&ctx.guild_id().unwrap()).unwrap().clone();
    let membercount: u64;
    if &ctx.guild_id().unwrap().everyone_role() == &role.id {
        membercount = guild.member_count;
    }
    else {
        let members_map = guild.members.clone();
        let members: Vec<&serenity::Member> = members_map
        .iter()
        .filter_map(|(_, member)| if member.roles.contains(&role.id) {Some(member)} else {None})
        .collect();
        membercount = members.len() as u64;
    }
    let embed = serenity::CreateEmbed::new()
        .color(role.colour)
        .title(&role.name)
        .description(format!(
            "Colour     : #{}
                Created     : <t:{}:R> ({} ago)
                ID          : {}
                Member Count: {}",
            &role.colour.hex(),
            &role.id.created_at().unix_timestamp(),
            format_duration(Duration::from_secs(
                parse_rfc3339(&role.id.created_at().to_rfc3339().unwrap())
                    .unwrap()
                    .elapsed()
                    .unwrap()
                    .as_secs()
            )),
            &role.id,
            &membercount
        ));
    let reply = poise::CreateReply::default().embed(embed).reply(true);
    ctx.send(reply).await?;
    Ok(())
}


/// Remove a role from all members
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_ROLES",
    guild_only = true
)]
pub async fn rall(
    ctx: Context<'_>,
    #[description = "Role to remove"] role: serenity::Role
) -> Result<(), Error> {
    if &role.id == &ctx.guild_id().unwrap().everyone_role() {
        ctx.say("Cannot remove the everyone role from everyone").await?;
        return Ok(())
    }
    else if role.position >= 
        highest_role(&ctx, &ctx.author_member().await.unwrap()).position {
        ctx.say("Cannot assign role higher than your highest role")
        .await?;
        return Ok(())
    }
    let guild = &ctx.cache().guild(&ctx.guild_id().unwrap()).unwrap().clone();
    let members_with_role: Vec<&serenity::Member> = guild.members.iter()
        .filter_map(|(_, mbr)| if mbr.roles.contains(&role.id) {Some(mbr)} else {None})
        .collect();
    let embed = serenity::CreateEmbed::new()
        .description(format!("Confirm removal of **{}** from **_{}_** members?", &role.name, &members_with_role.len()));
    let actionrow: serenity::CreateActionRow = serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new("Yes").emoji('✅'),
            serenity::CreateButton::new("No").emoji('❌')
        ]
    );
    let mut msg = ctx.send(
        poise::CreateReply::default()
        .embed(embed)
        .components(vec![actionrow])
    ).await?
    .into_message().await?;
    let interaction = msg
    .await_component_interaction(ctx.clone())
    .timeout(Duration::from_secs(60 * 5))
    .author_id(ctx.author().id)
    .await;
    let confirm: bool;
    if let Some(interaction) = interaction {
        interaction.defer(&ctx.http()).await?;
        match &*interaction.data.custom_id {
            "Yes" => confirm = true,
            "No" => confirm = false,
            _ => unreachable!()
        }
    } else {
        msg.edit(
            ctx.http(),
            serenity::EditMessage::default()
            .embed(serenity::CreateEmbed::default()
            .description(r"Action timed out")
        )).await?;
        return Ok(());
    }
    if confirm {
        msg.edit(
            ctx.http(),
            serenity::EditMessage::default()
            .content(format!("Removing **{}** from {} users", &role.id, &members_with_role.len()))
            .suppress_embeds(true)
            .components(vec![])
        ).await?;
        let mut fail = 0u32;
        for member in &members_with_role {
            match member.remove_role(&ctx.http(), &role.id).await {
                Err(_) => fail += 1,
                Ok(_) => fail += 0
            }
        }
        msg.edit(
            ctx.http(),
            serenity::EditMessage::default()
            .content(format!(
                "Removed **{}** from {} users, Failed to remove role from {} users",
                &role.id,
                &members_with_role.len() - fail as usize,
                &fail))
        ).await?;
    }
    else {
        msg.edit(
            ctx.http(),
            serenity::EditMessage::default()
            .suppress_embeds(true)
            .content("Action cancelled by user")
            .components(vec![])
        ).await?;
    }
    Ok(())
}


/// Edit a role
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_ROLES",
    guild_only = true
)]
pub async fn edit(
    ctx: Context<'_>,
    #[description = "Role to edit"] mut role: serenity::Role,
    #[description = "Name of the role"] name: Option<String>,
    #[description = "Colo(u)r of the role"] colour: Option<String>,
    #[description = "Whether the role should be hoisted"] hoisted: Option<bool>,
) -> Result<(), Error> {
    let colour = colour.unwrap_or_else(|| String::from("000000"));
    let hex: serenity::Colour;
    if regex::Regex::new("^#?[0-9a-fA-F]{6}$")
        .unwrap()
        .is_match(&colour)
    {
        if colour.len() == 7 {
            let colour = &colour[1..6];
        }
        hex = serenity::Colour(u32::from_str_radix(&colour, 16).unwrap());
    } else {
        hex = serenity::Colour(u32::from_str_radix(&colour, 16).unwrap());
    }
    let rl_edit = serenity::EditRole::new()
        .colour(hex)
        .hoist(hoisted.unwrap_or_else(|| role.clone().hoist))
        .name(&name.unwrap_or_else(|| role.clone().name));
    &role.edit(ctx.http(), rl_edit);
    Ok(())
}