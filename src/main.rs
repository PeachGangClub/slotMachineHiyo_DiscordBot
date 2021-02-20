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

async fn get_command_type(msg: &Message) -> (CommandTypeId,u8){
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

async fn gen_slot_str() -> String {
    let momo = "<:momo:747707481282838588>";
    let momogang = "<:momogang:747708446878728233>";
    let picture_num = 2;
    let slot_num = 3;

    let mut pictures = ["","",""];
    for a_picture in pictures.iter_mut() {
        let rand_num :i16 = rand::thread_rng().gen_range(0, picture_num);
        match rand_num{
            0=> *a_picture = momo,
            1=> *a_picture = momogang,
            _=> *a_picture = "error"
        }
    }

    let all_pictures = pictures[0].to_string()+pictures[1]+pictures[2]+"\n";
    return all_pictures;
}

async fn hiyoko_slot(ctx: &Context, msg: &Message,command_param: u8){
    println!("Shard {}", ctx.shard_id);

    let mut result = "".to_string();
    for _n in 0..command_param{
        let all_pictures = gen_slot_str().await;
        result = result+&all_pictures;
    }
    
    let response = MessageBuilder::new()
    .push(result)
    .build();
    if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
        println!("Error sending message: {:?}", why);
    }
   
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
