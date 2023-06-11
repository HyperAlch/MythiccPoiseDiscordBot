use crate::Data;
use crate::{constants::MASTER_ADMIN, state::BotStateInitialization};
use poise::serenity_prelude::{self as serenity};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Admins(pub Vec<u64>);

impl BotStateInitialization for Admins {
    fn get_key(&self) -> String {
        "admins".to_string()
    }
}

impl std::fmt::Display for Admins {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let raw_admins = &self.0;

        let mut admins = String::new();
        for admin in raw_admins {
            admins.push_str(&format!("{}\n", admin))
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
        let admins = data.bot_state.load::<Admins>("admins")?;
        Ok(admins)
    }

    pub fn add(&mut self, data: &Data, user: serenity::User) -> Result<bool, crate::Error> {
        let user_id: u64 = user.id.into();

        if !self.0.contains(&user_id) {
            self.0.push(user_id);

            data.bot_state.save::<Admins>("admins", self.clone())?;
            return Ok(true);
        }

        Ok(false)
    }
}
