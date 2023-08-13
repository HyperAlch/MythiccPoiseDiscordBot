use crate::checks::is_on_admin_list;
use crate::constants::MASTER_ADMIN;
use crate::data_enums::CustomId;
use crate::state::admins::Admins;
use crate::state::games::Games;
use crate::state::guild_apply::GuildApply;
use crate::state::t_rooms::TRooms;
use crate::state::SnowflakeHashmapStorage;
use crate::state::SnowflakeStorage;
use crate::Context;
use crate::Error;
use poise::futures_util::StreamExt;
use poise::serenity_prelude::colours::branding::BLACK;
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
    let mut instructions = "".to_string();

    instructions.push_str(":green_circle: **Add** - Press to get a dropdown of all available game roles that you don't already have, select the ones you want.\n\n");
    instructions.push_str(":red_circle: **Remove** - Press to get a dropdown of all game roles currently assigned to you, select the ones you want to remove.\n\n");
    instructions.push_str("**[IMPORTANT]\n\nDropdowns are meant for ONE TIME USE ONLY. Please press \"Dismiss message\" when you are done. \n\nThese temporary messages DO NOT automatically update to reflect role changes, you must press the buttons AGAIN to make any changes in the future!**");

    ctx.send(|b| {
        b.content("~ Pick Your Games ~")
            .embed(|e| {
                e.title("Instructions")
                    .color(BLACK)
                    .description(instructions)
            })
            .components(|c| {
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

/// "Unlock a triggered room for future use"
#[poise::command(slash_command, ephemeral, check = "is_on_admin_list")]
pub async fn unlock_triggered_channel(ctx: Context<'_>) -> Result<(), Error> {
    let http = ctx.http();

    let target_channel = ctx.channel_id();
    let mut messages = target_channel.messages_iter(http).boxed();
    let mut message_ids: Vec<MessageId> = vec![];

    while let Some(message_result) = messages.next().await {
        match message_result {
            Ok(message) => message_ids.push(message.id),
            Err(_) => {
                ctx.say("Error loading messages into prune process!")
                    .await?;
                return Ok(());
            }
        }
    }

    // Find Room struct using target_channel, then toggle the lock
    let mut t_rooms = TRooms::load(ctx.data())?;
    let room = t_rooms.find_room(target_channel)?;

    if let Some(&mut ref mut room) = room {
        if !message_ids.is_empty() {
            target_channel.delete_messages(http, message_ids).await?;
        }
        room.toggle_open();
        let data = ctx.data();
        t_rooms.save(data)?;

        ctx.say("Room unlocked!!").await?;
    } else {
        ctx.say("This is NOT a triggered room!!").await?;
    }

    Ok(())
}

/// Add a game / channel union to the list of games that support guild applications
#[poise::command(slash_command, ephemeral, required_permissions = "ADMINISTRATOR")]
pub async fn add_guild_application(
    ctx: Context<'_>,
    #[description = "Game name"] game_name: String,
    #[description = "Log channel"] channel: serenity::Channel,
) -> Result<(), Error> {
    let channel_id: u64 = channel.id().into();
    let data = ctx.data();

    let mut guild_apply = GuildApply::load(data)?;
    let successful = guild_apply.add(data, game_name.clone(), channel_id)?;

    if successful {
        ctx.say(format!("{} was added to the apply list!", game_name))
            .await?;
    } else {
        ctx.say("Key and / or value is already registered").await?;
    }

    Ok(())
}

/// Display the list of games that support guild applications
#[poise::command(slash_command, ephemeral, required_permissions = "ADMINISTRATOR")]
pub async fn list_guild_application(ctx: Context<'_>) -> Result<(), Error> {
    let data = ctx.data();

    let guild_apply = GuildApply::load(data)?;

    if guild_apply.0.is_empty() {
        ctx.say("Guild application list empty").await?;
    } else {
        ctx.say(guild_apply.to_string()).await?;
    }

    Ok(())
}

/// Remove a game / channel union from the list of games that support guild applications
#[poise::command(slash_command, ephemeral, required_permissions = "ADMINISTRATOR")]
pub async fn remove_guild_application(
    ctx: Context<'_>,
    #[description = "Game name"] game_name: String,
) -> Result<(), Error> {
    let data = ctx.data();

    let mut guild_apply = GuildApply::load(data)?;
    let successful = guild_apply.remove(data, game_name.clone())?;

    if successful {
        ctx.say(format!("{} was removed from the apply list!", game_name))
            .await?;
    } else {
        ctx.say(format!(
            "{} could not be found in the apply list...",
            game_name
        ))
        .await?;
    }

    Ok(())
}
