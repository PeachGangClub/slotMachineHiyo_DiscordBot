pub mod post{
    use serenity::{
        model::channel::Message,
        prelude::*,
        utils::MessageBuilder,
    };
    pub async fn post_message(ctx: &Context, msg: &Message, message_str: String) {
        let response = MessageBuilder::new()
        .push(message_str)
        .build();
        if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
            println!("Error sending message: {:?}", why);
        }
    }
}

pub mod channel{
    use std::env;
    use serenity::{
        model::channel::Message,
        prelude::*,
    };
    pub async fn get_channel_name(ctx: &Context, msg: &Message) -> String {
        let channel_name = match msg.channel_id.to_channel(&ctx).await{
            Ok(channel) =>channel,
            Err(why) =>{
                println!("Error:{:?}",why);
                return "".to_string();
            },
        };
        return channel_name.to_string();
    }
    
    pub fn is_target_channel(channel_name: String) -> bool {
        let target_channel = env::var("TARGET_CHANNEL")
            .expect("Expected a target channnel");
        if channel_name != target_channel {
            println!("This is not target channel:{}",channel_name);
            return false;
        }
        return true;
    }
}

pub mod command{
    use serenity::model::channel::Message;
    pub enum CommandTypeId{
        UnknownCommand,
        HiyokoSlot(u8),
    }
    pub fn get_command_type(msg: &Message) -> CommandTypeId {
        let command_str = &msg.content;
        
        if let Ok(n) = scan_fmt!(command_str, "!ひよこスロット*{d}", u8) {
            if n >= 9 {
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
}

