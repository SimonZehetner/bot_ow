use serenity::framework::standard::macros::hook;
use serenity::framework::standard::DispatchError;
use serenity::framework::StandardFramework;
use serenity::model::prelude::Message;
use serenity::prelude::{Context, GatewayIntents};
use serenity::Client;
use std::env;
use std::process::exit;

use bot_ow::commands::general::GENERAL_GROUP;
use bot_ow::Handler;

const TOKEN_VARIABLE_NAME: &str = "DISCORD_TOKEN";
const DEFAULT_PREFIX: &str = "!";

#[tokio::main]
async fn main() {
    let token = get_token();
    if token.is_none() {
        println!("No token provided. Please make sure to either set the {}-environment variable or pass the token",
                 TOKEN_VARIABLE_NAME);
        exit(1);
    }
    let token = token.unwrap();

    let mut client = Client::builder(
        &token,
        GatewayIntents::GUILDS
            | GatewayIntents::GUILD_MEMBERS
            | GatewayIntents::MESSAGE_CONTENT
            | GatewayIntents::GUILD_MESSAGES,
    )
    .event_handler(Handler)
    .framework(
        StandardFramework::new()
            .configure(|c| c.prefix(DEFAULT_PREFIX))
            .on_dispatch_error(dispatch_error)
            .group(&GENERAL_GROUP),
    )
    .await
    .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

#[hook]
async fn dispatch_error(_ctx: &Context, _msg: &Message, error: DispatchError, command_name: &str) {
    println!("Failed to execute command {}: {:?}", command_name, error)
}

#[inline]
fn get_token() -> Option<String> {
    let token = env::var(TOKEN_VARIABLE_NAME);
    if let Ok(t) = token {
        return Some(t);
    }
    if let Some(t) = env::args().nth(1) {
        return Some(t);
    }
    None
}
