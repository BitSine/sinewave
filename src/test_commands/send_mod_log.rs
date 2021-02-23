use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

#[command]
#[only_in(guilds)]
#[required_permissions(ADMINISTRATOR)]
pub async fn send_mod_log(ctx: &Context, msg: &Message) -> CommandResult {
    Ok(())
}
