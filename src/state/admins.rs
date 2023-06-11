use crate::Data;
use crate::{constants::MASTER_ADMIN, state::BotStateInitialization};

use crate::extensions::InteractiveUsernameExt;
use poise::serenity_prelude::UserId;
use serde::{Deserialize, Serialize};

const KEY: &str = "admins";

#[derive(Serialize, Deserialize, Clone)]
pub struct Admins(pub Vec<u64>);

impl BotStateInitialization for Admins {
    fn get_key(&self) -> String {
        KEY.to_string()
    }
}

impl std::fmt::Display for Admins {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let raw_admins = &self.0;

        let mut admins = String::new();
        for admin in raw_admins {
            let admin = UserId(*admin);
            let admin = admin.get_interactive_username();
            admins.push_str(&format!("{}\n", admin));
        }
        write!(f, "{}", admins)
    }
}

impl Default for Admins {
    fn default() -> Self {
        Self(vec![MASTER_ADMIN])
    }
}

impl Admins {
    pub fn load(data: &Data) -> Result<Self, crate::Error> {
        let admins = data.bot_state.load::<Admins>(KEY)?;
        Ok(admins)
    }

    pub fn add(&mut self, data: &Data, user_id: u64) -> Result<bool, crate::Error> {
        if !self.0.contains(&user_id) {
            self.0.push(user_id);

            data.bot_state.save::<Admins>(KEY, self.clone())?;
            return Ok(true);
        }

        Ok(false)
    }

    pub fn remove(&mut self, data: &Data, user_id: u64) -> Result<bool, crate::Error> {
        let index = self.0.iter().position(|&i| i == user_id);

        match index {
            Some(index) => {
                self.0.remove(index);
                data.bot_state.save::<Admins>(KEY, self.clone())?;
                Ok(true)
            }
            None => Ok(false),
        }
    }
}
