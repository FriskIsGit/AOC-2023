use std::collections::HashMap;

// Low pulse signal is sent to the broadcaster which repeats the same pulse to all its destinations
const BUTTON_PUSHES: usize = 1000;
pub fn pulse1(lines: Vec<String>) -> usize {
    let (mut modules, start_modules) = parse_input(lines);
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for i in 0..BUTTON_PUSHES {
        low_pulses += 1; // low pulse from the button module to the broadcaster
        for name in &start_modules {
            let index = get_module_index(name, &modules);
            let receiver = &mut modules[index.unwrap()];
            match receiver {
                Module::FlipFlop(ref mut flip) => {
                    low_pulses += 1;
                    flip.receive(Pulse::Low);
                }
                _ => panic!("Broadcaster always broadcasts to flip flops")
            }
        }

        let mut sender_modules = start_modules.clone();
        let mut receivers = vec![];
        loop {
            for name in sender_modules.iter() {
                let index = get_module_index(name, &modules);
                let (name, pulse_to_send, destinations) =
                    match &mut modules[index.unwrap()] {
                    Module::FlipFlop(flip) => {
                        let Some(pulse) = flip.pulse() else {
                            continue;
                        };
                        (flip.name.to_owned(), pulse, flip.destinations.to_owned())
                    }
                    Module::Conjunction(conj) => {
                        (conj.name.to_owned(), conj.pulse(), conj.destinations.to_owned())
                    }
                };

                for dest in &destinations {
                    // println!("{name} {:?}-> {}", pulse_to_send, dest);
                    match pulse_to_send {
                        Pulse::Low => low_pulses += 1,
                        Pulse::High => high_pulses += 1
                    }
                    let Some(dest_index) = get_module_index(dest, &modules) else {
                        continue
                    };
                    match &mut modules[dest_index] {
                        Module::FlipFlop(flip) =>
                            flip.receive(pulse_to_send),
                        Module::Conjunction(conjunction) =>
                            conjunction.receive(&name, pulse_to_send)
                    }
                    // cannot extend_from_slice as some destinations don't exist
                    receivers.push(dest.to_owned());
                }
            }

            if receivers.len() == 0 {
                println!("No more receivers");
                break;
            }
            std::mem::swap(&mut receivers, &mut sender_modules);
            receivers.clear();
        }
        // display_modules(&modules);
    }
    low_pulses * high_pulses
}

fn get_module_index(name: &String, modules: &Vec<Module>) -> Option<usize> {
    for (i, module) in modules.iter().enumerate() {
        match module {
            Module::FlipFlop(flip) => {
                if flip.name == *name {
                    return Some(i)
                }
            }
            Module::Conjunction(conj) => {
                if conj.name == *name {
                    return Some(i)
                }
            }
        }
    }
    return None
}

fn get_module_index_whose_dest_is(dest: &String, modules: &Vec<Module>) -> Option<usize> {
    for (i, module) in modules.iter().enumerate() {
        let destinations = match module {
            Module::FlipFlop(flip) => &flip.destinations,
            Module::Conjunction(conj) => &conj.destinations
        };
        for d in destinations {
            if d == dest {
                return Some(i);
            }
        }
    }
    return None
}

fn get_module_name(module: &Module) -> String {
    return match module {
        Module::FlipFlop(flip) => flip.name.clone(),
        Module::Conjunction(conj) => conj.name.clone()
    }
}

fn display_modules(modules: &Vec<Module>) {
    for module in modules {
        match module {
            Module::FlipFlop(flip) => {
                println!("%{} -> {:?} on:{} next_send:{:?}", flip.name, flip.destinations,
                         flip.on, flip.send);
            }
            Module::Conjunction(conj) => {
                println!("&{} -> {:?} received:{:?} {:?}",
                         conj.name, conj.destinations,
                         conj.input_modules, conj.input_pulses);
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
    let mut senders = Vec::with_capacity(lines.len());
    let mut receivers = Vec::with_capacity(lines.len());
    for line in lines {
        let whitespace = line.find(' ').unwrap();
        let name = line[1..whitespace].to_string();
        let destinations_raw = &line[whitespace+4..line.len()];
        let dest_names = parse_names(destinations_raw);
        senders.push(name.clone());
        receivers.push(dest_names.clone());
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
    for module in modules.iter_mut() {
        let Module::Conjunction(conjunction) = module else {
            continue
        };
        // in -> out, out, out
        for (i, out) in receivers.iter().enumerate() {
            if out.contains(&conjunction.name) {
                conjunction.add_input(senders[i].clone())
            }
        }

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
    while i < length {
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
// if it was off - turns on and sends a high piulse
// if it was on - turns off and sends a low pulse
pub struct FlipFlop {
    pub name: String,
    pub destinations: Vec<String>,
    on: bool,
    send: Option<Pulse>,
}
impl FlipFlop {
    pub fn new(name: String, destinations: Vec<String>) -> Self {
        Self { name, destinations, on: false, send: None }
    }
    pub fn receive(&mut self, pulse: Pulse) {
        if pulse == Pulse::High {
            return;
        }
        if self.on {
            self.on = false;
            self.send = Some(Pulse::Low);
            return;
        }
        self.on = true;
        self.send = Some(Pulse::High)
    }
    pub fn pulse(&mut self) -> Option<Pulse> {
        std::mem::take(&mut self.send)
    }
}
// & remember the most recent pulse for each input
// if remembers high pulses for all inputs then it sends a low pulse
pub struct Conjunction {
    pub name: String,
    pub destinations: Vec<String>,
    // inputs need to be filled ahead of time (not stated in the problem)
    input_modules: Vec<String>,
    input_pulses: Vec<Pulse>
}
impl Conjunction {
    pub fn new(name: String, destinations: Vec<String>) -> Self {
        Self { name, destinations, input_modules: vec![], input_pulses: vec![] }
    }
    pub fn receive(&mut self, from: &String, pulse: Pulse) {
        let mut mod_index = None;
        for (i, input) in self.input_modules.iter().enumerate() {
            if input == from {
                mod_index = Some(i);
                break;
            }
        }
        if let Some(index) = mod_index {
            self.input_pulses[index] = pulse;
        } else {
            panic!("Should have been filled with inputs");
        }
    }
    // receive() must be called before pulse()
    pub fn pulse(&self) -> Pulse {
        let low = self.input_pulses.iter().find(|pulse| **pulse == Pulse::Low);
        if low.is_none() {
            return Pulse::Low
        }
        Pulse::High
    }
    pub fn add_input(&mut self, name: String) {
        self.input_modules.push(name);
        self.input_pulses.push(Pulse::Low);
    }
}
pub enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}
#[derive(Clone, Debug, PartialEq, Copy)]
enum Pulse {
    Low, High
}

pub fn pulse2(lines: Vec<String>) -> usize {
    let (mut modules, start_modules) = parse_input(lines);
    let rx_sender_index = get_module_index_whose_dest_is(&"rx".to_string(), &modules)
        .expect("Full input must contain rx as output");
    let rx_sender = get_module_name(&modules[rx_sender_index]);
    for presses in 0..usize::MAX {
        for name in &start_modules {
            let index = get_module_index(name, &modules);
            let receiver = &mut modules[index.unwrap()];
            match receiver {
                Module::FlipFlop(ref mut flip) => {
                    flip.receive(Pulse::Low);
                }
                _ => panic!("Broadcaster always broadcasts to flip flops")
            }
        }

        let mut sender_modules = start_modules.clone();
        let mut receivers = vec![];
        loop {
            for name in sender_modules.iter() {
                let index = get_module_index(name, &modules);
                let (name, pulse_to_send, destinations) =
                    match &mut modules[index.unwrap()] {
                        Module::FlipFlop(flip) => {
                            let Some(pulse) = flip.pulse() else {
                                continue;
                            };
                            (flip.name.to_owned(), pulse, flip.destinations.to_owned())
                        }
                        Module::Conjunction(conj) => {
                            (conj.name.to_owned(), conj.pulse(), conj.destinations.to_owned())
                        }
                    };

                for dest in &destinations {
                    // println!("{name} {:?}-> {}", pulse_to_send, dest);
                    if dest == "rx" && pulse_to_send == Pulse::Low {
                        return presses;
                    }
                    let Some(dest_index) = get_module_index(dest, &modules) else {
                        continue
                    };
                    match &mut modules[dest_index] {
                        Module::FlipFlop(flip) => {
                            flip.receive(pulse_to_send);
                        }
                        Module::Conjunction(conjunction) => {
                            conjunction.receive(&name, pulse_to_send);
                        }
                    }
                    // cannot extend_from_slice as some destinations don't exist
                    receivers.push(dest.to_owned());
                }
            }

            if receivers.len() == 0 {
                break;
            }
            std::mem::swap(&mut receivers, &mut sender_modules);
            receivers.clear();
        }
        // display_modules(&modules);
    }
    panic!("Not found..?");
}