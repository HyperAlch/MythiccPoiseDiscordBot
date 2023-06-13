use crate::state::admins::Admins;
use crate::state::SnowflakeStorage;

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
