use super::{BotStateInitialization, Data};
use poise::serenity_prelude::RoleId;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

const KEY: &str = "t_rooms";

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TRooms(pub Vec<Room>);

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Room {
    role_id: u64,
    channel_id: u64,
    is_open: bool,
}

impl Room {
    pub fn new<R: Into<u64>, C: Into<u64>>(role_id: R, channel_id: C, is_open: bool) -> Self
    where
        R: Copy,
    {
        let role_id: u64 = role_id.into();
        let channel_id: u64 = channel_id.into();
        Self {
            role_id,
            is_open,
            channel_id,
        }
    }

    pub fn toggle_open(&mut self) {
        self.is_open = !self.is_open;
    }
}

// Main functionality
impl TRooms {
    pub fn find_open_room(&mut self, data: &Data) -> Result<Option<(u64, u64)>, crate::Error> {
        let mut toggle = false;
        let mut index: usize = 0;

        for (i, room) in self.0.iter().enumerate() {
            if room.is_open {
                toggle = true;
                index = i;
                break;
            }
        }

        if toggle {
            let rooms = &mut self.0;
            rooms[index].toggle_open();

            self.save(data)?;

            return Ok(Some((self.0[index].role_id, self.0[index].channel_id)));
        }

        Ok(None)
    }

    pub fn find_room<R: Into<u64>>(
        &mut self,
        target_room: R,
    ) -> Result<Option<&mut Room>, crate::Error> {
        let target_room: u64 = target_room.into();
        let rooms = &mut self.0;

        for mut room in rooms.iter_mut() {
            if room.channel_id == target_room {
                return Ok(Some(room));
            }
        }

        Ok(None)
    }

    pub fn save(&self, data: &Data) -> Result<(), crate::Error> {
        data.bot_state.save(&self.get_key(), self.clone())?;

        Ok(())
    }
}

// Core functionality
impl TRooms {
    pub fn load(data: &Data) -> Result<Self, crate::Error>
    where
        for<'de> Self: Deserialize<'de>,
    {
        let data = data.bot_state.load::<Self>(KEY)?;
        Ok(data)
    }

    fn add(&mut self, data: &Data, room: Room) -> Result<(), crate::Error> {
        self.0.push(room);

        self.save(data)?;

        Ok(())
    }
}

impl BotStateInitialization for TRooms {
    fn get_key(&self) -> String {
        KEY.to_string()
    }

    fn init_state(data: &Data) -> Result<(), shuttle_persist::PersistError>
    where
        for<'de> Self: Deserialize<'de>,
        Self: Serialize,
    {
        let mut data_struct = Self::default();
        data_struct.init_state_inner::<Self>(&data)?;

        data.t_ids
            .clone()
            .into_iter()
            .map(|x| {
                (
                    RoleId::from_str(x.0.as_str()).unwrap(),
                    RoleId::from_str(x.1.as_str()).unwrap(),
                )
            })
            .for_each(|ids| {
                data_struct
                    .add(data, Room::new(ids.0, ids.1, true))
                    .unwrap();
            });

        Ok(())
    }
}
