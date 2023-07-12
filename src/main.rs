use crate::state::init_all_state;
use anyhow::Context as _;
use log_channel::{
    user_events::{UserChangeType, UserEvent},
    voice_events::VoiceEvent,
};
use poise::serenity_prelude::{self as serenity, GuildId, Interaction, RoleId};
use serenity::GatewayIntents;
use shuttle_persist::PersistInstance;
use shuttle_poise::ShuttlePoise;
use shuttle_secrets::SecretStore;
use state::Data;
use std::str::FromStr;

mod checks;
mod constants;
mod data_enums;
mod data_structs;
mod extensions;
mod log_channel;
mod message_component_interactions;
mod slash_commands;
mod state;
mod utils;

// User data, which is stored and accessible in all command invocations

pub type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn event_handler(
    ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _ctx_poise: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        poise::Event::GuildMemberAddition { new_member } => {
            let mut new_member = new_member.clone();

            new_member
                .add_role(&ctx.http, RoleId::from_str(data.follower_role.as_str())?)
                .await?;

            let event = UserEvent::UserJoin(new_member.user.id);
            event.post_to_log_channel(ctx, data).await?;
        }
        poise::Event::GuildMemberRemoval {
            guild_id: _,
            user,
            member_data_if_available: _,
        } => {
            let event = UserEvent::UserLeave(user.id);
            event.post_to_log_channel(ctx, data).await?;
        }
        poise::Event::GuildBanAddition {
            guild_id: _,
            banned_user,
        } => {
            let event = UserEvent::UserBan(banned_user.id);
            event.post_to_log_channel(ctx, data).await?;
        }
        poise::Event::GuildBanRemoval {
            guild_id: _,
            unbanned_user,
        } => {
            let event = UserEvent::UserUnban(unbanned_user.id);
            event.post_to_log_channel(ctx, data).await?;
        }
        poise::Event::VoiceStateUpdate { old, new } => {
            let voice_event = VoiceEvent::new(old, new);
            voice_event.post_to_log_channel(ctx, data).await?;
        }
        poise::Event::GuildMemberUpdate {
            old_if_available,
            new,
        } => {
            if let Some(old) = old_if_available {
                let event = UserEvent::UserChange(new.user.id, UserChangeType::new(old, new));
                event.post_to_log_channel(ctx, data).await?;
            }
        }
        poise::Event::InteractionCreate { interaction } => match interaction {
            Interaction::MessageComponent(message_component_interaction) => {
                message_component_interactions::handle(ctx, message_component_interaction, data)
                    .await?
            }
            _ => (),
        },
        _ => (),
    }

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

    let minor_events_channel = secret_store
        .get("MINOR_EVENTS_CHANNEL")
        .context("'MINOR_EVENTS_CHANNEL' was not found")?;

    let major_events_channel = secret_store
        .get("MAJOR_EVENTS_CHANNEL")
        .context("'MAJOR_EVENTS_CHANNEL' was not found")?;

    let follower_role = secret_store
        .get("FOLLOWER_ROLE")
        .context("'FOLLOWER_ROLE' was not found")?;

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
                slash_commands::ping(),
                slash_commands::add_admin(),
                slash_commands::list_admins(),
                slash_commands::remove_admin(),
                slash_commands::add_game(),
                slash_commands::list_games(),
                slash_commands::remove_game(),
                slash_commands::prune(),
                slash_commands::pick_games_menu(),
            ],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
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

                let data = Data {
                    bot_state: persist,
                    minor_events_channel,
                    major_events_channel,
                    follower_role,
                };
                init_all_state(&data)?;

                Ok(data)
            })
        })
        .build()
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(framework.into())
}
