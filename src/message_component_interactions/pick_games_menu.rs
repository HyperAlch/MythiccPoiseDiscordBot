use super::MsgComponentInteraction;
use crate::data_enums::CustomId;
use crate::extensions::InteractiveSnowflakeExt;
use crate::state::games::Games;
use crate::state::{Data, SnowflakeStorage, SnowflakesToRoles};
use crate::utils::discord_cdn::get_avatar_url;
use async_trait::async_trait;
use chrono::Utc;
use poise::serenity_prelude::colours::branding::{RED, YELLOW};
use poise::serenity_prelude::{
    self as serenity, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter,
};
use poise::serenity_prelude::{InteractionResponseType, MessageComponentInteraction};
use poise::serenity_prelude::{Role, RoleId};
use std::vec;

#[derive(Default)]
pub struct PickGamesMenu(CustomId);

#[async_trait]
impl MsgComponentInteraction for PickGamesMenu {
    // List all valid custom component ids here
    fn valid_custom_ids() -> Vec<CustomId> {
        vec![
            CustomId::PickGamesAdd,
            CustomId::PickGamesRemove,
            CustomId::PickGamesAddExecute,
            CustomId::PickGamesRemoveExecute,
        ]
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
            CustomId::PickGamesRemove => {
                Self::remove_games_button_pressed(ctx, message_component_interaction, data).await?;
                return Ok(true);
            }
            CustomId::PickGamesAddExecute => {
                Self::pick_games_button_execute(ctx, message_component_interaction, data).await?;
                return Ok(true);
            }
            CustomId::PickGamesRemoveExecute => {
                Self::remove_games_button_execute(ctx, message_component_interaction, data).await?;
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
        let user = message_component_interaction.member.as_ref();

        if let Some(member) = user {
            let user_roles = &member.roles;
            let games: Vec<Role> = games
                .into_iter()
                .filter(|game| !user_roles.contains(&game.id))
                .collect();

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
                                            menu.custom_id(CustomId::PickGamesAddExecute.to_string());
                                            menu.placeholder("No games selected");
                                            if games.len() > 0 {
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
                                            } else {
                                                menu.max_values(1);
                                                menu.options(move |menu_options| {
                                                    menu_options.create_option(|option| {
                                                        option.label("All available games are already assigned to you...").value("__invalid__")
                                                    });
                                                    menu_options
                                                })
                                            }
                                        })
                                    })
                                })
                        })
                })
                .await?;
        }
        Ok(())
    }

    pub async fn pick_games_button_execute(
        ctx: &serenity::Context,
        message_component_interaction: &MessageComponentInteraction,
        data: &Data,
    ) -> Result<(), crate::Error> {
        let mut user = message_component_interaction.member.clone();

        let selected_games = &message_component_interaction.data.values;
        let mut invalid_execution = false;

        if selected_games.starts_with(&["__invalid__".to_string()]) {
            invalid_execution = true;
        }

        if let Some(member) = user.as_mut() {
            let selected_games: Vec<RoleId> = selected_games
                .into_iter()
                .map(|x| {
                    RoleId(
                        x.parse::<u64>()
                            .expect("Failed to parse String into u64..."),
                    )
                })
                .filter(|game| !&member.roles.contains(&game))
                .collect();

            if selected_games.len() < 1 {
                invalid_execution = true;
            }

            member.add_roles(&ctx.http, &selected_games).await?;

            let guild_apply_roles: Vec<String> =
                selected_games.iter().map(|x| x.to_string()).collect();

            let guild_apply_roles: Vec<&String> = guild_apply_roles
                .iter()
                .filter(|x| data.guild_apply_roles.contains(*x))
                .collect();

            let display_roles: String =
                selected_games.iter().map(|x| x.get_interactive()).collect();
            let display_roles = display_roles.replace("><", "> <");

            message_component_interaction
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|m| {
                            m.ephemeral(true);
                            if !invalid_execution {
                                let mut e1 = CreateEmbed::default();

                                let mut author = CreateEmbedAuthor::default();
                                author.icon_url(get_avatar_url(&member.user));
                                author.name(&member.user.name);

                                let mut footer = CreateEmbedFooter::default();
                                footer.text(format!("User ID: {}", member.user.id));

                                e1.title("Roles Updated")
                                    .color(YELLOW)
                                    .description("🔄 🔄 🔄")
                                    .field("New Roles: ", display_roles, true)
                                    .timestamp(Utc::now())
                                    .set_author(author)
                                    .field(
                                        "Display Name",
                                        format!("{}", member.user.id.get_interactive(),),
                                        false,
                                    )
                                    .set_footer(footer);

                                let mut e2 = CreateEmbed::default();
                                e2.title("Guild Application Required!")
                                    .description("# Guild Application Required!\n***Step 1: Move to any room you can type in***\nStep 2: `Right Click` yourself IN THE MYTHICC DISCORD, select `Apps`, and then `Guild Apply`")
                                    .color(RED);

                                if guild_apply_roles.len() > 0 {
                                    m.add_embed(e2)
                                } else {
                                    m.add_embed(e1)
                                }
                            } else {
                                m.content("Invalid Operation...")
                            }
                        })
                })
                .await?;
        }

        Ok(())
    }

    pub async fn remove_games_button_pressed(
        ctx: &serenity::Context,
        message_component_interaction: &MessageComponentInteraction,
        data: &Data,
    ) -> Result<(), crate::Error> {
        let cache = &ctx.cache;
        let games = Games::load(data)?;
        let games = games.to_roles(cache);
        let user = message_component_interaction.member.as_ref();

        if let Some(member) = user {
            let user_roles = &member.roles;
            let games: Vec<Role> = games
                .into_iter()
                .filter(|game| user_roles.contains(&game.id))
                .collect();

            message_component_interaction
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message
                                .content("Please select the game roles you would like to remove")
                                .ephemeral(true)
                                .components(|components| {
                                    components.create_action_row(|row| {
                                        // An action row can only contain one select menu!
                                        row.create_select_menu(|menu| {
                                            menu.custom_id(CustomId::PickGamesRemoveExecute.to_string());
                                            menu.placeholder("No games selected");
                                            if games.len() > 0 {
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
                                            } else {
                                                menu.max_values(1);
                                                menu.options(move |menu_options| {
                                                    menu_options.create_option(|option| {
                                                        option.label("None of the available games are assigned to you...").value("__invalid__")
                                                    });
                                                    menu_options
                                                })
                                            }
                                        })
                                    })
                                })
                        })
                })
                .await?;
        }

        Ok(())
    }

    pub async fn remove_games_button_execute(
        ctx: &serenity::Context,
        message_component_interaction: &MessageComponentInteraction,
        _data: &Data,
    ) -> Result<(), crate::Error> {
        let mut user = message_component_interaction.member.clone();

        let selected_games = &message_component_interaction.data.values;
        let mut invalid_execution = false;

        if selected_games.starts_with(&["__invalid__".to_string()]) {
            invalid_execution = true;
        }

        if let Some(member) = user.as_mut() {
            let selected_games: Vec<RoleId> = selected_games
                .into_iter()
                .map(|x| {
                    RoleId(
                        x.parse::<u64>()
                            .expect("Failed to parse String into u64..."),
                    )
                })
                .filter(|game| member.roles.contains(&game))
                .collect();

            if selected_games.len() < 1 {
                invalid_execution = true;
            }

            member.remove_roles(&ctx.http, &selected_games).await?;

            let display_roles: String =
                selected_games.iter().map(|x| x.get_interactive()).collect();
            let display_roles = display_roles.replace("><", "> <");

            message_component_interaction
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|m| {
                            m.ephemeral(true);
                            if !invalid_execution {
                                m.embed(|e| {
                                    let mut author = CreateEmbedAuthor::default();
                                    author.icon_url(get_avatar_url(&member.user));
                                    author.name(&member.user.name);

                                    let mut footer = CreateEmbedFooter::default();
                                    footer.text(format!("User ID: {}", member.user.id));

                                    e.title("Roles Updated")
                                        .color(YELLOW)
                                        .description("🔄 🔄 🔄")
                                        .field("Removed Roles: ", display_roles, true)
                                        .timestamp(Utc::now())
                                        .set_author(author)
                                        .field(
                                            "Display Name",
                                            format!("{}", member.user.id.get_interactive(),),
                                            false,
                                        )
                                        .set_footer(footer)
                                })
                            } else {
                                m.content("Invalid Operation...")
                            }
                        })
                })
                .await?;
        }

        Ok(())
    }
}
