use std::cmp::max;

const RED_CUBES: u32 = 12;
const GREEN_CUBES: u32 = 13;
const BLUE_CUBES: u32 = 14;

pub fn conundrum_1(lines: Vec<String>) -> usize {
    let mut id_sum = 0;
    let mut game_id = 0;
    for line in lines {
        game_id += 1;
        let colon = line.find(':').expect("Every line should contain a colon");
        let mut bytes = line.bytes().skip(colon + 2);
        let mut possible_game = true;
        let mut cube_count = 0;
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
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
                b';' => {
                    if red > RED_CUBES || green > GREEN_CUBES || blue > BLUE_CUBES {
                        possible_game = false;
                        // Any invalid set invalidates a game
                        break;
                    }
                    red = 0;
                    green = 0;
                    blue = 0;
                }
                b',' => {}
                b' ' => {}
                _ => {
                    if expect_letter {
                        match byte {
                            b'r' => red += cube_count as u32,
                            b'g' => green += cube_count as u32,
                            b'b' => blue += cube_count as u32,
                            _ => panic!("Unreachable")
                        }
                        expect_letter = false;
                    }
                }
            }
        }
        // Process last set here since lines don't end with colons
        if possible_game && red <= RED_CUBES && green <= GREEN_CUBES && blue <= BLUE_CUBES {
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
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
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
                b';' => {
                    max_red = max(red, max_red);
                    max_green = max(green, max_green);
                    max_blue = max(blue, max_blue);
                    red = 0;
                    green = 0;
                    blue = 0;
                }
                b',' => {}
                b' ' => {}
                _ => {
                    if expect_letter {
                        match byte {
                            b'r' => red += cube_count as u32,
                            b'g' => green += cube_count as u32,
                            b'b' => blue += cube_count as u32,
                            _ => panic!("Unreachable")
                        }
                        expect_letter = false;
                    }
                }
            }
        }
        // Process last set here since lines don't end with colons
        max_red = max(red, max_red);
        max_green = max(green, max_green);
        max_blue = max(blue, max_blue);
        power_sum += (max_red * max_green * max_blue) as usize;
    }
    power_sum
}