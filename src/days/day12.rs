use std::cmp::max;
use std::fmt::{Display, Formatter, Write};

// operational (.) or damaged (#) or unknown (?)
pub fn hot_springs1(lines: Vec<String>) -> usize {
    let mut records = parse_input(lines);
    simplify_records(&mut records);
    for record in &records {
        println!("{}", record);
    }
    println!("Most question marks: {}", max_question_marks_of_records(&records));
    let mut unknowns = vec![];
    for record in &records {
        let mut unknown_indices = vec![];
        for i in 0..record.springs.len() {
            if record.springs[i] == b'?' {
                unknown_indices.push(i);
            }
        }
        unknowns.push(unknown_indices);
    }
    let mut matching = 0;
    for (i, record) in records.iter().enumerate() {
        let mut record_combinations = 0;
        let unknown_indices = &unknowns[i];
        let mut flipper = BitFlipper::new(unknown_indices.len());
        let combinations = usize::pow(2, unknown_indices.len() as u32);
        for _ in 0..combinations {
            // create a new spring vec and fill it with a combination
            let mut candidate_springs = record.springs.clone();
            let mut bit = 0;
            for u in unknown_indices {
                let spring = if flipper.bits[bit] { b'.' } else { b'#' };
                candidate_springs[*u] = spring;
                bit += 1;
            }
            // check if combination matches damaged spring groups
            if matching_groups(&candidate_springs, &record.damaged_groups) {
                // print_byte_arr(&candidate_springs);
                record_combinations += 1;
            }
            flipper.next();
        }
        println!("RECORD COMBINATIONS: {record_combinations}");
        matching += record_combinations;
        // println!("==============");
    }

    matching
}

fn simplify_records(records: &mut Vec<Record>) {
    // delete joined dots
    let mut looking_for_dot = true;
    for record in records.iter_mut() {
        let mut new_springs = vec![];
        for byte in &record.springs {
            if looking_for_dot && *byte == b'.' {
                looking_for_dot = false;
                new_springs.push(b'.');
            } else if !looking_for_dot && *byte == b'.' {
                continue
            } else {
                looking_for_dot = true;
                new_springs.push(*byte);
            }
        }
        record.springs = new_springs;
    }
    trim_trailing_dots(records);
    discard_front_section(records);
    // discard_back_section(records);
}

fn discard_back_section(records: &mut Vec<Record>) {
    for record in records {
        let mut count = 0;
        let mut found_damaged = false;
        let last_group_index = record.damaged_groups.len() - 1;
        let extent = (record.damaged_groups[last_group_index] * 2) as usize;
        let mut s: isize = (record.springs.len() - 1) as isize;
        let mut depth = 0;
        while depth < extent && s > -1 {
            if record.springs[s as usize] == b'#' {
                if !found_damaged {
                    found_damaged = true;
                }
                count += 1;
            }
            else if found_damaged {
                break;
            }
            s -= 1;
            depth += 1;
        }
        if count == record.damaged_groups[last_group_index] {
            unsafe { record.springs.set_len(s as usize); }
            unsafe { record.damaged_groups.set_len(record.damaged_groups.len() - 1); }
        }
    }
}

fn discard_front_section(records: &mut Vec<Record>) {
    for record in records {
        let mut count = 0;
        let mut found_damaged = false;
        let extent = (record.damaged_groups[0] * 2) as usize;
        let mut s: usize = 0;
        while s < extent && s < record.springs.len() {
            if record.springs[s] == b'#' {
                if !found_damaged {
                    found_damaged = true;
                }
                count += 1;
            }
            else if found_damaged {
                break;
            }
            s += 1;
        }
        if count == record.damaged_groups[0] {
            let partition = s + 1;
            record.springs.rotate_left(partition);
            unsafe { record.springs.set_len(record.springs.len() - partition); }
            record.damaged_groups.rotate_left(1);
            unsafe { record.damaged_groups.set_len(record.damaged_groups.len() - 1); }
        }
    }
}

fn trim_trailing_dots(records: &mut Vec<Record>) {
    for record in records.iter_mut() {
        let length = record.springs.len();
        let first_dot = record.springs[0] == b'.';
        let last_dot  = record.springs[length-1] == b'.';
        if first_dot && last_dot {
            record.springs.rotate_left(1);
            unsafe { record.springs.set_len(length - 2); }
        } else if first_dot {
            record.springs.rotate_left(1);
            unsafe { record.springs.set_len(length - 1); }
        } else if last_dot {
            unsafe { record.springs.set_len(length - 1); }
        }
    }
}

fn starts_with(vec: &Vec<u8>, byte: u8, count: usize) -> bool {
    for i in 0..count {
        if vec[i] != byte {
            return false
        }
    }
    return true;
}

fn ends_with(vec: &Vec<u8>, byte: u8, count: usize) -> bool {
    let mut i = vec.len() - 1;
    let mut times = 0;

    while times < count {
        if vec[i] != byte {
            return false
        }
        i -= 1;
        times += 1;
    }
    return true;
}

fn distributions(length: usize, xs: usize) -> usize {
    length - xs + 1
}
fn pattern_length(vec: &Vec<u32>, from: usize) -> usize {
    let mut sum = 0;
    for i in from..vec.len() {
        sum += vec[i] as usize;
    }
    sum + vec.len() - from - 1
}
fn max_question_marks_of_records(records: &Vec<Record>) -> usize {
    let mut max_val = 0;
    for record in records {
        let mut count = 0;
        for byte in &record.springs {
            if *byte == b'?' {
                count += 1;
            }
        }
        max_val = max(count, max_val);
        // println!("{record}");
    }
    max_val
}

pub fn matching_groups(springs: &Vec<u8>, damaged_groups: &Vec<u32>) -> bool {
    let mut dmg_count = 0;
    let mut groups = vec![];
    for (si, spring) in springs.iter().enumerate() {
        if *spring == b'#' {
            dmg_count += 1;
            if si == springs.len()-1 {
                // there's more groups than there should be
                if groups.len() == damaged_groups.len() {
                    return false;
                }
                groups.push(dmg_count);
                break;
            }
        } else if *spring == b'.' {
            if dmg_count == 0 {
                continue
            }
            // there's more groups than there should be
            if groups.len() == damaged_groups.len() {
                return false;
            }
            groups.push(dmg_count);
            dmg_count = 0;
        }
    }
    return groups.eq(damaged_groups);
}

fn print_byte_arr(vec: &Vec<u8>) {
    let mut str = String::with_capacity(vec.len());
    for el in vec {
        str.push(char::from(*el));
    }
    println!("{str}");
}

fn parse_input(lines: Vec<String>) -> Vec<Record> {
    let mut records = vec![];
    for line in lines {
        let whitespace = line.find(' ').expect("AOC liar");
        let springs = &line[0..whitespace];
        let numbers = &line[whitespace+1..line.len()];
        let split = numbers.split(',');
        let mut groups = vec![];
        for str_num in split {
            let parsed = str_num.parse::<u32>().expect("Number parsing error");
            groups.push(parsed);
        }
        let record = Record::new(springs.as_bytes().to_vec(), groups);
        records.push(record);
    }
    records
}


pub struct BitFlipper {
    pub bits: Vec<bool>
}
impl BitFlipper {
    pub fn new(len: usize) -> Self {
        Self { bits: vec![false; len] }
    }
    pub fn next(&mut self) {
        for i in 0..self.bits.len() {
            if self.bits[i] {
                self.bits[i] = false;
                continue
            }
            else {
                self.bits[i] = true;
                break;
            }
        }
    }
}

pub struct Record {
    springs: Vec<u8>,
    damaged_groups: Vec<u32>,
}

impl Record {
    pub fn new(records: Vec<u8>, groups: Vec<u32>) -> Self {
        Self { springs: records, damaged_groups: groups }
    }
    pub fn bad_springs(&self) -> u32 {
        self.damaged_groups.iter().sum()
    }
}
impl Display for Record {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // This is perhaps not the fastest but it's only for debug
        let mut str = String::new();
        for spring in &self.springs {
            str.push(char::from(*spring));
        }
        str.push(' ');
        for group in &self.damaged_groups {
            str.push_str(&group.to_string());
            str.push(',');
        }
        f.write_str(&str)
    }
}

pub fn hot_springs2(lines: Vec<String>) -> usize {
    let mut records = parse_input(lines);
    append_input(&mut records, 4);
    simplify_records(&mut records);
    println!("Most question marks: {}", max_question_marks_of_records(&records));
    for record in records {
        println!("{}", record);
    }
    0
}

fn append_input(records: &mut Vec<Record>, times: u8) {
    for record in records {
        let base_spring = record.springs.clone();
        for i in 0..times {
            record.springs.extend_from_slice(&base_spring);
            if i != times-1 {
                record.springs.push(b'?');
            }
        }
        let base_group = record.damaged_groups.clone();
        for _ in 0..times {
            record.damaged_groups.extend_from_slice(&base_group);
        }
    }
}