use super::SnowflakeStorage;
use crate::state::BotStateInitialization;
use crate::Data;
use serde::{Deserialize, Serialize};

const KEY: &str = "active_collectors";

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct ActiveCollectors(pub Vec<u64>);

impl BotStateInitialization for ActiveCollectors {
    fn get_key(&self) -> String {
        KEY.to_string()
    }

    fn init_state_inner<T: Default + Serialize>(
        &self,
        data: &Data,
    ) -> Result<(), shuttle_persist::PersistError>
    where
        for<'de> Self: Deserialize<'de>,
        Self: Serialize,
    {
        let key = &self.get_key();
        let state = &data.bot_state;
        let _state_struct = state.save::<Self>(key, Self::default())?;

        Ok(())
    }
}

impl SnowflakeStorage for ActiveCollectors {
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
