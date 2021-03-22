mod common;
mod hiyokoslot;

use std::env;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use crate::common::{command,channel};
use dotenv::dotenv;

struct Handler;

#[macro_use] extern crate scan_fmt;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let channel_name = channel::get_channel_name(&ctx,&msg).await;
        println!("channelIs:{}", channel_name);
        if channel::is_target_channel(channel_name){
            let command_type = command::get_command_type(&msg);
            //println!("commandid:{}",slot_column);
            match command_type {
                command::CommandTypeId::HiyokoSlot(n) => hiyokoslot::hiyoko_slot(&ctx,&msg,n).await,
                command::CommandTypeId::HiyokoBingo => hiyokoslot::hiyoko_bingo(&ctx,&msg).await,
                command::CommandTypeId::HiyokoBowling => hiyokoslot::hiyoko_bowling(&ctx,&msg).await,
                command::CommandTypeId::UnknownCommand => println!("This is not target command")
            };
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start_shards(2).await {
        println!("Client error: {:?}", why);
    }
}
