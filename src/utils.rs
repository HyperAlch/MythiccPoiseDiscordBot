pub mod discord_cdn {
    use poise::serenity_prelude::User;

    pub fn get_avatar_url(user: &User) -> String {
        let avatar_url = match user.avatar.as_ref() {
            Some(url) => url,
            None => return "".to_string(),
        };

        format!(
            "https://cdn.discordapp.com/avatars/{}/{}.png",
            user.id, avatar_url,
        )
    }
}
