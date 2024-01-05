
// Low pulse signal is sent to the broadcaster which repeats the same pulse to all its destinations
pub fn pulse1(lines: Vec<String>) -> usize {
    let (modules, start_modules) = parse_input(lines);
    println!("Start modules {:?}", start_modules);
    display_modules(&modules);
    for module in modules {

    }
    0
}

fn display_modules(modules: &Vec<Module>) {
    for module in modules {
        match module {
            Module::FlipFlop(flip) => {
                println!("%{} -> {:?}", flip.name, flip.destinations);
            }
            Module::Conjunction(conj) => {
                println!("&{} -> {:?}", conj.name, conj.destinations);
            }
        }
    }
}

fn parse_input(lines: Vec<String>) -> (Vec<Module>, Vec<String>) {
    let mut starting_names = vec![];
    for line in &lines {
        if !line.starts_with('b') {
            continue
        }
        let destinations_raw = &line[15..line.len()];
        starting_names = parse_names(destinations_raw);
        break;
    }
    let mut modules = vec![];
    for line in lines {
        let whitespace = line.find(' ').unwrap();
        let name = line[1..whitespace].to_string();
        let destinations_raw = &line[whitespace+4..line.len()];
        let dest_names = parse_names(destinations_raw);

        let module = if line.starts_with('%') {
            let flip_flop = FlipFlop::new(name, dest_names);
            Module::FlipFlop(flip_flop)
        } else if line.starts_with('&') {
            let conjunction = Conjunction::new(name, dest_names);
            Module::Conjunction(conjunction)
        } else {
            continue
        };
        modules.push(module);
    }
    (modules, starting_names)
}

fn parse_names(destinations_raw: &str) -> Vec<String> {
    if destinations_raw.len() == 1 {
        return vec![destinations_raw[0..1].to_string()]
    }
    let mut destinations = vec![];
    let mut i = 0;
    let bytes = destinations_raw.as_bytes();
    let length = destinations_raw.len();
    while i < length - 1 {
        let mut end = i + 1;
        while end < length && bytes[end] != b',' {
            end += 1;
        }
        let dest_name = destinations_raw[i..end].to_string();
        destinations.push(dest_name);
        i = end + 2;
    }
    destinations
}

// % react only to low pulses
// if it was off - turns on and sends a high pulse
// if it was on - turns off and sends a low pulse
pub struct FlipFlop {
    pub name: String,
    pub on: bool,
    pub destinations: Vec<String>
}
impl FlipFlop {
    pub fn new(name: String, destinations: Vec<String>) -> Self {
        Self { name, on: false, destinations }
    }
    pub fn send(&mut self) -> Pulse {
        if self.on {
            self.on = false;
            return Pulse::Low;
        }
        self.on = true;
        Pulse::High
    }
}
// & remember the most recent pulse
pub struct Conjunction {
    pub name: String,
    pub last_pulse: Pulse,
    pub destinations: Vec<String>
}
impl Conjunction {
    pub fn new(name: String, destinations: Vec<String>) -> Self {
        Self { name, last_pulse: Pulse::Low, destinations }
    }
}
pub enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}
enum Pulse {
    Low, High
}