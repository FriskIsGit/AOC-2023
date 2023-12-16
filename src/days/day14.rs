use std::mem::swap;

// # - cuba-shaped rocks, O - rounded rocks, . - empty space
pub fn parabolic_dish1(lines: Vec<String>) -> usize {
    // Rocks move north
    let mut map = parse_input(lines);
    let all_columns = map[0].len();
    for col_index in 0..all_columns {
        let mut resting_spot = None;
        let mut round_rock = None;
        let mut row_index = 0;
        while row_index < map.len() {
            match map[row_index][col_index] {
                b'O' => {
                    if resting_spot.is_some() && round_rock.is_none() {
                        round_rock = Some(row_index);
                    }
                },
                b'#' => {
                    resting_spot = None;
                },
                b'.' => {
                    if resting_spot.is_none() {
                        resting_spot = Some(row_index);
                    }
                }
                _ => {}
            }
            if round_rock.is_some() && resting_spot.is_some() {
                let rock = round_rock.unwrap();
                let rest = resting_spot.unwrap();
                map[rock][col_index] = b'.';
                map[rest][col_index] = b'O';
                round_rock = None;
                resting_spot = None;
                row_index = rest;
            }
            row_index += 1;
        }
    }
    print_table(&map);
    let mut sum = 0;
    let mut row = 0;
    let mut col = 0;
    let all_rows = map.len();
    while row < all_rows {
        let mut count = 0;
        while col < map[0].len() {
            if map[row][col] == b'O' {
                count += 1;
            }
            col += 1;
        }
        sum += (all_rows - row) * count;
        col = 0;
        row += 1;
    }
    sum
}

pub fn parse_input(mut lines: Vec<String>) -> Vec<Vec<u8>> {
    let mut map = Vec::with_capacity(lines.len());
    for line in lines.iter_mut() {
        let taken_line = std::mem::take(&mut *line);
        map.push(taken_line.into_bytes());
    }
    map
}

pub fn print_table(arr: &Vec<Vec<u8>>) {
    for row in arr {
        for el in row {
            print!("{}", char::from(*el));
        }
        println!()
    }
}