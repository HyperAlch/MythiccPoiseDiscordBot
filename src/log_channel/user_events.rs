use std::{str::FromStr, vec};

use chrono::Utc;
use poise::serenity_prelude::{
    self as serenity,
    colours::branding::{GREEN, RED},
    ChannelId, CreateEmbedAuthor, CreateEmbedFooter, Member, RoleId, UserId,
};

use crate::{
    extensions::InteractiveSnowflakeExt,
    state::Data,
    utils::{discord_cdn::get_avatar_url, time::date_diff},
};

pub enum UserEvent {
    UserJoin(UserId),
    UserLeave(UserId),
    UserBan(UserId),
    UserUnban(UserId),
    UserChange(UserChangeType),
}

pub enum UserChangeType {
    RolesChanged(RoleState),
    Unknown,
}

impl UserChangeType {
    pub fn new(old_state: &Member, new_state: &Member) -> Self {
        if old_state.roles != new_state.roles {
            // Figure out what roles where added and removed
            // Set RoleState using the result.
            UserChangeType::RolesChanged(RoleState::new(vec![], vec![]))
        } else {
            UserChangeType::Unknown
        }
    }
}

pub struct RoleState {
    added: Vec<RoleId>,
    removed: Vec<RoleId>,
}

impl RoleState {
    fn new(added: Vec<RoleId>, removed: Vec<RoleId>) -> Self {
        RoleState { added, removed }
    }
}

// Log channel functionality
impl UserEvent {
    async fn execute_user_joined_guild_log(
        ctx: &serenity::Context,
        data: &Data,
        user_id: UserId,
    ) -> Result<(), crate::Error> {
        let target_channel = Self::get_major_event_channel(data);
        let user = user_id.to_user(&ctx.http).await?;

        target_channel
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    let mut author = CreateEmbedAuthor::default();
                    author.icon_url(get_avatar_url(&user));
                    author.name(user.name.clone());

                    let mut footer = CreateEmbedFooter::default();
                    footer.text(format!("User ID: {}", user.id));

                    let account_age = date_diff(&user.created_at());

                    e.title("Member Joined")
                        .color(GREEN)
                        .description(format!("{}", user_id.get_interactive()))
                        .image(get_avatar_url(&user))
                        .timestamp(Utc::now())
                        .set_author(author)
                        .field("Account Age", account_age, true)
                        .set_footer(footer)
                })
            })
            .await?;

        Ok(())
    }

    async fn execute_user_left_guild_log(
        ctx: &serenity::Context,
        data: &Data,
        user_id: UserId,
    ) -> Result<(), crate::Error> {
        let target_channel = Self::get_major_event_channel(data);
        let user = user_id.to_user(&ctx.http).await?;

        target_channel
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    let mut author = CreateEmbedAuthor::default();
                    author.icon_url(get_avatar_url(&user));
                    author.name(user.name.clone());

                    let mut footer = CreateEmbedFooter::default();
                    footer.text(format!("User ID: {}", user.id));

                    let account_age = date_diff(&user.created_at());

                    e.title("Member Left")
                        .color(RED)
                        .description(format!("{}", user.id.get_interactive()))
                        .image("https://i.ibb.co/1qyVmzG/left-discord.png")
                        .timestamp(Utc::now())
                        .set_author(author)
                        .field("Account Age", account_age, true)
                        .set_footer(footer)
                })
            })
            .await?;

        Ok(())
    }

    async fn execute_user_ban_guild_log(
        ctx: &serenity::Context,
        data: &Data,
        user_id: UserId,
    ) -> Result<(), crate::Error> {
        let target_channel = Self::get_major_event_channel(data);
        let user = user_id.to_user(&ctx.http).await?;

        target_channel
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    let mut author = CreateEmbedAuthor::default();
                    author.icon_url(get_avatar_url(&user));
                    author.name(user.name.clone());

                    let mut footer = CreateEmbedFooter::default();
                    footer.text(format!("User ID: {}", user.id));

                    let account_age = date_diff(&user.created_at());

                    e.title("Member Banned")
                        .color(RED)
                        .description(format!("{}", user.id.get_interactive()))
                        .image("https://i.ibb.co/P4m8YSL/banned.png")
                        .timestamp(Utc::now())
                        .set_author(author)
                        .field("Account Age", account_age, true)
                        .set_footer(footer)
                })
            })
            .await?;

        Ok(())
    }

    async fn execute_user_unban_guild_log(
        ctx: &serenity::Context,
        data: &Data,
        user_id: UserId,
    ) -> Result<(), crate::Error> {
        let target_channel = Self::get_major_event_channel(data);
        let user = user_id.to_user(&ctx.http).await?;

        target_channel
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    let mut author = CreateEmbedAuthor::default();
                    author.icon_url(get_avatar_url(&user));
                    author.name(user.name.clone());

                    let mut footer = CreateEmbedFooter::default();
                    footer.text(format!("User ID: {}", user.id));

                    let account_age = date_diff(&user.created_at());

                    e.title("Member Unbanned")
                        .color(GREEN)
                        .description(format!("{}", user.id.get_interactive()))
                        .image("https://i.ibb.co/7nqVFKd/unbanned.png")
                        .timestamp(Utc::now())
                        .set_author(author)
                        .field("Account Age", account_age, true)
                        .set_footer(footer)
                })
            })
            .await?;
        Ok(())
    }
}

// Core functionality
impl UserEvent {
    pub async fn post_to_log_channel(
        &self,
        ctx: &serenity::Context,
        data: &Data,
    ) -> Result<(), crate::Error> {
        match self {
            Self::UserJoin(user_id) => {
                Self::execute_user_joined_guild_log(ctx, data, *user_id).await?;
            }
            Self::UserLeave(user_id) => {
                Self::execute_user_left_guild_log(ctx, data, *user_id).await?;
            }
            Self::UserBan(user_id) => {
                Self::execute_user_ban_guild_log(ctx, data, *user_id).await?;
            }
            Self::UserUnban(user_id) => {
                Self::execute_user_unban_guild_log(ctx, data, *user_id).await?;
            }
            _ => {}
        }

        Ok(())
    }

    fn get_major_event_channel(data: &Data) -> ChannelId {
        ChannelId::from_str(data.major_events_channel.as_str())
            .expect("MINOR_EVENTS_CHANNEL secret could not be parsed into a u64")
    }
}
