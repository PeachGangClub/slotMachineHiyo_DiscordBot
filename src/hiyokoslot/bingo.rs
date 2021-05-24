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
