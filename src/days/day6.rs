
type Milliseconds = usize;
type Millimeters = usize;
struct Race {
    time: Milliseconds,
    best: Millimeters
}
impl Race {
    pub fn new(time: Milliseconds, best: Millimeters) -> Self {
        Self { time, best }
    }
    pub fn get_distance_travelled(&self, wait_time: Milliseconds) -> usize {
        wait_time * self.time - wait_time * wait_time
    }
    // (t-x)*x = -x^2 + tx
    // the key is finding the vertex
    pub fn best_possible(&self) -> usize {
        (self.time * self.time) / 4
    }
}

pub fn boats1(lines: Vec<String>) -> usize {
    let mut races = parse_input(lines);
    let mut factor = 1;
    let mut better = 0;
    for race in races {
        for wait in 1..race.time {
            let distance = race.get_distance_travelled(wait);
            if distance > race.best {
                better += 1;
            }
        }
        factor *= better;
        better = 0;
    }
    factor
}

fn parse_input(lines: Vec<String>) -> Vec<Race> {
    assert_eq!(lines.len(), 2);
    let time_line      = &lines[0];
    let distance_line  = &lines[1];
    let time_colon     = time_line.find(':').unwrap();
    let distance_colon = distance_line.find(':').unwrap();
    let times_vec      = parse_numbers(time_line, time_colon + 1);
    let distances_vec  = parse_numbers(distance_line, distance_colon + 1);
    let mut races = Vec::with_capacity(times_vec.len());
    println!("RACES: {}", times_vec.len());
    for i in 0..times_vec.len() {
        let race = Race::new(times_vec[i], distances_vec[i]);
        races.push(race)
    }
    races
}
fn parse_numbers(line: &str, from: usize) -> Vec<usize> {
    let line = line.bytes().skip(from);
    let mut expect_digit = true;
    let mut current_number: usize = 0;
    let mut numbers = vec![];
    for byte in line {
        match byte {
            b'0'..=b'9' => {
                expect_digit = false;
                let digit = byte - 48;
                current_number = current_number*10 + digit as usize;
            },
            b' ' => {
                if expect_digit {
                    continue
                }
                numbers.push(current_number);
                current_number = 0;
                expect_digit = true
            },
            _ => panic!("No other chars should be here {byte}")
        }
    }
    numbers.push(current_number);
    numbers
}