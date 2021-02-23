mod send_mod_log;

use send_mod_log::*;
use serenity::framework::standard::macros::group;

#[group]
#[commands(send_mod_log)]
pub struct TestCommands;
