use std::env;
use rand::Rng;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    utils::MessageBuilder,
};

struct Handler;
enum CommandTypeId{
    UnknownCommand,
    HiyokoSlot(u8),
}
#[macro_use] extern crate scan_fmt;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let channel_name = get_channel_name(&ctx,&msg).await;
        println!("channelIs:{}", channel_name);
        if is_target_channel(channel_name).await {
            let command_type = get_command_type(&msg).await;
            //println!("commandid:{}",slot_column);
            match command_type {
                CommandTypeId::HiyokoSlot(n) => hiyoko_slot(&ctx,&msg,n).await,
                CommandTypeId::UnknownCommand => println!("This is not target command"),
            };
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn get_command_type(msg: &Message) -> CommandTypeId {
    let command_str = &msg.content;
    
    if let Ok(n) = scan_fmt!(command_str, "!ひよこスロット*{d}", u8) {
        if n >= 10 {
            return CommandTypeId::HiyokoSlot(9);
        } else {
            return CommandTypeId::HiyokoSlot(n);
        }
    } else if command_str.starts_with("!ひよこスロット") {
        return CommandTypeId::HiyokoSlot(1);
    } else {
        return CommandTypeId::UnknownCommand;
    }
}

async fn post_message(ctx: &Context, msg: &Message, message_str: String) {
    let response = MessageBuilder::new()
    .push(message_str)
    .build();
    if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
        println!("Error sending message: {:?}", why);
    }
}

async fn get_channel_name(ctx: &Context, msg: &Message) -> String {
    let channel_name = match msg.channel_id.to_channel(&ctx).await{
        Ok(channel) =>channel,
        Err(why) =>{
            println!("Error:{:?}",why);
            return "".to_string();
        },
    };
    return channel_name.to_string();
}

async fn is_target_channel(channel_name: String) -> bool{
    if channel_name != "<#812364405840543764>" {
        println!("This is not target channel:{}",channel_name);
        return false;
    }
    return true;
}

async fn gen_slot_string(slot_row: u8, slot_column: u8) -> String {
    let momo = "<:momo:747707481282838588>";
    let momogang = "<:momogang:747708446878728233>";
    let rand_num = 2;

    let mut result_slot_string = String::new();
    for number in 0..slot_row*slot_column {
        if number%slot_row == 0{//最初も改行入るけど表示されなさそうだしいいかな
            result_slot_string.push_str("\n");
        }
        let rand_num :i16 = rand::thread_rng().gen_range(0, rand_num);
        match rand_num{
            0=> result_slot_string.push_str(momo),
            1=> result_slot_string.push_str(momogang),
            _=> result_slot_string.push_str("error")
        }

    }
    return result_slot_string;
}

async fn hiyoko_slot(ctx: &Context, msg: &Message,slot_column: u8){
    println!("Shard {}", ctx.shard_id);
    let slot_row = 3;

    let result = gen_slot_string(slot_row, slot_column).await;
    post_message(&ctx,&msg,result).await;
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
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
