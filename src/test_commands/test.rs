use log::debug;
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

#[command]
pub async fn test(ctx: &Context, msg: &Message) -> CommandResult {
    debug!("hey it runs!");
    msg.channel_id
        .say(&ctx.http, "testing testing 1 2 3...")
        .await?;

    Ok(())
}
