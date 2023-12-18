use std::cmp::max;
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
const PROBE_SIZE: usize = 5;

pub fn parabolic_dish2(lines: Vec<String>) -> usize {
    // Rocks move north, west, south, east = one cycle
    let mut map = parse_input(lines);
    let now = Instant::now();
    let initial_cycles = 2 * max(map.len(), map[0].len());
    // Simulate some number of cycles to scramble the unique starting position
    for _ in 0..initial_cycles {
        cycle(&mut map);
    }

    let mut sum_history = Vec::with_capacity(initial_cycles);
    // Create data to find repeating intervals
    for _ in 0..initial_cycles {
        cycle(&mut map);
        sum_history.push(sum_map(&map));
    }
    let mut remaining_cycles = CYCLES - 2 * initial_cycles;
    // [interval ] [interval ] [last] [fill ]
    // [123456789] [123456789] [1234] [56789]
    let interval_length = interval_length(&sum_history);
    let last_interval_length = sum_history.len() % interval_length;
    let length_to_fill = interval_length - last_interval_length;
    remaining_cycles -= length_to_fill;
    remaining_cycles %= interval_length;
    // At this point the last value of the interval is our HEAD and the current state
    let last_index = interval_length - 1;
    let current_index = (remaining_cycles + last_index) % interval_length;
    println!("Total load: {}", sum_history[current_index]);
    println!("Time elapsed: {:?}", now.elapsed());
    sum_history[current_index]
}

fn cycle(map: &mut Vec<Vec<u8>>) {
    roll_north(map);
    roll_west(map);
    roll_south(map);
    roll_east(map);
}

fn interval_length(sequence: &Vec<usize>) -> usize {
    let mut probe_vec = Vec::with_capacity(PROBE_SIZE);
    for i in 0..PROBE_SIZE {
        probe_vec.push(sequence[i])
    }
    let mut i = PROBE_SIZE;
    let exit_bound = sequence.len() - PROBE_SIZE;
    while i < exit_bound {
        if sequence[i] != probe_vec[0] {
            i += 1;
            continue;
        }
        let slice = &sequence[i..i + PROBE_SIZE];
        if slice.iter().eq(&probe_vec) {
            return i;
        }
        i += 1;
    }
    panic!("No interval found")
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
                }
                b'#' => {
                    resting_spot = None;
                }
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
                }
                b'#' => {
                    resting_spot = None;
                }
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
                }
                b'#' => {
                    resting_spot = None;
                }
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
                }
                b'#' => {
                    resting_spot = None;
                }
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