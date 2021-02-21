mod general;
mod handler;
mod help;
mod hooks;

use crate::handler::*;
use hooks::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use general::*;
use help::*;
use serenity::{
    framework::{standard::buckets::LimitedFor, StandardFramework},
    http::Http,
    Client,
};

#[tokio::main]
async fn main() {
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
        .configure(|c| c.on_mention(Some(bot_id)).prefix("~").owners(owners))
        // hooks
        .before(before)
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
        println!("Client error: {:?}", why);
    }
}
