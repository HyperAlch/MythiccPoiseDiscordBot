use crate::checks::is_on_admin_list;
use crate::constants::MASTER_ADMIN;
use crate::data_enums::CustomId;
use crate::state::admins::Admins;
use crate::state::games::Games;
use crate::state::SnowflakeStorage;
use crate::Context;
use crate::Error;
use poise::futures_util::StreamExt;
use poise::serenity_prelude::ButtonStyle;
use poise::serenity_prelude::{self as serenity};
use poise::serenity_prelude::{CacheHttp, MessageId};
use std::format;

/// Check if bot is online
#[poise::command(slash_command, ephemeral)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("I'm online!").await?;
    Ok(())
}

/// Add target user to admin list
#[poise::command(slash_command, ephemeral, required_permissions = "ADMINISTRATOR")]
pub async fn add_admin(
    ctx: Context<'_>,
    #[description = "Selected user"] user: serenity::User,
) -> Result<(), Error> {
    let user_id: u64 = user.id.into();
    let data = ctx.data();

    let mut admins = Admins::load(data)?;
    let successful = admins.add(data, user_id)?;

    if successful {
        ctx.say("User was added to the Admin list!").await?;
    } else {
        ctx.say("Admin is already registered...").await?;
    }

    Ok(())
}

/// Remove target user from admin list
#[poise::command(slash_command, ephemeral, required_permissions = "ADMINISTRATOR")]
pub async fn remove_admin(
    ctx: Context<'_>,
    #[description = "Selected user"] user: serenity::User,
) -> Result<(), Error> {
    let user_id: u64 = user.id.into();

    if user_id == MASTER_ADMIN {
        ctx.say("Can not remove master admin!").await?;
        return Ok(());
    }

    let data = ctx.data();

    let mut admins = Admins::load(data)?;
    let successful = admins.remove(data, user_id)?;

    if successful {
        ctx.say("User was remove from the Admin list!").await?;
    } else {
        ctx.say("User could not be found on the Admin list...")
            .await?;
    }

    Ok(())
}

/// Display admin list
#[poise::command(slash_command, ephemeral, required_permissions = "ADMINISTRATOR")]
pub async fn list_admins(ctx: Context<'_>) -> Result<(), Error> {
    let state = ctx.data();

    let admins = Admins::load(state)?.to_string();

    if admins.is_empty() {
        ctx.say("No admins found").await?;
    } else {
        ctx.say(admins).await?;
    }

    Ok(())
}

/// Add a game role to the list of games
#[poise::command(slash_command, ephemeral, required_permissions = "ADMINISTRATOR")]
pub async fn add_game(
    ctx: Context<'_>,
    #[description = "Selected user"] role: serenity::Role,
) -> Result<(), Error> {
    let role_id: u64 = role.id.into();
    let data = ctx.data();

    let mut games = Games::load(data)?;
    let successful = games.add(data, role_id)?;

    if successful {
        ctx.say("Game was added to the game list!").await?;
    } else {
        ctx.say("Game is already registered...").await?;
    }

    Ok(())
}

/// Remove a game role from the list of games
#[poise::command(slash_command, ephemeral, required_permissions = "ADMINISTRATOR")]
pub async fn remove_game(
    ctx: Context<'_>,
    #[description = "Selected user"] role: serenity::Role,
) -> Result<(), Error> {
    let role_id: u64 = role.id.into();
    let data = ctx.data();

    let mut games = Games::load(data)?;

    let successful = games.remove(data, role_id)?;

    if successful {
        ctx.say("Game was remove from the games list!").await?;
    } else {
        ctx.say("Game could not be found on the games list...")
            .await?;
    }

    Ok(())
}

/// Display game list
#[poise::command(slash_command, ephemeral, required_permissions = "ADMINISTRATOR")]
pub async fn list_games(ctx: Context<'_>) -> Result<(), Error> {
    let state = ctx.data();

    let games = Games::load(state)?.to_string();

    if games.is_empty() {
        ctx.say("No games found").await?;
    } else {
        ctx.say(games).await?;
    }

    Ok(())
}

/// "Delete 'x' amount of messages"
#[poise::command(slash_command, ephemeral, check = "is_on_admin_list")]
pub async fn prune(
    ctx: Context<'_>,
    #[description = "Selected user"] amount: usize,
) -> Result<(), Error> {
    let http = ctx.http();

    let target_channel = ctx.channel_id();
    let mut messages = target_channel.messages_iter(http).boxed();
    let mut message_ids: Vec<MessageId> = vec![];

    while let Some(message_result) = messages.next().await {
        match message_result {
            Ok(message) => {
                if message_ids.len() < amount {
                    message_ids.push(message.id)
                } else {
                    break;
                };
            }
            Err(_) => {
                ctx.say("Error loading messages into prune process!")
                    .await?;
                return Ok(());
            }
        }
    }

    target_channel.delete_messages(http, message_ids).await?;
    ctx.say(format!("{} message(s) deleted!", amount)).await?;

    Ok(())
}

/// Setup the 'Pick Your Games' menu
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn pick_games_menu(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(|b| {
        b.content("Pick Your Games").components(|c| {
            c.create_action_row(|row| {
                row.create_button(|button| {
                    button
                        .custom_id(CustomId::PickGamesAdd.to_string())
                        .label("Add")
                        .style(ButtonStyle::Success)
                });
                row.create_button(|button| {
                    button
                        .custom_id(CustomId::PickGamesRemove.to_string())
                        .label("Remove")
                        .style(ButtonStyle::Danger)
                })
            })
        })
    })
    .await?;

    Ok(())
}
