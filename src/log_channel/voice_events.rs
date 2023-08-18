use std::str::FromStr;

use chrono::Utc;
use poise::serenity_prelude::{
    self as serenity,
    colours::branding::{GREEN, RED, YELLOW},
    ChannelId, CreateEmbedAuthor, CreateEmbedFooter, UserId, VoiceState,
};

use crate::{extensions::InteractiveSnowflakeExt, state::Data, utils::discord_cdn::get_avatar_url};

pub enum VoiceEvent {
    UserJoinedChannel(ChannelId, UserId),
    UserLeftChannel(ChannelId, UserId),
    UserMovedChannel(ChannelId, ChannelId, UserId),
    Unknown,
}

// Log channel functionality
impl VoiceEvent {
    async fn execute_user_joined_vc_log(
        channel_id: &ChannelId,
        ctx: &serenity::Context,
        data: &Data,
        user_id: UserId,
    ) -> Result<(), crate::Error> {
        let target_channel = Self::get_minor_event_channel(data);
        let user = &user_id.to_user(&ctx.http).await?;

        target_channel
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    let mut author = CreateEmbedAuthor::default();
                    author.icon_url(get_avatar_url(user));
                    author.name(&user.name);

                    let mut footer = CreateEmbedFooter::default();
                    footer.text(format!("User ID: {}", user_id));

                    e.title("Joined Voice Chat")
                        .color(GREEN)
                        .description(format!("Channel: {}", channel_id.get_interactive()))
                        .timestamp(Utc::now())
                        .set_author(author)
                        .field(
                            "Display Name",
                            format!("{}", user_id.get_interactive(),),
                            false,
                        )
                        .set_footer(footer)
                })
            })
            .await?;
        Ok(())
    }

    async fn execute_user_left_vc_log(
        channel_id: &ChannelId,
        ctx: &serenity::Context,
        data: &Data,
        user_id: UserId,
    ) -> Result<(), crate::Error> {
        let target_channel = Self::get_minor_event_channel(data);
        let user = &user_id.to_user(&ctx.http).await?;

        target_channel
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    let mut author = CreateEmbedAuthor::default();
                    author.icon_url(get_avatar_url(user));
                    author.name(&user.name);

                    let mut footer = CreateEmbedFooter::default();
                    footer.text(format!("User ID: {}", user_id));

                    e.title("Left Voice Chat")
                        .color(RED)
                        .description(format!("Channel: {}", channel_id.get_interactive()))
                        .timestamp(Utc::now())
                        .set_author(author)
                        .field(
                            "Display Name",
                            format!("{}", user_id.get_interactive(),),
                            false,
                        )
                        .set_footer(footer)
                })
            })
            .await?;
        Ok(())
    }

    async fn execute_user_moved_vc_log(
        old_channel_id: &ChannelId,
        new_channel_id: &ChannelId,
        ctx: &serenity::Context,
        data: &Data,
        user_id: UserId,
    ) -> Result<(), crate::Error> {
        let target_channel = Self::get_minor_event_channel(data);
        let user = &user_id.to_user(&ctx.http).await?;

        target_channel
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    let mut author = CreateEmbedAuthor::default();
                    author.icon_url(get_avatar_url(user));
                    author.name(&user.name);

                    let mut footer = CreateEmbedFooter::default();
                    footer.text(format!("User ID: {}", user_id));

                    e.title("Moved Voice Chat")
                        .color(YELLOW)
                        .field("Left", old_channel_id.get_interactive(), true)
                        .field("Joined", new_channel_id.get_interactive(), true)
                        .timestamp(Utc::now())
                        .set_author(author)
                        .field(
                            "Display Name",
                            format!("{}", user_id.get_interactive(),),
                            false,
                        )
                        .set_footer(footer)
                })
            })
            .await?;
        Ok(())
    }
}

// Core functionality
impl VoiceEvent {
    pub fn new(old: &Option<VoiceState>, new: &VoiceState) -> Self {
        if Self::is_user_joined(old, new) {
            Self::UserJoinedChannel(new.channel_id.unwrap(), new.user_id)
        } else if Self::is_user_left(old, new) {
            Self::UserLeftChannel(old.as_ref().unwrap().channel_id.unwrap(), new.user_id)
        } else if Self::is_user_moved(old, new) {
            Self::UserMovedChannel(
                old.as_ref().unwrap().channel_id.unwrap(),
                new.channel_id.unwrap(),
                new.user_id,
            )
        } else {
            Self::Unknown
        }
    }

    pub async fn post_to_log_channel(
        &self,
        ctx: &serenity::Context,
        data: &Data,
    ) -> Result<(), crate::Error> {
        match self {
            Self::UserJoinedChannel(channel_id, user_id) => {
                Self::execute_user_joined_vc_log(channel_id, ctx, data, *user_id).await?;
            }
            Self::UserLeftChannel(channel_id, user_id) => {
                Self::execute_user_left_vc_log(channel_id, ctx, data, *user_id).await?;
            }
            Self::UserMovedChannel(old_channel_id, new_channel_id, user_id) => {
                Self::execute_user_moved_vc_log(
                    old_channel_id,
                    new_channel_id,
                    ctx,
                    data,
                    *user_id,
                )
                .await?;
            }
            Self::Unknown => (),
        }

        Ok(())
    }

    fn is_user_joined(old: &Option<VoiceState>, new: &VoiceState) -> bool {
        if old.is_none() && new.channel_id.is_some() {
            true
        } else {
            false
        }
    }

    fn is_user_left(old: &Option<VoiceState>, new: &VoiceState) -> bool {
        if old.is_some() && new.channel_id.is_none() {
            if old.as_ref().unwrap().channel_id.is_some() {
                return true;
            }
        }
        false
    }

    fn is_user_moved(old: &Option<VoiceState>, new: &VoiceState) -> bool {
        if old.is_some() && new.channel_id.is_some() {
            if old.as_ref().unwrap().channel_id.is_some() {
                if old.as_ref().unwrap().channel_id.unwrap() != new.channel_id.unwrap() {
                    return true;
                }
            }
        }
        false
    }

    fn get_minor_event_channel(data: &Data) -> ChannelId {
        ChannelId::from_str(data.minor_events_channel.as_str())
            .expect("MINOR_EVENTS_CHANNEL secret could not be parsed into a u64")
    }
}
