use std::collections::HashMap;

pub fn wasteland1(mut lines: Vec<String>) -> usize {
    let input = parse_input(lines);
    let (mut moves, network) = input;

    let mut steps_required = 0;
    let mut current_node;
    let mut current_label = &"AAA".to_string();
    loop {
        let mov = moves.next();
        current_node = network.get(current_label).expect("Node mapping missing");
        if mov == b'L' {
            current_label = &current_node.left;
        } else {
            current_label = &current_node.right;
        }
        steps_required += 1;
        if current_label == "ZZZ" {
            break;
        }

    }
    steps_required
}

pub fn parse_input(mut lines: Vec<String>) -> (LoopingIterator, HashMap<String, Node>){
    let instructions = std::mem::take(&mut lines[0]);
    let mut moves = LoopingIterator::new(instructions);
    let mut network: HashMap<String, Node> = HashMap::new();
    for i in 2..lines.len() {
        let line = &lines[i];
        // Hardcoding parsing because labels are always length 3
        let label = line[0..3].to_string();
        let left = line[7..10].to_string();
        let right = line[12..15].to_string();
        // println!("{label} = ({left}, {right})");
        let node = Node::new(left, right);
        network.insert(label, node);
    }
    (moves, network)
}

struct Node {
    left: String,
    right: String,
}

impl Node {
    pub fn new(left: String, right: String) -> Self {
        Self { left, right }
    }
}

// It will loop around its data indefinitely always returning valid elements
struct LoopingIterator {
    i: usize,
    len: usize,
    bytes: Vec<u8>,
}
#[allow(dead_code)]
impl LoopingIterator {
    pub fn new(string: String) -> Self {
        if string.len() < 1 {
            panic!("Data cannot be empty")
        }
        let bytes = string.into_bytes();
        Self {
            i: 0,
            len: bytes.len(),
            bytes,
        }

    }
    pub fn next(&mut self) -> u8 {
        let byte =  self.bytes[self.i];
        self.i += 1;
        if self.i == self.len {
            self.i = 0;
        }
        byte
    }
    pub fn reset(&mut self) {
        self.i = 0;
    }
    pub fn len(&self) -> usize {
        self.len
    }
}

// How many steps does it take before you're only on nodes that end with Z?
pub fn wasteland2(lines: Vec<String>) -> usize {
    let input = parse_input(lines);
    let (mut moves, network) = input;
    let mut ghost_labels = vec![];
    for key in network.keys() {
        if key.ends_with('A') {
            ghost_labels.push(key.clone())
        }
    }
    let mut steps_required = 0;
    let ghosts = ghost_labels.len();
    loop {
        let mov = moves.next();
        for i in 0..ghosts {
            let label= &ghost_labels[i];
            let current_node = network.get(label).expect("Node mapping missing");
            if mov == b'L' {
                ghost_labels[i] = current_node.left.to_owned();
            } else {
                ghost_labels[i] = current_node.right.to_owned();
            }
        }
        steps_required += 1;
        let mut all_done = true;
        for label in &ghost_labels {
            if !label.ends_with('Z') {
                all_done = false;
                break;
            }
        }
        if all_done {
            break;
        }
    }
    steps_required
}