use std::cmp::{max, min};
use std::fmt::{Display, Formatter};
use std::ops::Range;

const SECTION_NAMES: [&str; 8] = ["seeds", "seed-to-soil", "soil-to-fertilizer", "fertilizer-to-water", "water-to-light",
"light-to-temperature", "temperature-to-humidity", "humidity-to-location"];

// [destination range start] [source range start] [range length]
pub fn seeds1(lines: Vec<String>) -> usize {
    let first_line = &lines[0];
    let colon = first_line.find(':').expect("Seeds line should contain a colon");
    let str_seeds = first_line[colon+2..].split(' ');
    let mut seeds = Vec::with_capacity(4);
    for token in str_seeds {
        let seed_id = token.parse::<usize>().unwrap();
        seeds.push(seed_id);
    }
    let sections = parse_sections(lines);
    let mut min_location = usize::MAX;
    for seed in seeds {
        let location = pass_through_sections(seed, &sections);
        min_location = min(min_location, location);
    }
    min_location
}

fn parse_sections(lines: Vec<String>) -> Vec<Vec<MappedRange>> {
    let mut sections: Vec<Vec<MappedRange>> = vec![vec![]];
    let mut expect_header = true;
    for i in 2..lines.len() {
        let line = &lines[i];
        if expect_header {
            // push section
            sections.push(vec![]);
            expect_header = false;
            continue;
        }
        if line.is_empty() {
            expect_header = true;
            continue
        }

        // Lines with numbers only here
        let mut str_numbers = line.split(' ');
        let dest_range_st = str_numbers.next().unwrap().parse::<usize>().unwrap();
        let src_range_st = str_numbers.next().unwrap().parse::<usize>().unwrap();
        let length = str_numbers.next().unwrap().parse::<usize>().unwrap();

        let src_range = src_range_st..src_range_st + length;
        let dest_range = dest_range_st..dest_range_st + length;

        let vec_of_ranges = sections.last_mut().unwrap();
        vec_of_ranges.push(MappedRange::new(src_range, dest_range));
    }
    sections
}

fn pass_through_sections(seed: usize, sections: &Vec<Vec<MappedRange>>) -> usize {
    let mut id = seed;
    for i in 1..8 {
        let section = &sections[i];
        let mut is_mapped = false;
        for mapped_range in section {
            if mapped_range.contains(id) {
                let dest = mapped_range.get_destination(id);
                id = dest;
                is_mapped = true;
                break;
            }
        }
    }
    id
}

pub struct MappedRange {
    src_range: Range<usize>,
    dest_range: Range<usize>
}
impl MappedRange {
    pub fn new(src_range: Range<usize>, dest_range: Range<usize>) -> Self {
        Self { src_range, dest_range }
    }
    pub fn get_destination(&self, num: usize) -> usize {
        if self.src_range.contains(&num) {
            let offset = num - self.src_range.start;
            return self.dest_range.start + offset;
        }
        num
    }
    pub fn contains(&self, num: usize) -> bool {
        self.src_range.contains(&num)
    }
}
impl Display for MappedRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!("src[{:?}] dest[{:?}]", self.src_range, self.dest_range))
    }
}

// pair: [start] [length]
pub fn seeds2(lines: Vec<String>) -> usize {
    let first_line = &lines[0];
    let colon = first_line.find(':').expect("Seeds line should contain a colon");
    let mut str_seeds = first_line[colon+2..].split(' ');
    let mut seed_ranges: Vec<Range<usize>> = vec![];
    loop {
        if let Some(left) = str_seeds.next() {
            let right = str_seeds.next().expect("Uneven pairs");
            let seed_id = left.parse::<usize>().unwrap();
            let length = right.parse::<usize>().unwrap();
            seed_ranges.push(seed_id..seed_id + length);
        } else {
            break;
        }
    }
    let sections = parse_sections(lines);
    let mut min_location = usize::MAX;
    for seed_range in seed_ranges {
        for seed in seed_range {
            let location = pass_through_sections(seed, &sections);
            min_location = min(min_location, location);
        }
    }
    min_location
}


