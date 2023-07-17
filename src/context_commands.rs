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
    let http = ctx.http();
    let mut member = ctx.guild_id().unwrap().member(http, user.id).await?;

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
            ctx.say("All triggered rooms are full. Contact bot admin for details...")
                .await?;
        }
    } else {
        ctx.say(format!(
            "{} is already triggered...",
            user.id.get_interactive()
        ))
        .await?;
    }

    Ok(())
}
