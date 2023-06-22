pub mod common{
    use super::{channel, post};
    use crate::hiyokoslot::{command, bingo, bowling, slot};
    use serenity::model::prelude::Message;
    use serenity::client::Context;

    pub async fn receptionist(ctx: &Context, msg: &Message){
        let channel_name = channel::get_channel_name(&ctx, &msg).await;
        println!("channelIs:{}", channel_name);
        if channel::is_target_channel(channel_name) {
            let command_type = command::get_command_type(&msg.content);
            match command_type {
                command::CommandTypeId::HiyokoSlot(n) => post::post_message(&ctx, &msg, slot::hiyoko_slot(n)).await,
                command::CommandTypeId::HiyokoBingo => post::post_message(&ctx, &msg, bingo::hiyoko_bingo()).await,
                command::CommandTypeId::HiyokoBowling => post::post_message(&ctx, &msg, bowling::hiyoko_bowling()).await,
                command::CommandTypeId::KakumeiSlot => post::post_message(&ctx, &msg, slot::kakumei_slot()).await,
                command::CommandTypeId::UnknownCommand => println!("This is not target command"),
            };
        }
    }
}

pub mod channel {
    use serenity::{model::channel::Message, prelude::*};
    use std::env;
    pub async fn get_channel_name(ctx: &Context, msg: &Message) -> String {
        let channel_name = match msg.channel_id.to_channel(&ctx).await {
            Ok(channel) => channel,
            Err(why) => {
                println!("Error:{:?}", why);
                return "".to_string();
            }
        };
        return channel_name.to_string();
    }
    pub fn is_target_channel(channel_name: String) -> bool {
        let target_channel = env::var("TARGET_CHANNEL").expect("Expected a target channnel");
        if channel_name != target_channel {
            println!("This is not target channel:{}", channel_name);
            return false;
        }
        return true;
    }
}

pub mod post {
    use serenity::{model::channel::Message, prelude::*, utils::MessageBuilder};
    pub async fn post_message(ctx: &Context, msg: &Message, message_str: String) {
        let response = MessageBuilder::new().push(message_str).build();
        if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
            println!("Error sending message: {:?}", why);
        }
    }
}
