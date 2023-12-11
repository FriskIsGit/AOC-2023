use crate::days::{*};

mod tests;
mod days;

fn main() {
    println!("Advent of Code shell:");
    loop {
        aoc_shell();
    }
}

fn aoc_shell() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    // Example: CREATE 25
    if buffer.starts_with("CREATE") {
        // Creates empty files: day_.rs test_day_.rs and input files: input_.txt demo_.txt
        let day = read_day(&buffer);
        let _ = std::fs::File::create(format!("src/days/day{day}.rs"));
        let _ = std::fs::File::create(format!("src/tests/test_day{day}.rs"));
        let _ = std::fs::File::create(format!("full/input{day}.txt"));
        let _ = std::fs::File::create(format!("demo_p1/demo{day}.txt"));
    }
    // Example LAUNCH 23
    else if buffer.starts_with("LAUNCH") {
        let day = read_day(&buffer);
        let input = read_lines(day, InputType::Full);
        match day {
            1 => {
                let sum = day1::trebuchet_1(input);
                println!("Day1: {sum}")
            },
            2 => {
                let output = day2::conundrum_1(input);
                println!("Day2: {output}")
            },
            3 => {
                let output = day3::gear_ratios1(input);
                println!("Day3: {output}")
            },
            4 => {
                let output = day4::scratchcards1(input);
                println!("Day4: {output}")
            }
            5 => {
                let output = day5::seeds1(input);
                println!("Day5: {output}")
            },
            6 => {
                let output = day6::boats1(input);
                println!("Day6: {output}")
            },
            7 => {
                let output = day7::camel_cards1(input);
                println!("Day7: {output}")
            },
            8 => {
                let output = day8::wasteland1(input);
                println!("Day8: {output}")
            },
            9 => {
                let output = day9::mirage1(input);
                println!("Day9: {output}")
            },
            10 => {
                let output = day10::maze1(input);
                println!("Day10: {output}")
            },
            _ => eprintln!("Day{day} is not completed")
        }
    }
}

fn read_day(buffer: &String) -> u32 {
    let whitespace = buffer.find(' ').expect("No day specified");
    // what buffer ends with is entirely reliant on the system
    let mut end = whitespace;
    let byte_skip = buffer.bytes().enumerate().skip(whitespace+1);
    for (index, byte) in byte_skip {
        match byte {
            b'0'..=b'9' => {
                end = index;
            }
            _ => {
                break
            }
        }
    }
    let day_str = &buffer[whitespace+1..=end];
    day_str.parse::<u32>().unwrap()
}
