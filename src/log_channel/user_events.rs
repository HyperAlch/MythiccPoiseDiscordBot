use std::str::FromStr;

use chrono::Utc;
use poise::serenity_prelude::{
    self as serenity,
    colours::branding::{GREEN, RED, YELLOW},
    ChannelId, CreateEmbedAuthor, CreateEmbedFooter, Member, Role, RoleId, UserId,
};

use crate::{
    extensions::InteractiveSnowflakeExt,
    state::Data,
    utils::{discord_cdn::get_avatar_url, time::date_diff},
};

pub enum UserEvent {
    UserJoin(UserId),
    UserLeave(UserId, Vec<Role>),
    UserBan(UserId),
    UserUnban(UserId),
    UserChange(UserId, UserChangeType),
}

pub enum UserChangeType {
    RolesChanged(RoleState),
    Unknown,
}

// Core functionality
impl UserChangeType {
    pub fn new(old_state: &Member, new_state: &Member) -> Self {
        if old_state.roles != new_state.roles {
            UserChangeType::RolesChanged(Self::get_role_changes(old_state, new_state))
        } else {
            UserChangeType::Unknown
        }
    }

    fn get_role_changes(old_state: &Member, new_state: &Member) -> RoleState {
        let new_roles: Vec<RoleId> = new_state
            .roles
            .clone()
            .into_iter()
            .filter(|r| !old_state.roles.contains(r))
            .collect();

        let old_roles: Vec<RoleId> = old_state
            .roles
            .clone()
            .into_iter()
            .filter(|r| !new_state.roles.contains(r))
            .collect();

        RoleState::new(new_roles, old_roles)
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

    pub fn added(&self) -> &Vec<RoleId> {
        &self.added
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
        all_roles: Vec<Role>,
    ) -> Result<(), crate::Error> {
        let target_channel = Self::get_major_event_channel(data);
        let user = user_id.to_user(&ctx.http).await?;

        let all_roles: Vec<String> = all_roles
            .iter()
            .map(|x| x.id)
            .map(|x| x.get_interactive())
            .collect();

        let all_roles = all_roles.join(" ");

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
                        .field("Roles", all_roles, false)
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

    async fn execute_user_changed_log(
        ctx: &serenity::Context,
        data: &Data,
        user_id: UserId,
        role_state: &RoleState,
    ) -> Result<(), crate::Error> {
        let target_channel = Self::get_major_event_channel(data);
        let user = user_id.to_user(&ctx.http).await?;

        let new_roles: String = role_state
            .added
            .iter()
            .map(|x| x.get_interactive())
            .collect();
        let new_roles = new_roles.replace("><", "> <");

        let old_roles: String = role_state
            .removed
            .iter()
            .map(|x| x.get_interactive())
            .collect();
        let old_roles = old_roles.replace("><", "> <");

        target_channel
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    let mut author = CreateEmbedAuthor::default();
                    author.icon_url(get_avatar_url(&user));
                    author.name(user.name.clone());

                    let mut footer = CreateEmbedFooter::default();
                    footer.text(format!("User ID: {}", user.id));

                    let embed = e
                        .title("Roles Updated")
                        .color(YELLOW)
                        .description("🔄 🔄 🔄");

                    if new_roles.len() > 0 {
                        embed.field("New Roles: ", new_roles, false);
                    }

                    if old_roles.len() > 0 {
                        embed.field("Removed Roles: ", old_roles, false);
                    }

                    embed
                        .timestamp(Utc::now())
                        .set_author(author)
                        .field("Username", format!("{}", user.id.get_interactive()), false)
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
            Self::UserLeave(user_id, all_roles) => {
                Self::execute_user_left_guild_log(ctx, data, *user_id, all_roles.clone()).await?;
            }
            Self::UserBan(user_id) => {
                Self::execute_user_ban_guild_log(ctx, data, *user_id).await?;
            }
            Self::UserUnban(user_id) => {
                Self::execute_user_unban_guild_log(ctx, data, *user_id).await?;
            }
            Self::UserChange(user_id, user_change_type) => match user_change_type {
                UserChangeType::RolesChanged(role_state) => {
                    Self::execute_user_changed_log(ctx, data, *user_id, role_state).await?;
                }
                UserChangeType::Unknown => (),
            },
        }

        Ok(())
    }

    fn get_major_event_channel(data: &Data) -> ChannelId {
        ChannelId::from_str(data.major_events_channel.as_str())
            .expect("MINOR_EVENTS_CHANNEL secret could not be parsed into a u64")
    }
}
