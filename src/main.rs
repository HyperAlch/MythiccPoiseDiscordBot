use anyhow::Context as _;
use poise::futures_util::StreamExt;
use poise::serenity_prelude::{self as serenity, GuildId};
use poise::serenity_prelude::{CacheHttp, MessageId};

use shuttle_poise::ShuttlePoise;
use shuttle_secrets::SecretStore;

use serenity::GatewayIntents;

use crate::constants::MASTER_ADMIN;

use crate::state::admins::Admins;
use crate::state::games::Games;
use crate::state::init_all_state;
use shuttle_persist::PersistInstance;
use state::Data;

use state::SnowflakeStorage;

mod constants;
mod data_structs;
mod extensions;
mod state;

// User data, which is stored and accessible in all command invocations

pub type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Check if bot is online
#[poise::command(slash_command, ephemeral)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("I'm online!").await?;
    Ok(())
}

/// Add target user to admin list
#[poise::command(slash_command, ephemeral, required_permissions = "ADMINISTRATOR")]
async fn add_admin(
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
async fn remove_admin(
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
async fn list_admins(ctx: Context<'_>) -> Result<(), Error> {
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
async fn add_game(
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
async fn remove_game(
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
async fn list_games(ctx: Context<'_>) -> Result<(), Error> {
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
#[poise::command(slash_command, ephemeral)]
async fn prune(
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

#[shuttle_runtime::main]
async fn poise(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
    #[shuttle_persist::Persist] persist: PersistInstance,
) -> ShuttlePoise<Data, Error> {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILD_BANS
        | GatewayIntents::GUILD_PRESENCES
        | GatewayIntents::GUILD_MEMBERS;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            // [IMPORTANT]
            //  The first command must always be ping() as the command listed on index 0
            //  will always be set as the one and only global command
            commands: vec![
                ping(),
                add_admin(),
                list_admins(),
                remove_admin(),
                add_game(),
                list_games(),
                remove_game(),
                prune(),
            ],
            ..Default::default()
        })
        .token(discord_token)
        .intents(intents)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                let commands = &framework.options().commands;
                poise::builtins::register_globally(ctx, &commands[..1]).await?;
                poise::builtins::register_in_guild(
                    ctx,
                    &commands[1..],
                    GuildId(888144293989085224),
                )
                .await?;

                let data = Data { bot_state: persist };
                init_all_state(&data)?;

                Ok(data)
            })
        })
        .build()
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(framework.into())
}
