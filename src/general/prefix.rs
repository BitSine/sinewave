use crate::mongo::Guild;
use log::debug;
use mongodb::{bson::doc, options::UpdateOptions, Client};
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

#[command]
#[only_in(guilds)]
#[bucket = "complicated"]
pub async fn prefix(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let db = Client::with_uri_str(&dotenv::var("MONGO_URL")?)
        .await?
        .database("sinewave");

    let collection = db.collection_with_type::<Guild>("guilds");

    let new_prefix = args.single::<String>();
    debug!("prefix from args: {:?}", new_prefix);

    if let Ok(prefix) = new_prefix {
        collection
            .update_one(
                doc! {
                    "id": msg.guild_id.ok_or("Didnt run in a guild")?.0
                },
                doc! {
                    "prefix": prefix.clone(),
                    // incase the guild doesnt exist already we just reupdate this
                    "id": msg.guild_id.ok_or("Didnt run in a guild")?.0
                },
                UpdateOptions::builder().upsert(true).build(),
            )
            .await?;

        debug!(
            "updated prefix for server {} to {}",
            msg.guild_id.ok_or("was not ran in a guild")?.0,
            prefix.clone(),
        );

        msg.channel_id
            .say(&ctx.http, format!("current prefix is {}", prefix))
            .await?;
    } else {
        debug!("no arg was present");

        let prefix = collection
            .find_one(
                doc! {
                    "id": msg.guild_id.ok_or("Didnt run in a guild")?.0
                },
                None,
            )
            .await?
            .unwrap_or(Guild {
                id: msg.guild_id.ok_or("Didnt run in a guild")?.0,
                prefix: Some("~".to_string()),
                log_chnl_id: None,
            })
            .prefix;

        debug!("prefix from db: {:?}", new_prefix);

        msg.channel_id
            .say(
                &ctx.http,
                format!(
                    "current prefix is {}",
                    prefix.ok_or("could not get prefix")?
                ),
            )
            .await?;
    }

    Ok(())
}
