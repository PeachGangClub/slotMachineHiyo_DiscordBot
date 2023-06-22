use super::common;
pub fn hiyoko_slot(slot_column: u8) -> String {
    common::output_time("hiyoko_slot function");
    let slot_row = 3;
    let emoji_str_list= vec!["<:momo:747707481282838588>","<:momogang:747708446878728233>"];
    let rand_vec = common::gen_rand_vec(slot_row*slot_column,emoji_str_list.len());
    let result_srting = common::gen_string(rand_vec, emoji_str_list, slot_row, slot_column);
    return result_srting;
}

pub fn kakumei_slot(slot_column: u8) -> String {
    common::output_time("kakumei_slot function");
    let slot_row = 3;
    let emoji_str_list= vec!["<:kizoku:1115461872251064402>","<:heimin:1115461823987187923>","<:king:1115495354373242880>"];
    let rand_vec = common::gen_rand_vec(slot_row*slot_column,emoji_str_list.len());
    let result_srting = common::gen_string(rand_vec, emoji_str_list, slot_row, slot_column);
    return result_srting;
}