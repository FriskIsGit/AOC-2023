use std::cmp::max;
use std::fmt::{Display, format, Formatter, Write};
use std::ops::RangeInclusive;

// operational (.) or damaged (#) or unknown (?)
pub fn hot_springs1(lines: Vec<String>) -> usize {
    let records = parse_input(lines);
    let mut question_marks_max = 0;
    for record in &records {
        let mut count = 0;
        for byte in &record.springs {
            if *byte == b'?' {
                count += 1;
            }
        }
        question_marks_max = max(count, question_marks_max);
        // println!("{record}");
    }
    println!("MAX QUESTIONS: {question_marks_max}");
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
                matching += 1;
            }
            flipper.next();
        }
        // println!("==============");
    }

    matching
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