use std::fmt::{Display, format, Formatter};
use std::ops::Range;

const SECTION_NAMES: [&str; 8] = ["seeds", "seed-to-soil", "soil-to-fertilizer", "fertilizer-to-water", "water-to-light",
"light-to-temperature", "temperature-to-humidity", "humidity-to-location"];

// [destination range start] [source range start] [range length]
pub fn seeds1(lines: Vec<String>) -> usize {
    let first_line = &lines[0];
    let colon = first_line.find(':').expect("Every line should contain a colon");
    let str_seeds = first_line[colon+2..].split(' ');
    let mut seeds = Vec::with_capacity(4);
    for token in str_seeds {
        let seed_id = token.parse::<usize>().unwrap();
        seeds.push(seed_id);
    }
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

        let mut vec_of_ranges = sections.last_mut().unwrap();
        vec_of_ranges.push(MappedRange::new(src_range, dest_range));
    }

    let mut translations = seeds;
    let mut next_translations = vec![];
    for i in 1..8 {
        println!("{}", SECTION_NAMES[i-1]);
        println!("{:?}", translations);
        for id in translations {
            let section = &sections[i];
            // if id can correspond to any range then it must choose this range's mapping
            // because apparently ranges don't overlap themselves and none of it is clearly stated in the problem
            let mut corresponded_to_none = true;
            for mapped_range in section {
                if mapped_range.can_correspond(id) {
                    let dest = mapped_range.get_destination(id);
                    next_translations.push(dest);
                    corresponded_to_none = false;
                    break;
                }
            }
            if corresponded_to_none {
                for mapped_range in section {
                    let dest = mapped_range.get_destination(id);
                    next_translations.push(dest);
                }
            }
        }
        translations = next_translations;
        next_translations = vec![];
    }
    *translations.iter().min().unwrap()
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
    pub fn can_correspond(&self, num: usize) -> bool {
        self.src_range.contains(&num)
    }
}
impl Display for MappedRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!("src[{:?}] dest[{:?}]", self.src_range, self.dest_range))
    }
}