use poise::serenity_prelude as serenity;
use poise;

use crate::Error;
use crate::Context;

/// Base command for role management - Give a role to a member
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_ROLES",
    guild_only = true,
     // subcommands = ("add", "remove", "rall", "custom", )
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
