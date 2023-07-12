use poise::serenity_prelude::{ChannelId, RoleId, User, UserId};

pub trait InteractiveSnowflakeExt {
    fn get_interactive(&self) -> String;
}

pub trait FromStringSnowflakeExt {
    fn from(value: String) -> Self;
}

impl InteractiveSnowflakeExt for User {
    fn get_interactive(&self) -> String {
        format!("<@{}>", &self.id.0)
    }
}

impl InteractiveSnowflakeExt for UserId {
    fn get_interactive(&self) -> String {
        format!("<@{}>", &self.0)
    }
}

impl InteractiveSnowflakeExt for RoleId {
    fn get_interactive(&self) -> String {
        format!("<@&{}>", &self.0)
    }
}

impl InteractiveSnowflakeExt for ChannelId {
    fn get_interactive(&self) -> String {
        format!("<#{}>", &self.0)
    }
}
