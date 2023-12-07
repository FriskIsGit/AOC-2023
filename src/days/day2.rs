use std::cmp::max;

const RED_CUBES: u8 = 12;
const GREEN_CUBES: u8 = 13;
const BLUE_CUBES: u8 = 14;

pub fn conundrum_1(lines: Vec<String>) -> usize {
    let mut id_sum = 0;
    let mut game_id = 0;
    for line in lines {
        game_id += 1;
        let colon = line.find(':').expect("Every line should contain a colon");
        let mut bytes = line.bytes().skip(colon + 2);
        let mut possible_game = true;
        let mut cube_count = 0;
        let mut expect_letter = false;
        while let Some(byte) = bytes.next() {
            match byte {
                b'1'..=b'9' => {
                    if expect_letter {
                        continue;
                    }
                    // Only account for two digits
                    let next_opt = bytes.next();
                    let next = next_opt.unwrap();
                    cube_count = if next == b' ' {
                        byte - 48
                    } else {
                        (byte - 48) * 10 + (next - 48)
                    };
                    expect_letter = true;
                }
                b';' => {}
                b',' => {}
                b' ' => {}
                _ => {
                    if !expect_letter {
                        continue
                    }
                    match byte {
                        b'r' => {
                            if cube_count > RED_CUBES {
                                possible_game = false;
                                break;
                            }
                        },
                        b'g' => {
                            if cube_count > GREEN_CUBES {
                                possible_game = false;
                                break;
                            }
                        },
                        b'b' => {
                            if cube_count > BLUE_CUBES {
                                possible_game = false;
                                break;
                            }
                        },
                        _ => panic!("Unreachable")
                    }
                    cube_count = 0;
                    expect_letter = false;
                }
            }
        }
        if possible_game {
            id_sum += game_id;
        }
    }
    id_sum
}

pub fn conundrum_2(lines: Vec<String>) -> usize {
    let mut power_sum = 0;
    for line in lines {
        let colon = line.find(':').expect("Every line should contain a colon");
        let mut bytes = line.bytes().skip(colon + 2);
        let mut cube_count = 0;
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        let mut expect_letter = false;
        while let Some(byte) = bytes.next() {
            match byte {
                b'1'..=b'9' => {
                    if expect_letter {
                        continue;
                    }
                    // Only account for two digits
                    let next_opt = bytes.next();
                    let next = next_opt.unwrap();
                    cube_count = if next == b' ' {
                        byte - 48
                    } else {
                        (byte - 48) * 10 + (next - 48)
                    } as usize;
                    expect_letter = true;
                }
                b';' => {}
                b',' => {}
                b' ' => {}
                _ => {
                    if !expect_letter {
                        continue
                    }
                    match byte {
                        b'r' => max_red = max(cube_count, max_red),
                        b'g' => max_green = max(cube_count, max_green),
                        b'b' => max_blue = max(cube_count, max_blue),
                        _ => panic!("Unreachable")
                    }
                    cube_count = 0;
                    expect_letter = false;
                }
            }
        }
        power_sum += (max_red * max_green * max_blue) as usize;
    }
    power_sum
}