use rand::Rng;

use serenity::{
    model::channel::Message,
    prelude::*,
};

use crate::common::post;

fn gen_slot_string(slot_row: u8, slot_column: u8) -> (String,bool) {
    let emoji_str_list= vec!["<:momo:747707481282838588>","<:momogang:747708446878728233>"];
    let emoji_length = emoji_str_list.len();
    //
    let result_atari = false;

    let mut slot_rand_result: Vec<u8> = Vec::new();
    for number in 0..slot_row*slot_column {//二次元配列にしようかな
        let rand = rand::thread_rng().gen_range(0, emoji_length);
        let rand = rand as u8;
        slot_rand_result.push(rand);
    }
    let result_slot_string = gen_string(slot_rand_result,emoji_str_list, slot_row, slot_column);
    return (result_slot_string, result_atari);
}

fn gen_string(slot_rand_result: Vec<u8>, emoji_str_list: Vec<&str>, slot_row: u8, slot_column: u8) -> String {
    let emoji_length = emoji_str_list.len();
    let mut result_slot_string = String::new();
    //二次元配列にして書き方変えようかなあ
    //for element in slot_rand_result.iter() {
    for number in 0..slot_row*slot_column {
        //最初も改行入るけど表示されなさそうだしいいかな
        if number%slot_row == 0{
            result_slot_string.push_str("\n");
        }
        result_slot_string.push_str(&emoji_str_list[slot_rand_result[number as usize] as usize]);//ここ.get()がよいかも
    }
    return result_slot_string;
}

pub async fn hiyoko_slot(ctx: &Context, msg: &Message,slot_column: u8){
    println!("Shard {}", ctx.shard_id);
    let slot_row = 3;

    let (result_srting, result_atari) = gen_slot_string(slot_row, slot_column);
    post::post_message(&ctx,&msg,result_srting).await;
}
