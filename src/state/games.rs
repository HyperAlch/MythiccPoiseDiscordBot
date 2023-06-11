use crate::state::BotStateInitialization;
use crate::Data;

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
}
