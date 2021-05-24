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
