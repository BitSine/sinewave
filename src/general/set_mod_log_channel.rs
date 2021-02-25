use log::{debug, LevelFilter};
use mongodb::{bson::doc, options::UpdateOptions};
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::{channel::Message, id::ChannelId},
};

use crate::{handler::create_db_connection, mongo::Guild};

#[command]
#[only_in(guilds)]
pub async fn set_mod_log_channel(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let chnl_id = args.single::<ChannelId>()?;

    debug!("channel id is {}", chnl_id.0);
    if log::max_level() == LevelFilter::Debug {
        chnl_id.say(&ctx.http, "test mod log").await?;
    }

    let db = create_db_connection()
        .await
        .ok_or("Error connecting to database")?;

    let collection = db.collection_with_type::<Guild>("guilds");

    collection
        .update_one(
            doc! {
                "id": msg.guild_id.ok_or("Didnt run in a guild")?.0
            },
            doc! {
                "log_chnl_id": chnl_id.0,
                // incase the guild doesnt exist already we just reupdate this
                "id": msg.guild_id.ok_or("Didnt run in a guild")?.0
            },
            UpdateOptions::builder().upsert(true).build(),
        )
        .await?;

    msg.channel_id
        .say(
            &ctx.http,
            format!("successfully set channel to <#{}>", chnl_id),
        )
        .await?;

    Ok(())
}
