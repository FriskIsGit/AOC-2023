use std::fs::read_to_string;

pub fn read_contents(day: u32, input: InputType) -> String {
    if day < 1 || day > 25 {
        panic!("That day doesn't exist!")
    }

    let path: String = match input{
        InputType::Demo => format!("./demo/demo{day}.txt"),
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

#[warn(dead_code)]
pub enum InputType{
    Demo, Full, Custom(String),
}

mod day1;