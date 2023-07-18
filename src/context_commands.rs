use crate::checks::is_master_admin;
use crate::checks::is_on_admin_list;
use crate::extensions::InteractiveSnowflakeExt;
use crate::state::role_backups::RoleBackups;
use crate::state::t_rooms::TRooms;
use crate::Context;
use crate::Error;
use poise::serenity_prelude::CacheHttp;
use poise::serenity_prelude::ChannelId;
use poise::serenity_prelude::RoleId;
use poise::serenity_prelude::{self as serenity};
use std::format;
use std::str::FromStr;

/// "User is triggered, contain the cancer!"
#[poise::command(
    ephemeral,
    check = "is_on_admin_list",
    context_menu_command = "Triggered!"
)]
pub async fn triggered(
    ctx: Context<'_>,
    #[description = "The triggered user"] user: serenity::User,
) -> Result<(), Error> {
    if is_master_admin(&user) {
        ctx.say("Nice try...").await?;
        return Ok(());
    }

    let author = ctx.author();

    let http = ctx.http();
    let member = ctx.guild_id();

    if let Some(member) = member {
        let mut member = member.member(http, user.id).await?;

        // Store all user roles into state
        let mut role_backups = RoleBackups::load(ctx.data())?;
        let success = role_backups.add(ctx.data(), user.id, &member.roles)?;

        if success {
            // Remove all current roles
            member.remove_roles(http, &member.roles.clone()).await?;

            // Give "triggered" role to user
            member
                .add_role(http, RoleId::from_str(ctx.data().triggered_role.as_str())?)
                .await?;

            // Get the current state of all TRooms
            let mut t_rooms = TRooms::load(ctx.data())?;

            let open_room = t_rooms.find_room(ctx.data())?;

            if let Some(open_room) = open_room {
                let role = open_room.0;
                let channel = open_room.1;
                let channel = ChannelId(channel);
                member.add_role(http, role).await?;

                let message = format!("{}\nYou have been pulled into a private room by a moderator. Please wait for details...", member.user.id.get_interactive());
                channel.say(http, message).await?;
                ctx.say("Retard has been contained...").await?;
            } else {
                let message = format!(
                    "{}\nAll triggered rooms are full. Contact bot admin for details...",
                    author.id.get_interactive()
                );
                ctx.say(message).await?;
            }
        } else {
            ctx.say(format!(
                "{} is already triggered...",
                user.id.get_interactive()
            ))
            .await?;
        }
    } else {
        ctx.say("Member could not be found...").await?;
    }

    Ok(())
}

/// "Cancer contained, release the retard..."
#[poise::command(
    ephemeral,
    check = "is_on_admin_list",
    context_menu_command = "Release Trigger"
)]
pub async fn release_trigger(
    ctx: Context<'_>,
    #[description = "The triggered user"] user: serenity::User,
) -> Result<(), Error> {
    let http = ctx.http();
    let member = ctx.guild_id();

    if let Some(member) = member {
        let mut member = member.member(http, user.id).await?;
        let author = ctx.author();
        // Extract, then delete stored user roles
        let mut role_backups = RoleBackups::load(ctx.data())?;
        let extracted_roles = role_backups.remove(ctx.data(), user.id)?;

        if let Some(extracted_roles) = extracted_roles {
            // Remove currently assigned roles
            member.remove_roles(http, &member.roles.clone()).await?;

            let extracted_roles: Vec<RoleId> =
                extracted_roles.into_iter().map(|x| RoleId(x)).collect();

            // Assign extracted roles
            member.add_roles(http, &extracted_roles).await?;

            let message = format!("{}\nRelease of retard successful.\n\nPlease remember to use the `/unlock_triggered_channel` command on any cancer containment channel to clear the old conversation and unlock it for the future use.\n\nONLY DO THIS AFTER THE CONVERSATION HAS BEEN REVIEWED AND IMPORTANT SCREENSHOTS HAVE BEEN TAKEN!!", author.id.get_interactive());
            ctx.say(message).await?;
        } else {
            ctx.say(format!(
                "{} is has already been released...",
                user.id.get_interactive()
            ))
            .await?;
        }
    } else {
        ctx.say("Member could not be found...").await?;
    }

    Ok(())
}
