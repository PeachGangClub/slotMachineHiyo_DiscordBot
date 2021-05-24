use super::common;
pub enum CommandTypeId {
    UnknownCommand,
    HiyokoSlot(u8),
    HiyokoBingo,
    HiyokoBowling,
}
pub fn get_command_type(command_str: &str) -> CommandTypeId {
    common::output_time("get_command_type function");
    if let Ok(n) = scan_fmt!(command_str, "!ひよこスロット*{d}", u8) {
        if n >= 9 {
            return CommandTypeId::HiyokoSlot(9);
        } else {
            return CommandTypeId::HiyokoSlot(n);
        }
    } else if command_str.starts_with("!ひよこスロット") {
        return CommandTypeId::HiyokoSlot(1);
    } else if command_str.starts_with("!ひよこビンゴ") {
        return CommandTypeId::HiyokoBingo;
    } else if command_str.starts_with("!ひよこボウリング") {
        return CommandTypeId::HiyokoBowling;
    } else {
        return CommandTypeId::UnknownCommand;
    }
}
