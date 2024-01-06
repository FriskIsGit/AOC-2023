
const STEPS: usize = 64;
pub fn step_counter1(lines: Vec<String>) -> usize {
    let map = parse_input(lines);
    print_map(&map);
    0
}

fn parse_input(mut lines: Vec<String>) -> Vec<Vec<u8>> {
    let mut map: Vec<Vec<u8>> = Vec::with_capacity(lines.len());
    for line in lines.iter_mut() {
        let taken_line = std::mem::take(line);
        map.push(taken_line.into_bytes());
    }
    map
}

fn print_map(map: &Vec<Vec<u8>>) {
    for row in map {
        for val in row {
            print!("{}", char::from(*val));
        }
        println!();
    }
}