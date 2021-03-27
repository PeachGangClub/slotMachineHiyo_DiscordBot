pub mod common{
    use rand::Rng;
    use chrono::prelude::{DateTime, Local};
    pub fn gen_rand_vec(gen_length: u8, emoji_length: usize) -> Vec<u8> {
        let mut slot_rand_result: Vec<u8> = Vec::new();
        for _number in 0..gen_length {
            let rand = rand::thread_rng().gen_range(0, emoji_length);
            let rand = rand as u8;
            slot_rand_result.push(rand);
        }
        return slot_rand_result;
    }
    pub fn gen_string(
        slot_rand_result: Vec<u8>,
        emoji_str_list: Vec<&str>,
        slot_row: u8,
        slot_column: u8,
    ) -> String {
        let mut result_string = String::new();
        for number in 0..slot_row * slot_column {
            //最初も改行入るけど表示されなさそうだしいいかな
            if number % slot_row == 0 {
                result_string.push_str("\n");
            }
            result_string.push_str(&emoji_str_list[slot_rand_result[number as usize] as usize]);
            //ここ.get()がよいかも
        }
        return result_string;
    }
    pub fn isnotzero(vec_num: Vec<u8>) -> bool {
        //ジェネリクスの勉強の後書き換えるとよいかも
        return vec_num.iter().all(|&x| x > 0);
    }
    pub fn output_time(title_str: &str){
        let local: DateTime<Local> = Local::now();
        println!("{}:{}", local.to_string(), title_str);
    }
}

pub mod command {
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
}


pub mod slot{
    use super::common;
    pub fn hiyoko_slot(slot_column: u8) -> String {
        common::output_time("hiyoko_slot function");
        let slot_row = 3;
        let emoji_str_list= vec!["<:momo:747707481282838588>","<:momogang:747708446878728233>"];
        let rand_vec = common::gen_rand_vec(slot_row*slot_column,emoji_str_list.len());
        let result_srting = common::gen_string(rand_vec, emoji_str_list, slot_row, slot_column);
        return result_srting;
    }
}

pub mod bingo{
    use super::common;
    pub fn hiyoko_bingo() -> String {
        common::output_time("hiyoko_bingo function");
        let slot_row = 5;
        let slot_column = 5;
        let emoji_str_list = vec![
            "<:bingo_close:823544230546374676>",
            "<:bingo_open:825355221156495361>",
            "<:bingo_atari:825349102576336906>",
            "<:bingo_free:825352494141538304>",
        ];
    
        let mut rand_vec = common::gen_rand_vec(slot_row * slot_column, 2);
        let rand_vec = bingo_check(&mut rand_vec, slot_row, slot_column);
        let result_srting = common::gen_string(rand_vec.to_vec(), emoji_str_list, slot_row, slot_column);
        return result_srting;
    }
    
    fn bingo_check(rand_vec: &mut Vec<u8>, row: u8, column: u8) -> &mut Vec<u8> {
        //あとで何とかする
        rand_vec[12] = 3;
        //縦方向チェック
        for number in 0..row {
            //あとで何とかする
            let is_bingo = common::isnotzero(vec![
                rand_vec[(row * 0 + number) as usize],
                rand_vec[(row * 1 + number) as usize],
                rand_vec[(row * 2 + number) as usize],
                rand_vec[(row * 3 + number) as usize],
                rand_vec[(row * 4 + number) as usize],
            ]);
            if is_bingo == true {
                //println!("tate bingo");
                //あとで何とかする
                for n in 0..column {
                    rand_vec[(row * n + number) as usize] = 2;
                }
            }
        }
        //横方向チェック
        for number in 0..column {
            //あとで何とかする
            //let is_bingo = isnotzero(rand_vec[(((0+row*number) as usize)..((4+row*number) as usize)].to_vec());
            let is_bingo = common::isnotzero(vec![
                rand_vec[(0 + row * number) as usize],
                rand_vec[(1 + row * number) as usize],
                rand_vec[(2 + row * number) as usize],
                rand_vec[(3 + row * number) as usize],
                rand_vec[(4 + row * number) as usize],
            ]);
            if is_bingo == true {
                //println!("yoko bingo");
                //あとで何とかする
                for n in 0..row {
                    rand_vec[(n + row * number) as usize] = 2;
                }
            }
        }
        //斜め方向チェック
        //あとで何とかする
        let is_bingo = common::isnotzero(vec![
            rand_vec[0],
            rand_vec[6],
            rand_vec[12],
            rand_vec[18],
            rand_vec[24],
        ]);
        if is_bingo == true {
            //println!("naname bingo");
            //あとで何とかする rowは適切じゃないかも
            for n in 0..row {
                rand_vec[(row * n + n) as usize] = 2;
            }
        }
        //あとで何とかする
        let is_bingo = common::isnotzero(vec![
            rand_vec[4],
            rand_vec[8],
            rand_vec[12],
            rand_vec[16],
            rand_vec[20],
        ]);
        if is_bingo == true {
            //println!("naname bingo");
            //あとで何とかする
            for n in 0..row {
                rand_vec[(row * n - n + 4) as usize] = 2;
            }
        }
        return rand_vec;
    }
}

pub mod bowling{
    use super::common;
    pub fn hiyoko_bowling() -> String {
        common::output_time("hiyoko_bowling function");
        let emoji_str_list = vec![
            ":bowling:",
            ":basketball:",
            ":football:",
            ":flying_disc:",
            ":fishing_pole_and_fish:",
            ":curling_stone:",
            ":boxing_glove:",
            ":boomerang:",
        ];
        let buon_emoji_str = "<:buon:788397267882344468>";
        let dunk_hiyoko_emoji_str = "<:dunk:811008417337704498>";
    
        let rand_vec = common::gen_rand_vec(1, emoji_str_list.len());
        let mut result_string = String::new();
        if rand_vec[0] == 1 {
            result_string.push_str(dunk_hiyoko_emoji_str);
        } else {
            result_string.push_str(buon_emoji_str);
        }
        result_string.push_str(emoji_str_list[rand_vec[0] as usize]);
        return result_string;
    }    
}
