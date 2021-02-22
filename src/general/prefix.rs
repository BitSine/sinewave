use crate::mongo::Guild;
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
    if new_prefix.is_err() {
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
                prefix: "~".to_string(),
            })
            .prefix;

        msg.channel_id
            .say(&ctx.http, format!("current prefix is {}", prefix))
            .await?;

        return Ok(());
    }

    collection
        .update_one(
            doc! {
                "id": msg.guild_id.ok_or("Didnt run in a guild")?.0
            },
            doc! {
                "prefix": new_prefix?,
                "id": msg.guild_id.ok_or("Didnt run in a guild")?.0
            },
            UpdateOptions::builder().upsert(true).build(),
        )
        .await?;

    Ok(())
}
