use serde::{Deserialize, Serialize};

use crate::state::admins::Admins;
use shuttle_persist::{PersistError, PersistInstance};

use self::games::Games;
pub mod admins;
pub mod games;

pub struct Data {
    pub bot_state: PersistInstance,
}

pub fn init_all_state(data: &Data) -> Result<(), PersistError> {
    Admins::init_state(data)?;
    Games::init_state(data)?;

    Ok(())
}

trait BotStateInitialization: std::default::Default {
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
