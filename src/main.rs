use std::env;
use rand::Rng;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    utils::MessageBuilder,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        //暫定対応//
        println!("channnelId:{}", msg.channel_id);
        if(msg.channel_id==809365851802173470){  
            let mut command_num = 0;
            let command_str_0to7 = msg.content.chars().skip(0).take(8).collect::<String>();
            let command_str_8 = msg.content.chars().skip(8).take(1).collect::<String>();
            let command_str_9 = msg.content.chars().skip(9).take(1).collect::<String>();
            //println!("{}, {}, {}", command_str_0to7,command_str_8,command_str_9);
            if command_str_0to7 == "!ひよこスロット" {
                command_num = 1;
                if command_str_8 == "*"{
                    command_num = match command_str_9.parse::<u8>(){
                        Ok(_)=> command_str_9.parse::<u8>().unwrap(),
                        Err(_)=> 1, 
                    };
                }

                println!("commandnum:{}", command_num);
                hiyoko_slot(&ctx,&msg,command_num).await;
            }
        }
        //暫定対応//
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
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

async fn hiyoko_slot(ctx: &Context, msg: &Message,command_num: u8){
    println!("Shard {}", ctx.shard_id);
    let channel = match msg.channel_id.to_channel(&ctx).await {
        Ok(channel) => channel,
        Err(why) => {
            println!("Error getting channel: {:?}", why);
            return;
        },
    };

    let mut result = "".to_string();
    for n in 0..command_num{
        let all_pictures = gen_slot_str().await;
        result = result+&all_pictures;
    }
    
    let response = MessageBuilder::new()
    //.mention(&msg.author)
    //.push(" ")
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
