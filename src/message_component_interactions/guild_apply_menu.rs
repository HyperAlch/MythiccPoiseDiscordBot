use super::MsgComponentInteraction;
use crate::data_enums::CustomId;
use crate::state::Data;
use async_trait::async_trait;
use poise::serenity_prelude::MessageComponentInteraction;
use poise::serenity_prelude::{self as serenity};
use poise::Modal;
use std::vec;

#[derive(Default)]
pub struct GuildApplyMenu(CustomId);

#[derive(Debug, Modal)]
#[name = "Apply to guild"] // Struct name by default
struct GuildApplyUserModal {
    #[name = "You EXACT in-game name"] // Field name by default
    #[placeholder = "leeroy jenkins"] // No placeholder by default
    #[min_length = 1] // No length restriction by default (so, 1-4000 chars)
    #[max_length = 32]
    first_input: String,
}

#[async_trait]
impl MsgComponentInteraction for GuildApplyMenu {
    // List all valid custom component ids here
    fn valid_custom_ids() -> Vec<CustomId> {
        vec![CustomId::GuildApply]
    }

    fn custom_id(&self) -> &CustomId {
        &self.0
    }

    fn inner_new(custom_id: &CustomId) -> Self
    where
        Self: Sized,
    {
        Self(*custom_id)
    }

    // Match all valid custom component ids with their methods
    async fn inner_execute(
        &self,
        ctx: &serenity::Context,
        message_component_interaction: &MessageComponentInteraction,
        data: &Data,
    ) -> Result<bool, crate::Error> {
        match self.0 {
            CustomId::GuildApply => {
                Self::guild_apply_modal_popup(ctx, message_component_interaction, data).await?;
                return Ok(true);
            }
            _ => (),
        }

        Ok(false)
    }
}

impl GuildApplyMenu {
    pub async fn guild_apply_modal_popup(
        _ctx: &serenity::Context,
        _message_component_interaction: &MessageComponentInteraction,
        _data: &Data,
    ) -> Result<(), crate::Error> {
        // This is currently not possible, this project route is abandoned
        Ok(())
    }
}
