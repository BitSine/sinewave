mod command_usage;
mod ping;
mod prefix;
mod test;

use command_usage::*;
use ping::*;
use prefix::*;
use serenity::framework::standard::macros::group;
use test::*;

#[group]
#[commands(test, ping, command_usage, prefix)]
#[description = "general commands/utils"]
pub struct General;
