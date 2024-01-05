use std::fs::read_to_string;

pub fn read_contents(day: u32, input: InputType) -> String {
    if day < 1 || day > 25 {
        panic!("That day doesn't exist!")
    }

    let path: String = match input{
        InputType::DemoPart1 => format!("./demo_p1/demo{day}.txt"),
        InputType::DemoPart2 => format!("./demo_p2/demo{day}.txt"),
        InputType::Full => format!("./full/input{day}.txt"),
        InputType::Custom(file_name) => format!("./custom/{file_name}"),
    };
    let content = match read_to_string(&path) {
        Ok(content) => content,
        Err(err) => panic!("{}; Failed to read: {path}", err.to_string())
    };

    content
}

pub fn read_lines(day: u32, input: InputType) -> Vec<String> {
    let content = read_contents(day, input);
    let mut lines = Vec::new();
    for line in content.lines() {
        lines.push(line.to_string())
    }
    lines
}

pub enum InputType {
    DemoPart1,
    DemoPart2,
    Full,
    Custom(&'static str),
}

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;