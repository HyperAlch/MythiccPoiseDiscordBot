use poise::serenity_prelude::{User, UserId};

pub trait InteractiveUsernameExt {
    fn get_interactive_username(&self) -> String;
}

impl InteractiveUsernameExt for User {
    fn get_interactive_username(&self) -> String {
        format!("<@{}>", &self.id.0)
    }
}

impl InteractiveUsernameExt for UserId {
    fn get_interactive_username(&self) -> String {
        format!("<@{}>", &self.0)
    }
}
