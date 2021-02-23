mod general;
mod handler;
mod help;
mod hooks;
mod mongo;
mod test_commands;

use crate::handler::*;
use crate::mongo::Guild;
use hooks::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use general::*;
use help::*;
use mongodb::{bson::doc, Client as MongoClient};
use serenity::{
    framework::{standard::buckets::LimitedFor, StandardFramework},
    http::Http,
    Client,
};

use log::{error, info, LevelFilter};
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
};

#[tokio::main]
async fn main() {
    // before anything init the logger

    let stdout: ConsoleAppender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{h({d(%Y-%m-%d %H:%M:%S %Z)} | {l})}> {m}\n",
        )))
        .build();

    let log_config = log4rs::config::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(log_config).unwrap();

    dotenv::dotenv().ok();

    let token = dotenv::var("TOKEN").expect("Error getting token");
    let http = Http::new_with_token(&token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| {
            c.on_mention(Some(bot_id))
                .dynamic_prefix(|_ctx, msg| {
                    Box::pin(async move {
                        let db = MongoClient::with_uri_str(&dotenv::var("MONGO_URL").ok()?)
                            .await
                            .ok()?
                            .database("sinewave");
                        let collection = db.collection_with_type::<Guild>("guilds");
                        let filter = doc! {
                            "id": msg.guild_id.unwrap().0
                        };
                        let guild =
                            collection
                                .find_one(filter, None)
                                .await
                                .ok()?
                                .unwrap_or(Guild {
                                    prefix: Some("~".to_string()),
                                    id: msg.guild_id.ok_or("Didnt run in a guild").ok()?.0,
                                    log_chnl_id: None,
                                });

                        guild.prefix
                    })
                })
                .owners(owners)
        })
        // hooks
        .before(before)
        .after(after)
        //buckets
        .bucket("complicated", |b| {
            b.limit(2)
                .time_span(30)
                .delay(5)
                .limit_for(LimitedFor::User)
        })
        .await
        //help and groups
        .help(&MY_HELP)
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<CommandCounter>(HashMap::default());
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
