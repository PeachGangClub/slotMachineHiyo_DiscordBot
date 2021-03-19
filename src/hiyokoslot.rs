use rand::Rng;

use serenity::{
    model::channel::Message,
    prelude::*,
};

use crate::common::post;

fn gen_rand_vec(gen_length: u8, emoji_length:usize) -> Vec<u8> {
    let mut slot_rand_result: Vec<u8> = Vec::new();
    for number in 0..gen_length {
        let rand = rand::thread_rng().gen_range(0, emoji_length);
        let rand = rand as u8;
        slot_rand_result.push(rand);
    }
    return slot_rand_result;
}

fn gen_string(slot_rand_result: Vec<u8>, emoji_str_list:Vec<&str>, slot_row: u8, slot_column: u8) -> String {
    let mut result_string = String::new();
    for number in 0..slot_row*slot_column {
        //最初も改行入るけど表示されなさそうだしいいかな
        if number%slot_row == 0{
            result_string.push_str("\n");
        }
        result_string.push_str(&emoji_str_list[slot_rand_result[number as usize] as usize]);//ここ.get()がよいかも
    }
    return result_string;
}

pub async fn hiyoko_slot(ctx: &Context, msg: &Message,slot_column: u8){
    println!("Shard {}", ctx.shard_id);
    let slot_row = 3;
    let emoji_str_list= vec!["<:momo:747707481282838588>","<:momogang:747708446878728233>"];

    let rand_vec = gen_rand_vec(slot_row*slot_column,emoji_str_list.len());
    let result_srting = gen_string(rand_vec, emoji_str_list, slot_row, slot_column);
    post::post_message(&ctx,&msg,result_srting).await;
}

pub async fn hiyoko_bingo(ctx: &Context, msg: &Message){
    println!("Shard {}", ctx.shard_id);
    let slot_row = 5;
    let slot_column = 5;
    let emoji_str_list= vec!["<:momo:747707481282838588>","<:momogang:747708446878728233>","<:peace:786899830453567498> "];

    let mut rand_vec = gen_rand_vec(slot_row*slot_column,emoji_str_list.len()-1);
    rand_vec[12] = 2;
    let result_srting = gen_string(rand_vec, emoji_str_list, slot_row, slot_column);
    post::post_message(&ctx,&msg,result_srting).await;
}
