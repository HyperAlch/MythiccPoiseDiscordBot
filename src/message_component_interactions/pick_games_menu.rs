use super::MsgComponentInteraction;
use crate::data_enums::CustomId;
use crate::state::games::Games;
use crate::state::{Data, SnowflakeStorage, SnowflakesToRoles};
use async_trait::async_trait;
use poise::serenity_prelude::{self as serenity};
use poise::serenity_prelude::{InteractionResponseType, MessageComponentInteraction};
use std::vec;

#[derive(Default)]
pub struct PickGamesMenu(CustomId);

#[async_trait]
impl MsgComponentInteraction for PickGamesMenu {
    // List all valid custom component ids here
    fn valid_custom_ids() -> Vec<CustomId> {
        vec![CustomId::PickGamesAdd, CustomId::PickGamesRemove]
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
            CustomId::PickGamesAdd => {
                Self::pick_games_button_pressed(ctx, message_component_interaction, data).await?;
                return Ok(true);
            }
            _ => (),
        }

        Ok(false)
    }
}

// All component interaction methods defined here
impl PickGamesMenu {
    pub async fn pick_games_button_pressed(
        ctx: &serenity::Context,
        message_component_interaction: &MessageComponentInteraction,
        data: &Data,
    ) -> Result<(), crate::Error> {
        let cache = &ctx.cache;
        let games = Games::load(data)?;
        let games = games.to_roles(cache);

        message_component_interaction
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message
                            .content("Please select the games you're interested in")
                            .ephemeral(true)
                            .components(|components| {
                                components.create_action_row(|row| {
                                    // An action row can only contain one select menu!
                                    row.create_select_menu(|menu| {
                                        menu.custom_id("pick-games-add-select");
                                        menu.placeholder("No games selected");
                                        menu.max_values(
                                            u64::try_from(games.len())
                                                .expect("usize to u 64 conversion failed"),
                                        );
                                        menu.options(move |menu_options| {
                                            for game in games {
                                                menu_options.create_option(|option| {
                                                    option.label(game.name).value(game.id)
                                                });
                                            }
                                            menu_options
                                        })
                                    })
                                })
                            })
                    })
            })
            .await?;
        Ok(())
    }
}
