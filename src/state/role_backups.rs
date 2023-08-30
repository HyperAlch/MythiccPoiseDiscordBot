use crate::state::BotStateInitialization;
use crate::Data;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const KEY: &str = "role_backup";

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct RoleBackups(HashMap<u64, Vec<u64>>);

impl RoleBackups {
    pub fn load(data: &Data) -> Result<Self, anyhow::Error>
    where
        for<'de> Self: Deserialize<'de>,
    {
        let data = data.bot_state.load::<Self>(KEY);
        match data {
            Ok(data) => Ok(data),
            Err(e) => Err(anyhow::anyhow!("{}", e)),
        }
    }

    pub fn add<U: Into<u64>, R: Into<u64>>(
        &mut self,
        data: &Data,
        user_id: U,
        role_ids: &Vec<R>,
    ) -> Result<bool, anyhow::Error>
    where
        R: Copy,
    {
        let user_id: u64 = user_id.into();
        let role_ids: Vec<u64> = role_ids.into_iter().map(|x| (*x).into()).collect();

        if !self.0.contains_key(&user_id) {
            self.0.insert(user_id, role_ids);

            let result = data.bot_state.save(&self.get_key(), self.clone());
            match result {
                Ok(_) => return Ok(true),
                Err(e) => return Err(anyhow::anyhow!("{}", e)),
            }
        }

        Ok(false)
    }

    pub fn remove<U: Into<u64>>(
        &mut self,
        data: &Data,
        user_id: U,
    ) -> Result<Option<Vec<u64>>, anyhow::Error> {
        let user_id: u64 = user_id.into();

        if self.0.contains_key(&user_id) {
            let return_data = self.0.remove(&user_id);
            let result = data.bot_state.save(&self.get_key(), self.clone());
            match result {
                Ok(_) => return Ok(return_data),
                Err(e) => return Err(anyhow::anyhow!("{}", e)),
            }
        }

        Ok(None)
    }
}

impl BotStateInitialization for RoleBackups {
    fn get_key(&self) -> String {
        KEY.to_string()
    }
}
