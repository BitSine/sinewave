mod ping;
mod send_mod_log;
mod test;

use ping::*;
use send_mod_log::*;
use serenity::framework::standard::macros::group;
use test::*;

#[group]
#[commands(send_mod_log, ping, test)]
pub struct TestCommands;
