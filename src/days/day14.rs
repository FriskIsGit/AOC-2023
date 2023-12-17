use std::mem::swap;
use std::time::Instant;

// # - cuba-shaped rocks, O - rounded rocks, . - empty space
pub fn parabolic_dish1(lines: Vec<String>) -> usize {
    // Rocks move north
    let mut map = parse_input(lines);
    roll_north(&mut map);
    print_table(&map);
    sum_map(&map)
}

fn sum_map(map: &Vec<Vec<u8>>) -> usize {
    let mut sum = 0;
    let all_rows = map.len();
    let all_cols = map[0].len();
    let mut count = 0;
    for row in 0..all_rows {
        for col in 0..all_cols {
            if map[row][col] == b'O' {
                count += 1;
            }
        }
        sum += (all_rows - row) * count;
        count = 0;
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

const CYCLES: usize = 1_000_000_000;
pub fn parabolic_dish2(lines: Vec<String>) -> usize {
    // Rocks move north, west, south, east = one cycle
    let mut map = parse_input(lines);
    let now = Instant::now();
    for i in 0..CYCLES {
        roll_north(&mut map);
        roll_west(&mut map);
        roll_south(&mut map);
        roll_east(&mut map);
    }
    print_table(&map);
    sum_map(&map)
}

fn roll_north(map: &mut Vec<Vec<u8>>) {
    let all_columns = map[0].len();
    for col_index in 0..all_columns {
        let mut resting_spot = None;
        let mut round_rock = None;
        let mut row_index = 0;
        // ITERATE: TOP => DOWN
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
}

fn roll_west(map: &mut Vec<Vec<u8>>) {
    let column_length = map[0].len();
    let row_length = map.len();
    for row_index in 0..row_length {
        let mut resting_spot = None;
        let mut round_rock = None;
        let mut col_index = 0;
        // ITERATE: LEFT => RIGHT
        while col_index < column_length {
            match map[row_index][col_index] {
                b'O' => {
                    if resting_spot.is_some() && round_rock.is_none() {
                        round_rock = Some(col_index);
                    }
                },
                b'#' => {
                    resting_spot = None;
                },
                b'.' => {
                    if resting_spot.is_none() {
                        resting_spot = Some(col_index);
                    }
                }
                _ => {}
            }
            if round_rock.is_some() && resting_spot.is_some() {
                let rock = round_rock.unwrap();
                let rest = resting_spot.unwrap();
                map[row_index][rock] = b'.';
                map[row_index][rest] = b'O';
                round_rock = None;
                resting_spot = None;
                col_index = rest;
            }
            col_index += 1;
        }
    }
}

fn roll_east(map: &mut Vec<Vec<u8>>) {
    let column_length = map[0].len();
    let row_length = map.len();
    for row_index in 0..row_length {
        let mut resting_spot = None;
        let mut round_rock = None;
        let mut col_index = (column_length - 1) as isize;
        // ITERATE: RIGHT => LEFT
        while col_index > -1 {
            match map[row_index][col_index as usize] {
                b'O' => {
                    if resting_spot.is_some() && round_rock.is_none() {
                        round_rock = Some(col_index);
                    }
                },
                b'#' => {
                    resting_spot = None;
                },
                b'.' => {
                    if resting_spot.is_none() {
                        resting_spot = Some(col_index);
                    }
                }
                _ => {}
            }
            if round_rock.is_some() && resting_spot.is_some() {
                let rock = round_rock.unwrap() as usize;
                let rest = resting_spot.unwrap() as usize;
                map[row_index][rock] = b'.';
                map[row_index][rest] = b'O';
                round_rock = None;
                resting_spot = None;
                col_index = rest as isize;
            }
            col_index -= 1;
        }
    }
}

fn roll_south(map: &mut Vec<Vec<u8>>) {
    let all_columns = map[0].len();
    for col_index in 0..all_columns {
        let mut resting_spot = None;
        let mut round_rock = None;
        let mut row_index = (map.len() - 1) as isize;
        // ITERATE: DOWN => TOP
        while row_index > -1 {
            match map[row_index as usize][col_index] {
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
                let rock = round_rock.unwrap() as usize;
                let rest = resting_spot.unwrap() as usize;
                map[rock][col_index] = b'.';
                map[rest][col_index] = b'O';
                round_rock = None;
                resting_spot = None;
                row_index = rest as isize;
            }
            row_index -= 1;
        }
    }
}