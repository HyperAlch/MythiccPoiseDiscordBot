use crate::state::BotStateInitialization;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
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
