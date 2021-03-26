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
        HiyokoBingo,
        HiyokoBowling,
    }
    pub fn get_command_type(msg: &Message) -> CommandTypeId {
        let command_str = &msg.content;
        
        if let Ok(n) = scan_fmt!(command_str, "!ひよこスロット*{d}", u8) {
            if n >= 9 {
                return CommandTypeId::HiyokoSlot(9);
            }
            else {
                return CommandTypeId::HiyokoSlot(n);
            }
        }
        else if command_str.starts_with("!ひよこスロット") {
            return CommandTypeId::HiyokoSlot(1);
        }
        else if command_str.starts_with("!ひよこビンゴ") {
            return CommandTypeId::HiyokoBingo;
        }
        else if command_str.starts_with("!ひよこボウリング") {
            return CommandTypeId::HiyokoBowling;
        }
        else {
            return CommandTypeId::UnknownCommand;
        }
    }
}

