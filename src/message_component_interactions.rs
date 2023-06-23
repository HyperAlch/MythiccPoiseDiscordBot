use crate::data_enums::CustomId;
use crate::state::Data;
use async_trait::async_trait;
use poise::serenity_prelude::MessageComponentInteraction;
use poise::serenity_prelude::{self as serenity};
use std::{format, vec};

mod pick_games_menu;

pub async fn handle(
    ctx: &serenity::Context,
    message_component_interaction: &MessageComponentInteraction,
    data: &Data,
) -> Result<(), crate::Error> {
    let custom_id = CustomId::new(&message_component_interaction.data.custom_id);

    // List all interaction structs here
    let all_interactions = AllInteractions(vec![Box::new(
        pick_games_menu::PickGamesMenu::new(&custom_id).unwrap_or_default(),
    )]);

    for interaction in all_interactions.0 {
        let result = interaction
            .execute(ctx, message_component_interaction, data)
            .await?;
        if result {
            return Ok(());
        }
    }

    invalid_interaction(ctx, message_component_interaction, &custom_id).await?;

    Ok(())
}

async fn invalid_interaction(
    ctx: &serenity::Context,
    message_component_interaction: &MessageComponentInteraction,
    custom_id: &CustomId,
) -> Result<(), crate::Error> {
    message_component_interaction
        .create_interaction_response(&ctx.http, |r| {
            r.interaction_response_data(|f| {
                f.ephemeral(true)
                    .content(format!("Invalid message component id: {}", custom_id))
            })
        })
        .await?;
    Ok(())
}

#[async_trait]
pub trait MsgComponentInteraction {
    fn new(custom_id: &CustomId) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::valid_custom_ids().contains(&custom_id) {
            Some(Self::inner_new(custom_id))
        } else {
            None
        }
    }
    fn inner_new(custom_id: &CustomId) -> Self
    where
        Self: Sized;

    fn valid_custom_ids() -> Vec<CustomId>
    where
        Self: Sized;
    fn custom_id(&self) -> &CustomId;

    async fn execute(
        &self,
        ctx: &serenity::Context,
        message_component_interaction: &MessageComponentInteraction,
        data: &Data,
    ) -> Result<bool, crate::Error> {
        let result = self
            .inner_execute(ctx, message_component_interaction, data)
            .await?;

        Ok(result)
    }

    async fn inner_execute(
        &self,
        ctx: &serenity::Context,
        message_component_interaction: &MessageComponentInteraction,
        data: &Data,
    ) -> Result<bool, crate::Error>;
}

struct AllInteractions(Vec<Box<dyn MsgComponentInteraction + Sync + Send>>);
