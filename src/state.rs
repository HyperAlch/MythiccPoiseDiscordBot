use self::{
    active_collectors::ActiveCollectors, games::Games, guild_apply::GuildApply,
    role_backups::RoleBackups, t_rooms::TRooms,
};
use crate::state::admins::Admins;
use poise::serenity_prelude::{Cache, Role, RoleId};
use serde::{Deserialize, Serialize};
use shuttle_persist::{PersistError, PersistInstance};

pub mod active_collectors;
pub mod admins;
pub mod games;
pub mod guild_apply;
pub mod role_backups;
pub mod t_rooms;

pub struct Data {
    pub bot_state: PersistInstance,
    pub minor_events_channel: String,
    pub major_events_channel: String,
    pub follower_role: String,
    pub triggered_role: String,
    pub t_ids: Vec<(String, String)>,
    pub guild_apply_roles: Vec<String>,
    pub needs_to_apply_role: String,
    pub needs_to_apply_channel: String,
}

pub fn init_all_state(data: &Data) -> Result<(), anyhow::Error> {
    Admins::init_state(data)?;
    Games::init_state(data)?;
    ActiveCollectors::init_state(data)?;
    RoleBackups::init_state(data)?;
    TRooms::init_state(data)?;
    GuildApply::init_state(data)?;

    Ok(())
}

pub trait BotStateInitialization: std::default::Default {
    fn init_state_inner<'a, T: Default + Serialize>(
        &'a self,
        data: &'a Data,
    ) -> Result<(), PersistError>
    where
        for<'de> Self: Deserialize<'de>,
        Self: Serialize,
    {
        let key = &self.get_key();
        let state = &data.bot_state;

        let state_struct = state.load::<Self>(key);

        match state_struct {
            Ok(_) => (),
            Err(error) => match error {
                PersistError::Open(_) => {
                    let _result = state.save::<Self>(key, Self::default())?;
                }
                _ => (),
            },
        }

        Ok(())
    }

    fn get_key(&self) -> String;

    fn init_state(data: &Data) -> Result<(), anyhow::Error>
    where
        for<'de> Self: Deserialize<'de>,
        Self: Serialize + 'static,
    {
        let data_struct = Self::default();
        let result = data_struct.init_state_inner::<Self>(&data);
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow::anyhow!("{}", e)),
        }
    }
}

pub trait SnowflakeStorage: BotStateInitialization + Clone {
    fn load(data: &Data) -> Result<Self, anyhow::Error>
    where
        for<'de> Self: Deserialize<'de>;

    fn add(&mut self, data: &Data, id: u64) -> Result<bool, anyhow::Error>
    where
        for<'de> Self: Serialize,
    {
        if !self.snowflake_found(&id) {
            self.push_snowflake(id);

            let result = data.bot_state.save::<Self>(&self.get_key(), self.clone());
            match result {
                Ok(_) => return Ok(true),
                Err(e) => return Err(anyhow::anyhow!("{}", e)),
            };
        }

        Ok(false)
    }

    fn remove(&mut self, data: &Data, id: u64) -> Result<bool, anyhow::Error>
    where
        for<'de> Self: Serialize,
    {
        let index = self.snowflakes().position(|&i| i == id);

        match index {
            Some(index) => {
                self.remove_snowflake(index);
                let result = data.bot_state.save::<Self>(&self.get_key(), self.clone());
                match result {
                    Ok(_) => return Ok(true),
                    Err(e) => return Err(anyhow::anyhow!("{}", e)),
                }
            }
            None => return Ok(false),
        };
    }

    fn snowflake_found(&self, id: &u64) -> bool;
    fn push_snowflake(&mut self, id: u64);
    fn remove_snowflake(&mut self, index: usize);
    fn snowflakes(&self) -> std::slice::Iter<'_, u64>;
}

pub trait SnowflakeHashmapStorage: BotStateInitialization + Clone {
    fn load(data: &Data) -> Result<Self, anyhow::Error>
    where
        for<'de> Self: Deserialize<'de>;

    fn add(&mut self, data: &Data, key: String, value: u64) -> Result<bool, anyhow::Error>
    where
        for<'de> Self: Serialize,
    {
        if !self.snowflake_key_found(&key) && !self.snowflake_value_found(&value) {
            self.push_kv_inner(key, value);

            let result = data.bot_state.save::<Self>(&self.get_key(), self.clone());
            match result {
                Ok(_) => return Ok(true),
                Err(e) => return Err(anyhow::anyhow!("{}", e)),
            }
        }

        Ok(false)
    }

    fn remove(&mut self, data: &Data, key: String) -> Result<bool, anyhow::Error>
    where
        for<'de> Self: Serialize,
    {
        let index = self.all().position(|i| i.0 == &key);

        match index {
            Some(_index) => {
                self.remove_inner(key);
                let result = data.bot_state.save::<Self>(&self.get_key(), self.clone());
                match result {
                    Ok(_) => return Ok(true),
                    Err(e) => return Err(anyhow::anyhow!("{}", e)),
                }
            }
            None => Ok(false),
        }
    }

    fn snowflake_key_found(&self, key: &String) -> bool;
    fn snowflake_value_found(&self, value: &u64) -> bool;
    fn push_kv_inner(&mut self, key: String, value: u64);
    fn remove_inner(&mut self, key: String);
    fn all(&self) -> std::collections::hash_map::Iter<'_, std::string::String, u64>;
}
pub trait SnowflakesToRoles: SnowflakeStorage {
    fn to_roles(&self, cache: &Cache) -> Vec<Role> {
        let snowflakes = self.snowflakes();
        let mut roles: Vec<Role> = vec![];

        for snowflake in snowflakes {
            let role = RoleId(*snowflake);
            let role = role.to_role_cached(cache);
            match role {
                Some(value) => roles.push(value),
                None => (),
            }
        }

        roles
    }
}
