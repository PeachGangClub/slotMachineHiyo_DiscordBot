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
        let mut command_num = 0;
        if msg.content == "!ひよこスロット" {
            command_num = 1;
        }
        if msg.content == "!ひよこスロット*3" {
            command_num = 3;
        }
        
        for n in 0..command_num{
            hiyoko_slot(&ctx,&msg).await;
        }
        //暫定対応//

        // if msg.content == "!ひよこスロット*3" {
        //     let num_str = get_command_num();
        //     let num:i32 = num_str.parse().unwrap();
        //     hiyoko_slot(&ctx,&msg).await;
        //     hiyoko_slot(&ctx,&msg).await;
        //     hiyoko_slot(&ctx,&msg).await;
        // }


    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

// async fn get_command_num(command_str)=> &str{
//     let mut str = "".to_string();
//     let flg = 0;
//     for (i, c) in command_str.chars().enumerate() {
//         if c=="*" {
//             flg = 1;
//         }
//         if flg ==1{
//             str.push(c);
//         }
//     }
//     return str; 
// }

async fn hiyoko_slot(ctx: &Context, msg: &Message){
    let momo = "<:momo:747707481282838588>";
    let momogang = "<:momogang:747708446878728233>";
    let picture_num = 2;
    let slot_num = 3;
    println!("Shard {}", ctx.shard_id);
    let channel = match msg.channel_id.to_channel(&ctx).await {
        Ok(channel) => channel,
        Err(why) => {
            println!("Error getting channel: {:?}", why);
            return;
        },
    };

    let mut pictures = ["","",""];
    for a_picture in pictures.iter_mut() {
        let rand_num :i16 = rand::thread_rng().gen_range(0, picture_num);
        match rand_num{
            0=> *a_picture = momo,
            1=> *a_picture = momogang,
            _=> *a_picture = "error"
        }
    }
    
    let response = MessageBuilder::new()
    //.mention(&msg.author)
    //.push(" ")
    .push(pictures[0].to_string())
    .push(" ")
    .push(pictures[1].to_string())
    .push(" ")
    .push(pictures[2].to_string())
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
