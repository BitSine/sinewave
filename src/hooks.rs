use crate::CommandCounter;
use log::{debug, info, warn};
use serenity::{
    client::Context,
    framework::standard::{macros::hook, CommandResult},
    model::channel::Message,
};

#[hook]
pub async fn before(ctx: &Context, msg: &Message, command_name: &str) -> bool {
    info!(
        "Got command '{}' by user '{}'",
        command_name, msg.author.name
    );

    let mut data = ctx.data.write().await;
    let counter = data
        .get_mut::<CommandCounter>()
        .expect("Expected CommandCounter in TypeMap.");
    let entry = counter.entry(command_name.to_string()).or_insert(0);
    *entry += 1;
    debug!(
        "Current command count for {} is {}",
        command_name,
        counter.get(command_name).unwrap_or(&0)
    );

    true
}

#[hook]
pub async fn delay_action(ctx: &Context, msg: &Message) {
    // You may want to handle a Discord rate limit if this fails.
    let _ = msg.react(ctx, '⏱').await;
}

#[hook]
pub async fn after(
    ctx: &Context,
    msg: &Message,
    command_name: &str,
    command_result: CommandResult,
) {
    match command_result {
        Ok(()) => info!("Processed command '{}'", command_name),
        Err(why) => {
            warn!("Command '{}' returned error {:?}", command_name, why);
            let _ = msg
                .channel_id
                .say(
                    &ctx.http,
                    format!("Command `{}` returned error `{}`", command_name, why),
                )
                .await;
        }
    }
}
