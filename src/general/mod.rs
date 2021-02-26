mod command_usage;
mod prefix;
mod set_mod_log_channel;

use command_usage::*;
use prefix::*;
use serenity::framework::standard::macros::group;
use set_mod_log_channel::*;

#[group]
#[commands(command_usage, prefix, set_mod_log_channel)]
#[description = "general commands/utils"]
pub struct General;
