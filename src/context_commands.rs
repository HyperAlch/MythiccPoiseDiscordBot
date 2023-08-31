use crate::checks::is_master_admin;
use crate::checks::is_on_admin_list;
use crate::extensions::InteractiveSnowflakeExt;
use crate::state::role_backups::RoleBackups;
use crate::state::t_rooms::TRooms;
use crate::state::Data;
use crate::utils::discord_cdn::get_avatar_url;
use crate::Context;
use crate::Error;
use poise::serenity_prelude::colours::roles::DARK_PURPLE;
use poise::serenity_prelude::CacheHttp;
use poise::serenity_prelude::ChannelId;
use poise::serenity_prelude::CreateEmbedAuthor;
use poise::serenity_prelude::RoleId;
use poise::serenity_prelude::{self as serenity};
use poise::Modal;
use std::format;
use std::str::FromStr;

type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, Error>;

#[derive(Debug, Modal)]
#[name = "Apply to guild"] // Struct name by default
struct GuildApplyUserModal {
    #[name = "Your EXACT in-game name"] // Field name by default
    #[placeholder = "leeroy jenkins"] // No placeholder by default
    #[min_length = 1] // No length restriction by default (so, 1-4000 chars)
    #[max_length = 32]
    in_game_name: String,
}

/// "Apply to the guild"
#[poise::command(context_menu_command = "Guild Apply", ephemeral)]
pub async fn archeage_apply(
    ctx: ApplicationContext<'_>,
    #[description = "Apply for ArcheAge guild membership"] user: serenity::User,
) -> Result<(), Error> {
    let author = ctx.author();
    if &user != author {
        ctx.say("Select YOURSELF retard!").await?;
        return Ok(());
    }

    let http = ctx.serenity_context().http();
    let needs_to_apply_role = ctx.data().needs_to_apply_role.as_ref();
    let needs_to_apply_role = RoleId::from_str(&needs_to_apply_role).unwrap();
    let has_role = user
        .has_role(http, ctx.guild_id().unwrap(), needs_to_apply_role)
        .await?;

    if !has_role {
        ctx.say("Application already sent OR you never selected a supported game while joining")
            .await?;
        return Ok(());
    }
    let res = GuildApplyUserModal::execute(ctx).await?;

    let res = match res {
        Some(ref modal_data) => {
            let needs_to_apply_channel = ctx.data().needs_to_apply_channel.as_ref();
            let needs_to_apply_channel = ChannelId::from_str(&needs_to_apply_channel)?;

            let member = ctx.guild_id().unwrap();
            let mut member = member.member(http, user.id).await?;
            let nickname = &modal_data.in_game_name;
            member.edit(http, |x| x.nickname(nickname)).await?;
            member.remove_role(http, needs_to_apply_role).await?;

            needs_to_apply_channel
                .send_message(http, |m| {
                    m.embed(|e| {
                        let mut author = CreateEmbedAuthor::default();
                        author.icon_url(get_avatar_url(&member.user));

                        e.title("Guild Application Request")
                            .color(DARK_PURPLE)
                            .description(
                                "Please DONT delete this after promoting or rejecting an applicant",
                            )
                            .set_author(author)
                            .field("Discord Username", &member.user.name, true)
                            .field(
                                "Display Name",
                                format!("{}", member.user.id.get_interactive(),),
                                true,
                            )
                            .field("In-Game Name", nickname, false)
                    })
                })
                .await?;

            ctx.say("Guild Application was sent!").await
        }
        None => ctx.say("No information was sent...").await,
    };

    res?;
    Ok(())
}

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

    if user.bot {
        ctx.say("Can not execute action on a bot...").await?;
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
            // TODO: Extend `Member` to have a method that wipes all roles, this is better than the .clone() hack
            member.remove_roles(http, &member.roles.clone()).await?;

            // Give "triggered" role to user
            member
                .add_role(http, RoleId::from_str(ctx.data().triggered_role.as_str())?)
                .await?;

            // Get the current state of all TRooms
            let mut t_rooms = TRooms::load(ctx.data())?;

            let open_room = t_rooms.find_open_room(ctx.data())?;

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
    if user.bot {
        ctx.say("Can not execute action on a bot...").await?;
        return Ok(());
    }
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
            // TODO: Extend `Member` to have a method that wipes all roles, this is better than the .clone() hack
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
