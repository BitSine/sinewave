mod command_usage;
mod ping;
mod test;
use command_usage::*;
use ping::*;
use test::*;

use serenity::framework::standard::macros::group;

#[group]
#[commands(test, ping, command_usage)]
#[description = "general commands to see if the bot is working"]
pub struct General;
