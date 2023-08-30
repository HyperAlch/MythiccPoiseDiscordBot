use super::{SnowflakeStorage, SnowflakesToRoles};
use crate::Data;
use crate::{extensions::InteractiveSnowflakeExt, state::BotStateInitialization};
use poise::serenity_prelude::RoleId;
use serde::{Deserialize, Serialize};

const KEY: &str = "games";

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Games(pub Vec<u64>);

impl std::fmt::Display for Games {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let raw_games = &self.0;

        let mut games = String::new();
        for game in raw_games {
            let game = RoleId(*game);
            let game = game.get_interactive();
            games.push_str(&format!("{}\n", game));
        }
        write!(f, "{}", games)
    }
}

impl BotStateInitialization for Games {
    fn get_key(&self) -> String {
        KEY.to_string()
    }
}

impl SnowflakeStorage for Games {
    fn load(data: &Data) -> Result<Self, anyhow::Error>
    where
        for<'de> Self: Deserialize<'de>,
    {
        let data = data.bot_state.load::<Self>(KEY);
        match data {
            Ok(data) => Ok(data),
            Err(e) => Err(anyhow::anyhow!("{}", e.to_string())),
        }
    }

    fn snowflake_found(&self, id: &u64) -> bool {
        self.0.contains(id)
    }

    fn push_snowflake(&mut self, id: u64) {
        self.0.push(id);
    }

    fn snowflakes(&self) -> std::slice::Iter<'_, u64> {
        self.0.iter()
    }

    fn remove_snowflake(&mut self, index: usize) {
        self.0.remove(index);
    }
}

impl SnowflakesToRoles for Games {}
