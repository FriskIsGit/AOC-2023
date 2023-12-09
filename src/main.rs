mod tests;
mod days;

fn main() {
    // Creates empty files: day_.rs test_day_.rs and input files: input_.txt demo_.txt
    read_create_day();
}

fn read_create_day() {
    println!("Advent of code shell:");
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    // Example: CREATE 25
    if buffer.starts_with("CREATE") {
        let whitespace = buffer.find(' ').expect("No day specified");
        let day_str = &buffer[whitespace+1..buffer.len()-2]; // ends with both \r\n
        let day = day_str.parse::<usize>().unwrap();
        let _ = std::fs::File::create(format!("src/days/day{day}.rs"));
        let _ = std::fs::File::create(format!("src/tests/test_day{day}.rs"));
        let _ = std::fs::File::create(format!("full/input{day}.txt"));
        let _ = std::fs::File::create(format!("demo_p1/demo{day}.txt"));
    }
}
