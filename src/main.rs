use anyhow::Context as _;
use poise::serenity_prelude::{self as serenity, GuildId};
use serde::{Deserialize, Serialize};
use shuttle_poise::ShuttlePoise;
use shuttle_secrets::SecretStore;

use serenity::GatewayIntents;

use shuttle_persist::PersistInstance;

// User data, which is stored and accessible in all command invocations
struct Data {
    bot_state: PersistInstance,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Serialize, Deserialize, Clone)]
struct Admins(Vec<u64>);

impl std::fmt::Display for Admins {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let raw_admins = &self.0;

        let mut admins = String::new();
        for admin in raw_admins {
            admins.push_str(&format!("{}\n", admin))
        }
        write!(f, "{}", admins)
    }
}

/// Check if bot is online
#[poise::command(slash_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("I'm online!").await?;
    Ok(())
}

/// Add target user to admin list
#[poise::command(slash_command)]
async fn add_admin(ctx: Context<'_>) -> Result<(), Error> {
    let state = ctx.data();
    let _state = state.bot_state.save::<Admins>(
        "admins",
        Admins(vec![224597366324461568, 213501447185104896]),
    )?;
    ctx.say("Admin added!").await?;
    Ok(())
}

/// Display admin list
#[poise::command(slash_command)]
async fn list_admins(ctx: Context<'_>) -> Result<(), Error> {
    let state = ctx.data();
    let admins = state.bot_state.load::<Admins>("admins")?;

    ctx.say(admins.to_string()).await?;
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
            commands: vec![ping(), add_admin(), list_admins()],
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

                Ok(Data { bot_state: persist })
            })
        })
        .build()
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(framework.into())
}
