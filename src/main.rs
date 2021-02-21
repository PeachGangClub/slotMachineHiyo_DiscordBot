use std::env;
use rand::Rng;
//use List::{Cons, Nil};

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    utils::MessageBuilder,
};

struct Handler;
enum CommandTypeId{
    UnknownCommand,
    HiyokoSlot,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let channel_name = get_channel_name(&ctx,&msg).await;
        println!("channelIs:{}", channel_name);
        if is_target_channel(channel_name).await {
            let (command_type,command_param) = get_command_type(&msg).await;
            println!("commandid:{}",command_param);
            match (command_type,command_param) {
                (CommandTypeId::HiyokoSlot,_) => hiyoko_slot(&ctx,&msg,command_param).await,
                (CommandTypeId::UnknownCommand,_) => println!("This is not target command"),
            };
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn get_command_type(msg: &Message) -> (CommandTypeId,u8) {
    let command_str = &msg.content;
    let mut command_param = 0;
    let command_str_0to7 = command_str.chars().skip(0).take(8).collect::<String>();
    let command_str_8 = command_str.chars().skip(8).take(1).collect::<String>();
    let command_str_9 = command_str.chars().skip(9).take(1).collect::<String>();
    //println!("{}, {}, {}", command_str_0to7,command_str_8,command_str_9);
    if command_str_0to7 == "!ひよこスロット" {
        command_param = 1;
        if command_str_8 == "*"{
            command_param = match command_str_9.parse::<u8>(){
                Ok(_) => command_str_9.parse::<u8>().unwrap(),
                Err(_) => 1, 
            };
        }
        return (CommandTypeId::HiyokoSlot,command_param);
    }
    return (CommandTypeId::UnknownCommand,0);
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

async fn is_target_channel(channel_name: String) -> bool {
    if channel_name != "<#812364405840543764>" {
        println!("This is not target channel:{}",channel_name);
        return false;
    }
    return true;
}

//★未完成
async fn gen_slot_str(row:u8, col:u8) -> String {
    let emoji_list= get_emoji_list().await;

    let mut pictures_list = Vec::new();
    for n in 0..col*row{

        //println!("emojilist len: {}", emoji_list.len());
        let emoji_len = emoji_list.len() as u8;
        let rand_num :i16 = rand::thread_rng().gen_range(0, emoji_len).into();
        //let rand_num :i16 = rand::thread_rng().gen_range(0, 2);
        //pictures_list.push(emoji_list[rand_num].to_string());
        let rand_num_u8:u8 = rand_num as u8;
        println!("rand: {}", rand_num_u8);
        pictures_list.push(String::from(emoji_list[rand_num].to_string()));
        if (n%row==0) & (n!=0) {
            pictures_list.push("\n".to_string())
        }
    }
    let pictures_str = str_connection(pictures_list).await;
    return pictures_str;
   
}

 //★未完成
async fn str_connection(str_list: Vec<String>) -> String {
    let mut connected_str = "test".to_string();
    for n in 0..str_list.len(){
        println!("str1: {}", str_list[n]);
        //connected_str = connected_str + srt_list[n];
    }
    return connected_str;
}

async fn hiyoko_slot(ctx: &Context, msg: &Message,command_param: u8){
    //println!("Shard {}", ctx.shard_id);
    let result = gen_slot_str(3,command_param).await;
    post_message(&ctx,&msg,result).await;
}

//★OK?
async fn get_emoji_list() -> Vec<String> {
    let mut emoji_list = Vec::new();
    emoji_list.push(String::from("<:momo:747707481282838588>"));
    emoji_list.push(String::from("<:momogang:747708446878728233>"));
    return emoji_list;
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
