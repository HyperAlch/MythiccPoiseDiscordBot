use crate::state::BotStateInitialization;
use crate::Data;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const KEY: &str = "role_backup";

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct RoleBackups(HashMap<u64, Vec<u64>>);

impl RoleBackups {
    pub fn load(data: &Data) -> Result<Self, crate::Error>
    where
        for<'de> Self: Deserialize<'de>,
    {
        let data = data.bot_state.load::<Self>(KEY)?;
        Ok(data)
    }

    pub fn add<U: Into<u64>, R: Into<u64>>(
        &mut self,
        data: &Data,
        user_id: U,
        role_ids: &Vec<R>,
    ) -> Result<bool, crate::Error>
    where
        R: Copy,
    {
        let user_id: u64 = user_id.into();
        let role_ids: Vec<u64> = role_ids.into_iter().map(|x| (*x).into()).collect();

        if !self.0.contains_key(&user_id) {
            self.0.insert(user_id, role_ids);

            data.bot_state.save(&self.get_key(), self.clone())?;
            return Ok(true);
        }

        Ok(false)
    }
}

impl BotStateInitialization for RoleBackups {
    fn get_key(&self) -> String {
        KEY.to_string()
    }
}
