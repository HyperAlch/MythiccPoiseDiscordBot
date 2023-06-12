use crate::Data;
use crate::{extensions::InteractiveSnowflakeExt, state::BotStateInitialization};

use poise::serenity_prelude::RoleId;
use serde::{Deserialize, Serialize};

const KEY: &str = "games";

#[derive(Serialize, Deserialize, Clone)]
pub struct Games(pub Vec<u64>);

impl BotStateInitialization for Games {
    fn get_key(&self) -> String {
        KEY.to_string()
    }
}

impl Default for Games {
    fn default() -> Self {
        Self(vec![])
    }
}

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

impl Games {
    pub fn load(data: &Data) -> Result<Self, crate::Error> {
        let games = data.bot_state.load::<Games>(KEY)?;
        Ok(games)
    }

    pub fn add(&mut self, data: &Data, role_id: u64) -> Result<bool, crate::Error> {
        if !self.0.contains(&role_id) {
            self.0.push(role_id);

            data.bot_state.save::<Games>(KEY, self.clone())?;
            return Ok(true);
        }

        Ok(false)
    }

    pub fn remove(&mut self, data: &Data, role_id: u64) -> Result<bool, crate::Error> {
        let index = self.0.iter().position(|&i| i == role_id);

        match index {
            Some(index) => {
                self.0.remove(index);
                data.bot_state.save::<Games>(KEY, self.clone())?;
                Ok(true)
            }
            None => Ok(false),
        }
    }
}
