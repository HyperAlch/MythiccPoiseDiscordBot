use crate::state::SnowflakeStorage;
use crate::{constants::MASTER_ADMIN, state::admins::Admins};
use poise::serenity_prelude::{self as serenity};

/// Returns true if author is on the admin list
pub async fn is_on_admin_list(ctx: crate::Context<'_>) -> Result<bool, crate::Error> {
    let data = ctx.data();

    let command_user = ctx.author().id;

    let admins = Admins::load(data)?;
    if !admins.snowflake_found(command_user.as_u64()) {
        ctx.say("You are not authorized to use this command...")
            .await?;
        return Ok(false);
    }

    Ok(true)
}

pub fn is_master_admin(user: &serenity::User) -> bool {
    if user.id.0 == MASTER_ADMIN {
        true
    } else {
        false
    }
}
