use std::cmp::{max, min};
use std::fmt::{Display, Formatter};
use std::ops::Range;
use crate::days::day5::Traverse::{Increment, IncrementIndefinitely, MustIncrement};

const SECTION_NAMES: [&str; 8] = ["seeds", "seed-to-soil", "soil-to-fertilizer", "fertilizer-to-water", "water-to-light",
    "light-to-temperature", "temperature-to-humidity", "humidity-to-location"];

// [destination range start] [source range start] [range length]
pub fn seeds1(lines: Vec<String>) -> usize {
    let first_line = &lines[0];
    let colon = first_line.find(':').expect("Seeds line should contain a colon");
    let str_seeds = first_line[colon + 2..].split(' ');
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
    let mut sections: Vec<Vec<MappedRange>> = vec![];
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
            continue;
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
    for section in sections {
        let mut is_mapped = false;
        for mapped_range in section {
            if mapped_range.src_contains(id) {
                let dest = mapped_range.get_destination(id);
                id = dest;
                is_mapped = true;
                break;
            }
        }
    }
    id
}

#[derive(Default)]
pub struct MappedRange {
    pub src_range: Range<usize>,
    pub dest_range: Range<usize>,
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

    pub fn src_contains(&self, num: usize) -> bool {
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
    let mut str_seeds = first_line[colon + 2..].split(' ');
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
    seed_ranges.sort_by(|range1, range2| {
        range1.start.partial_cmp(&range2.start).unwrap()
    });

    // Sort the ranges by starting exit values to iterate over ascending input
    let mut sections = parse_sections(lines);
    for i in 0..sections.len() {
        let sect = &mut sections[i];
        sect.sort_by(|range1, range2| {
            range1.dest_range.start.partial_cmp(&range2.dest_range.start).unwrap()
        });
    }
    for sect in &sections {
        println!("====================={}=====================", sect.len());
        for range in sect {
            println!("{:?} => {:?}", range.src_range, range.dest_range);
        }
    }
    // Looking at the transformations as a function of: y=f(x) with a big input
    // It's best to assume that ranges of x can be skipped where delta=y-x is constant,
    // provided that the jump length is less than the length of the smallest range
    // so we don't miss a range in between and avoid potentially finding identical delta for a different range
    let mut best_jump = usize::MAX;
    for sect in &sections {
        for mapped_range in sect {
            let length = mapped_range.dest_range.end - mapped_range.dest_range.start;
            best_jump = min(length, best_jump);
        }
    }
    println!("BEST JUMP: {best_jump}");

    use std::time::Instant;
    let now = Instant::now();
    let mut min_location = usize::MAX;
    let last_index = sections.len() - 1;
    let last_section = &mut sections[last_index];
    let first_range = &last_section[0];
    // If last section doesn't have a destination range starting from 0 we'll create a virtual one
    let start = first_range.dest_range.start;
    if start > 0 {
        last_section.insert(0, MappedRange::new(0..start, 0..start));
    }
    let last_section = &sections[last_index]; //reborrow as immutable
    // Rather than performing a full brute force we can perform an optimized bruteforce from the back
    // Instead of passing input and getting an output, we can derive input from output
    // This code runs 27 times slower than in release
    'input: for exit_range in last_section {
        let mut previous_delta = usize::MAX; // some arbitrary value
        let mut jump_from = 0;
        let mut state = Increment;

        let last_exit_value = exit_range.dest_range.end;
        let mut x = exit_range.dest_range.start;
        while x < last_exit_value {
            let mut id = x;
            for back_sect in sections.iter().rev() {
                for mapped_range in back_sect {
                    let dest_range = &mapped_range.dest_range;
                    if dest_range.contains(&id) {
                        let offset = id - dest_range.start;
                        id = offset + mapped_range.src_range.start;
                        break;
                    }
                }
            }
            let delta = delta(x, id);
            // println!("x:{x}=>seed:{id} d={delta}");
            match state {
                Increment => {
                    if delta == previous_delta {
                        // JUMP
                        if x + best_jump < last_exit_value {
                            jump_from = x;
                            x += best_jump;
                            state = Traverse::Jump;
                        } else {
                            x += 1;
                            state = IncrementIndefinitely;
                        }
                        continue
                    }
                    x += 1;
                }
                MustIncrement(until) => {
                    x += 1;
                    if x >= until {
                        state = Increment;
                    }
                }
                IncrementIndefinitely => {
                    x += 1;
                }
                Traverse::Jump => {
                    // Overshot some change in values (expected but we don't expect to overshoot an entire range)
                    if previous_delta != delta {
                        x = jump_from + 1; // move the pointer back
                        state = MustIncrement(jump_from + 1);
                        previous_delta = delta;
                        continue // don't check seed ranges because this is not the lowest
                    } else if x + best_jump < last_exit_value {
                        // Jump again!
                        jump_from = x;
                        x += best_jump;
                        state = Traverse::Jump;
                    } else {
                        x += 1;
                        state = IncrementIndefinitely;
                    }
                }
            }
            // ATP: We have a seed, we check if it can be our input
            for seed_range in &seed_ranges {
                if !seed_range.contains(&id) {
                    continue
                }
                match state {
                    Increment | MustIncrement(_) | IncrementIndefinitely => {
                        min_location = x - 1; // -1 because it's ready for the next iteration but we quit
                        break 'input;
                    }
                    Traverse::Jump => { // Rare case?
                        println!("Jumped into valid seed.");
                        x = jump_from + 1; // move the pointer back
                        state = IncrementIndefinitely
                    }
                }
            }
            previous_delta = delta;
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    min_location
}

type Until = usize;
pub enum Traverse {
    Increment,
    MustIncrement(Until),
    IncrementIndefinitely,
    Jump,
}

pub fn delta(x: usize, y: usize) -> usize {
    (x as isize - y as isize).abs() as usize
}

fn _brute_force_seeds(seed_ranges: Vec<Range<usize>>, sections: &Vec<Vec<MappedRange>>) -> usize {
    let mut min_location = usize::MAX;
    for seed_range in seed_ranges {
        for seed in seed_range {
            let location = pass_through_sections(seed, &sections);
            min_location = min(min_location, location);
        }
    }
    min_location
}

fn _establish_src_bounds(sections: &Vec<Vec<MappedRange>>) -> Vec<MappedRange> {
    let mut section_bounds = Vec::with_capacity(sections.len());
    for section in sections {
        let mut lowest_src_start = usize::MAX;
        let mut highest_src_end = usize::MIN;

        let mut lowest_dest_start = usize::MAX;
        let mut highest_dest_end = usize::MIN;
        for range in section {
            lowest_src_start = min(lowest_src_start, range.src_range.start);
            highest_src_end = max(highest_src_end, range.src_range.end);

            lowest_dest_start = min(lowest_dest_start, range.dest_range.start);
            highest_dest_end = max(highest_dest_end, range.dest_range.end);
        }
        let mapped_range = MappedRange::new(
            lowest_src_start..highest_src_end,
            lowest_dest_start..highest_dest_end,
        );
        section_bounds.push(mapped_range)
    }
    section_bounds
}