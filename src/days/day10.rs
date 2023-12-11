use std::fmt::{Display, Formatter};

pub fn maze1(lines: Vec<String>) -> usize {
    let pipe_map = parse_input(lines);
    let start_pos = find_starting_point(&pipe_map);
    let routes: Vec<Direction> = get_routes(&pipe_map, &start_pos);
    // Since pipe is one large, continuous loop there should always be exactly 2 starting directions
    assert_eq!(routes.len(), 2);
    let mut current_dir = routes[0].clone();
    let mut position = start_pos.clone();
    let mut distance = 0;
    // Assuming first direction is always valid
    loop {
        match current_dir {
            Direction::Up => position.row -= 1,
            Direction::Down => position.row += 1,
            Direction::Left => position.col -= 1,
            Direction::Right => position.col += 1,
        }
        distance += 1;
        if position.is(&start_pos) {
            break;
        }
        let pipe_type = pipe_map[position.row][position.col];
        current_dir = current_dir.next_direction(pipe_type);
    }
    distance/2
}

pub fn parse_input(mut lines: Vec<String>) -> Vec<Vec<u8>> {
    let mut pipe_map = Vec::with_capacity(lines.len());
    for mut line in lines.iter_mut() {
        let taken_line = std::mem::take(&mut *line);
        pipe_map.push(taken_line.into_bytes());
    }
    pipe_map
}

fn find_starting_point(map: &Vec<Vec<u8>>) -> Point {
    for (r, row) in map.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if *col == b'S' {
                return Point::new(r, c);
            }
        }
    }
    panic!("No starting point found - invalid map!")
}

fn get_routes(map: &Vec<Vec<u8>>, start_pos: &Point) -> Vec<Direction> {
    let mut starting_directions = vec![];
    if start_pos.col > 0 {
        let left_tile = map[start_pos.row][start_pos.col];
        if left_tile == b'-' || left_tile == b'F' || left_tile == b'L' {
            starting_directions.push(Direction::Left);
        }
    }
    if start_pos.col + 1 < map[0].len() {
        let right_tile = map[start_pos.row][start_pos.col + 1];
        if right_tile == b'-' || right_tile == b'J' || right_tile == b'7' {
            starting_directions.push(Direction::Right);
        }
    }
    if start_pos.row > 0 {
        let top_tile = map[start_pos.row-1][start_pos.col];
        if top_tile == b'|' || top_tile == b'F' || top_tile == b'7' {
            starting_directions.push(Direction::Up);
        }
    }
    if start_pos.row + 1 < map.len() {
        let bottom_tile = map[start_pos.row+1][start_pos.col];
        if bottom_tile == b'|' || bottom_tile == b'L' || bottom_tile == b'J' {
            starting_directions.push(Direction::Down);
        }
    }
    starting_directions
}

#[derive(PartialEq, Debug, Clone)]
pub enum Direction {
    Up, Down, Left, Right
}
impl Direction {
    pub fn next_direction(self, byte: u8) -> Self {
        return match byte {
            b'F' => {
                return if Direction::Left == self {
                    Direction::Down
                } else { // if going up
                    Direction::Right
                }
            },
            b'7' => {
                return if Direction::Right == self {
                    Direction::Down
                } else { // if going up
                    Direction::Left
                }
            },
            b'L' => {
                return if Direction::Left == self {
                    Direction::Up
                } else { // if going down
                    Direction::Right
                }
            },
            b'J' => {
                return if Direction::Right == self {
                    Direction::Up
                } else { // if going down
                    Direction::Left
                }
            },
            b'S' => self,
            b'-' => self,
            b'|' => self,
            _ => panic!("Invalid char: {}", char::from(byte)),
        }
    }
}

#[derive(Clone)]
struct Point {
    pub row: usize,
    pub col: usize
}
impl Point {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
    #[allow(dead_code)]
    pub fn is_not(&self, point: &Point) -> bool {
        self.row != point.row || self.col != point.col
    }
    pub fn is(&self, point: &Point) -> bool {
        self.row == point.row && self.col == point.col
    }
}
impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("[{}, {}]", self.row, self.col))
    }
}

pub fn maze2(lines: Vec<String>) -> usize {
    let pipe_map = parse_input(lines);
    let mut enclosed = 0;
    for (r, row) in pipe_map.iter().enumerate() {
        for (c_index, col) in row.iter().enumerate() {
            if *col != b'.' {
                continue
            }
            // Ray casting algorithm
            let mut intersections = 0;
            for scan in c_index+1..row.len() {
                let scanned_el = row[scan];
                match scanned_el {
                    b'.' | b'-' => {},
                    b'|' | b'F' | b'J' | b'7' | b'S' => intersections += 1,
                    _ => {}
                }
            }
            if intersections % 2 == 1 {
                println!("row:{r} at index:{c_index}");
                enclosed += 1;
            }

        }
    }
    enclosed
}