// this isnt just for the event handler but for other misc setup things

use mongodb::{Client, Database};
use serenity::{
    async_trait,
    client::{bridge::gateway::ShardManager, Context, EventHandler},
    model::prelude::Ready,
    prelude::{Mutex, TypeMapKey},
};
use std::{collections::HashMap, sync::Arc};

pub struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct CommandCounter;
impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}

pub struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

pub async fn create_db_connection() -> Option<Database> {
    Some(
        Client::with_uri_str(&dotenv::var("MONGO_URL").ok()?)
            .await
            .ok()?
            .database("sinewave"),
    )
}
