use std::fs::read_to_string;

pub fn read_contents(day: u32, input: InputType) -> String {
    if day < 1 || day > 25 {
        panic!("That day doesn't exist!")
    }

    let path: String = match input{
        InputType::DemoPart1 => format!("./demo_p1/demo{day}.txt"),
        InputType::DemoPart2 => format!("./demo_p2/demo{day}.txt"),
        InputType::Full => format!("./full/input{day}.txt"),
        InputType::Custom(file_path) => file_path,
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
    #[warn(dead_code)]
    Custom(String),
}

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;