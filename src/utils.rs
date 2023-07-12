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

pub mod time {
    use chrono::{DateTime, Utc};
    use poise::serenity_prelude::Timestamp;

    pub fn date_diff(date: &Timestamp) -> String {
        let today = Utc::now();

        // Check if we can access 0..10 before getting it
        let str_length = date.to_string().len();
        assert!(str_length >= 10);

        // Get only the date, not the time
        let date = &date.to_string()[0..10];

        let date = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d")
            .expect("date_diff: parse_from_str failed");

        let date = date
            .and_hms_opt(0, 0, 0)
            .expect("date_diff: and_hms_opt failed");

        let datetime = DateTime::<Utc>::from_utc(date, Utc);

        let diff = today.signed_duration_since(datetime);
        let days = diff.num_days();
        let years = days / 365;
        let remaining_days = days % 365;
        let months = remaining_days / 30;
        let days = remaining_days % 30;

        format!("{} years {} months {} days", years, months, days)
    }
}
