use std::fmt::{Display, format, Formatter, Write};
use std::ops::RangeInclusive;

// operational (.) or damaged (#) or unknown (?)
pub fn hot_springs1(lines: Vec<String>) -> usize {
    let records = parse_input(lines);
    for record in &records {
        println!("{record}");
    }
    let mut arrangement_sum  = 0;
    for record in &records {
        let mut unknown_indices = vec![];
        let groups = &record.damaged_groups;
        for i in 0..record.springs.len() {
            let spring = record.springs[i];
            if spring == b'?' {
                unknown_indices.push(i);
            }
        }
        for i in 0..record.springs.len() {
            let spring = record.springs[i];
            if spring == b'?' {

            }
        }
    }
    arrangement_sum
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

struct Record {
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