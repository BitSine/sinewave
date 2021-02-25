mod command_usage;
mod ping;
mod prefix;
mod set_mod_log_channel;
mod test;

use command_usage::*;
use ping::*;
use prefix::*;
use serenity::framework::standard::macros::group;
use set_mod_log_channel::*;
use test::*;

#[group]
#[commands(test, ping, command_usage, prefix, set_mod_log_channel)]
#[description = "general commands/utils"]
pub struct General;
