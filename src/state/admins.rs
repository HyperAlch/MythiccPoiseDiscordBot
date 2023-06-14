use super::SnowflakeStorage;
use crate::extensions::InteractiveSnowflakeExt;
use crate::Data;
use crate::{constants::MASTER_ADMIN, state::BotStateInitialization};
use poise::serenity_prelude::UserId;
use serde::{Deserialize, Serialize};

const KEY: &str = "admins";

#[derive(Serialize, Deserialize, Clone)]
pub struct Admins(pub Vec<u64>);

impl Default for Admins {
    fn default() -> Self {
        Self(vec![MASTER_ADMIN])
    }
}

impl std::fmt::Display for Admins {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let raw_admins = &self.0;

        let mut admins = String::new();
        for admin in raw_admins {
            let admin = UserId(*admin);
            let admin = admin.get_interactive();
            admins.push_str(&format!("{}\n", admin));
        }
        write!(f, "{}", admins)
    }
}

impl BotStateInitialization for Admins {
    fn get_key(&self) -> String {
        KEY.to_string()
    }
}

impl SnowflakeStorage for Admins {
    fn load(data: &Data) -> Result<Self, crate::Error>
    where
        for<'de> Self: Deserialize<'de>,
    {
        let data = data.bot_state.load::<Self>(KEY)?;
        Ok(data)
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
