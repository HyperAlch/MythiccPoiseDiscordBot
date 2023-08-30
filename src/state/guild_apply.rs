use std::collections::HashMap;

use super::SnowflakeHashmapStorage;
use crate::Data;
use crate::{extensions::InteractiveSnowflakeExt, state::BotStateInitialization};
use poise::serenity_prelude::ChannelId;
use serde::{Deserialize, Serialize};

const KEY: &str = "guild_apply";

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GuildApply(pub HashMap<String, u64>);

impl BotStateInitialization for GuildApply {
    fn get_key(&self) -> String {
        KEY.to_string()
    }
}

impl SnowflakeHashmapStorage for GuildApply {
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

    fn snowflake_key_found(&self, key: &String) -> bool {
        self.0.contains_key(key)
    }

    fn snowflake_value_found(&self, value: &u64) -> bool {
        for item in self.0.values() {
            if item == value {
                return true;
            }
        }
        false
    }

    fn all(&self) -> std::collections::hash_map::Iter<'_, std::string::String, u64> {
        self.0.iter()
    }

    fn push_kv_inner(&mut self, key: String, value: u64) {
        self.0.insert(key, value);
    }

    fn remove_inner(&mut self, key: String) {
        self.0.remove(&key);
    }
}

impl std::fmt::Display for GuildApply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for game in self.all() {
            let k = game.0;
            let v = ChannelId(*game.1).get_interactive();
            out.push_str(&format!("{}: {}\n", k, v))
        }

        write!(f, "{}", out)
    }
}
