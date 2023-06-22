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


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn isnotzero_test(){
        assert!(!isnotzero(vec![1,1,1,0,0,1,0]));
        assert!(!isnotzero(vec![1,1,0,1,1,1,1]));
        assert!(!isnotzero(vec![1,0,1,0,1,0,1]));
        assert!(!isnotzero(vec![0,1,1,1,0,0,0]));
        assert!(!isnotzero(vec![0,1,0,0,1,1,0]));
        assert!(isnotzero(vec![1,1,1,1,1,1,1]));
    }
}
