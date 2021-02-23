use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

#[command]
pub async fn set_mod_log_channel(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    Ok(())
}
