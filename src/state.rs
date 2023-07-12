use self::{active_collectors::ActiveCollectors, games::Games};
use crate::state::admins::Admins;
use poise::serenity_prelude::{Cache, Role, RoleId};
use serde::{Deserialize, Serialize};
use shuttle_persist::{PersistError, PersistInstance};

pub mod active_collectors;
pub mod admins;
pub mod games;

pub struct Data {
    pub bot_state: PersistInstance,
    pub minor_events_channel: String,
    pub major_events_channel: String,
    pub follower_role: String,
}

pub fn init_all_state(data: &Data) -> Result<(), PersistError> {
    Admins::init_state(data)?;
    Games::init_state(data)?;
    ActiveCollectors::init_state(data)?;

    Ok(())
}

pub trait BotStateInitialization: std::default::Default {
    fn init_state_inner<T: Default + Serialize>(&self, data: &Data) -> Result<(), PersistError>
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

    fn init_state(data: &Data) -> Result<(), PersistError>
    where
        for<'de> Self: Deserialize<'de>,
        Self: Serialize,
    {
        let data_struct = Self::default();
        data_struct.init_state_inner::<Self>(&data)?;
        Ok(())
    }
}

pub trait SnowflakeStorage: BotStateInitialization + Clone {
    fn load(data: &Data) -> Result<Self, crate::Error>
    where
        for<'de> Self: Deserialize<'de>;

    fn add(&mut self, data: &Data, id: u64) -> Result<bool, crate::Error>
    where
        for<'de> Self: Serialize,
    {
        if !self.snowflake_found(&id) {
            self.push_snowflake(id);

            data.bot_state.save::<Self>(&self.get_key(), self.clone())?;
            return Ok(true);
        }

        Ok(false)
    }

    fn remove(&mut self, data: &Data, id: u64) -> Result<bool, crate::Error>
    where
        for<'de> Self: Serialize,
    {
        let index = self.snowflakes().position(|&i| i == id);

        match index {
            Some(index) => {
                self.remove_snowflake(index);
                data.bot_state.save::<Self>(&self.get_key(), self.clone())?;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    fn snowflake_found(&self, id: &u64) -> bool;
    fn push_snowflake(&mut self, id: u64);
    fn remove_snowflake(&mut self, index: usize);
    fn snowflakes(&self) -> std::slice::Iter<'_, u64>;
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
