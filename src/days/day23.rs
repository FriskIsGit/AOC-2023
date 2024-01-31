use std::collections::HashSet;
use std::fmt::{Display, Formatter};

pub fn long_walk1(lines: Vec<String>) -> usize {
    let map = read_map(lines);
    print_map(&map);
    let walkable = count_byte(&map, b'.');
    let dijkstra = Dijkstra::new_detect(map);
    println!("start: {} | end: {}", dijkstra.start, dijkstra.end);
    println!("walkable: {walkable}");
    let mut current = dijkstra.start.clone();
    // alternatively run until all nodes are explored?
    while !current.eq(&dijkstra.end) {
        current = dijkstra.end.clone();
    }
    0
}

pub fn read_map(lines: Vec<String>) -> Vec<Vec<u8>> {
    let mut map: Vec<Vec<u8>> = Vec::with_capacity(lines.len());
    for line in lines {
        map.push(line.into_bytes());
    }
    return map;
}

pub fn print_map(map: &Vec<Vec<u8>>) {
    for row in map {
        unsafe {
            let str = std::str::from_utf8_unchecked(row);
            println!("{str}");
        }
    }
}

pub fn print_distance_map(distances: &Vec<Vec<u32>>) {
    let cols_count = distances[0].len();
    let rows_count = distances.len();

    let mut column_width = vec![1; cols_count];
    for col_index in 0..cols_count {
        let mut max_width_in_row = 0;
        for row_index in 0..rows_count {
            let value = distances[row_index][col_index];
            max_width_in_row = std::cmp::max(max_width_in_row, value.length());
        }
        column_width[col_index] = max_width_in_row;
    }
    let mut format = String::with_capacity(distances.len() * distances[0].len());
    for row in distances {
        let mut c_index = 0;
        let mut line = String::new();
        for num in row {
            let width = column_width[c_index];
            let diff = width - num.length();
            line.push_str(&num.to_string());
            for _ in 0..diff + 1 {
                line.push(' ');
            }
            c_index += 1;
        }
        format.push_str(&line);
        format.push('\n');
    }
    println!("{}", format);
}

pub fn count_byte(map: &Vec<Vec<u8>>, byte: u8) -> usize {
    let mut count = 0;
    for row in map {
        for b in row {
            if *b == byte {
                count += 1;
            }
        }
    }
    count
}

trait Length {
    fn length(&self) -> u32;
}

impl Length for u32 {
    fn length(&self) -> u32 {
        self.checked_ilog10().unwrap_or(0) + 1
    }
}

pub struct Dijkstra {
    start: Point,
    end: Point,
    visited: HashSet<u16>, // visited hashes
    map: Vec<Vec<u8>>, // map with bytes: b'#', b'>', b'v', b'.'
    nodes: Vec<Vec<Node>>
}

impl Dijkstra {
    pub fn new(map: Vec<Vec<u8>>, start: Point, end: Point) -> Self {
        let mut nodes = vec![vec![Node::new(); map[0].len()]; map.len()];
        nodes[start.row][start.col].best_distance = 0;
        Self { map, start, end, visited: HashSet::with_capacity(32), nodes }
    }

    pub fn new_detect(map: Vec<Vec<u8>>) -> Self {
        let start = Self::find_start(&map);
        let end = Self::find_end(&map);
        let mut nodes = vec![vec![Node::new(); map[0].len()]; map.len()];
        nodes[start.row][start.col].best_distance = 0;
        Self { map, start, end, visited: HashSet::with_capacity(32), nodes }
    }

    pub fn is_visited(&self, row: u16, col: u16) -> bool {
        let hash = Self::get_hash(row, col);
        self.visited.contains(&hash)
    }

    pub fn mark_visited(&mut self, hash: u16) -> bool {
        self.visited.insert(hash)
    }

    //  row(u8)|col(u8)
    // 00000000|00000000
    pub fn get_hash(row: u16, col: u16) -> u16 {
        let mut hash = col;
        hash |= row << 8;
        hash
    }

    pub fn get_row_col(hash: u16) -> (u16, u16) {
        let col = hash & 0xFF;
        let row = hash >> 8;
        (row, col)
    }

    pub fn find_start(map: &Vec<Vec<u8>>) -> Point {
        let col = &map[0];
        let mut index = 0;
        for byte in col {
            if *byte == b'.' {
                return Point::new(0, index);
            }
            index += 1;
        }
        panic!("Starting point not found!")
    }
    pub fn find_end(map: &Vec<Vec<u8>>) -> Point {
        let last_row = map.len() - 1;
        let col = &map[last_row];
        let mut index = 0;
        for byte in col {
            if *byte == b'.' {
                return Point::new(last_row, index);
            }
            index += 1;
        }
        panic!("End point not found!")
    }
}

type Hash = u16;
#[derive(Clone)]
pub struct Node {
    origin: Option<Hash>,
    best_distance: u32
}

const MAX_DISTANCE: u32 = 9999;
impl Node {
    pub fn new() -> Self {
        Self { origin: None, best_distance: MAX_DISTANCE}
    }

    pub fn get_origin(&self) -> Option<Hash> {
        self.origin
    }

    pub fn set_origin(&mut self, hash: Hash) {
        self.origin = Some(hash);
    }
}

#[derive(Clone)]
pub struct Point {
    row: usize,
    col: usize,
}

impl Point {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
    pub fn eq(&self, point: &Point) -> bool {
        self.row == point.row && self.col == point.col
    }
}

impl Display for Point {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&format!("[{}, {}]", self.row, self.col))
    }
}
