use std::cmp::max;

pub fn lagoon1(lines: Vec<String>) -> usize {
    let digs = parse_input(lines, false);
    let mut bounds = DigBounds::from(&digs);
    println!("L:{} R:{} U:{} D:{}", bounds.left, bounds.right, bounds.up, bounds.down);
    let all_digs: usize = digs.iter().map(|dig| dig.meters).sum();
    println!("ALL DIGS {all_digs}");
    let (mut x, mut y) = bounds.origin();
    println!("Origin: y={y}, x={x}");
    let mut space: Vec<Vec<u8>> = vec![vec![b'.'; bounds.left + bounds.right]; bounds.up + bounds.down];
    for dig in digs {
        match dig.direction {
            Direction::Up => {
                for _ in 0..dig.meters {
                    y += 1;
                    space[y][x] = b'#';
                }
            }
            Direction::Down => {
                for _ in 0..dig.meters {
                    y -= 1;
                    space[y][x] = b'#';
                }
            }
            Direction::Left => {
                for _ in 0..dig.meters {
                    x -= 1;
                    space[y][x] = b'#';
                }
            }
            Direction::Right => {
                for _ in 0..dig.meters {
                    x += 1;
                    space[y][x] = b'#';
                }
            }
        }
    }

    let interior = interior_size(&space);
    let mut cubic_meters = interior;
    for row in &space {
        for cube in row {
            if *cube == b'#' {
                cubic_meters += 1
            }
        }
    }
    print_map(&space);
    cubic_meters
}

fn interior_size(space: &Vec<Vec<u8>>) -> usize {
    let mut cubic_meters = 0;
    for (r, row) in space.iter().enumerate() {
        let mut inners = 0;
        // Intersections determined as if the hashtags were slightly raised
        let mut intersections = 0;
        let mut previous = b'.';
        for col in 0..row.len() {
            let current = row[col];
            if current == b'.' && intersections % 2 == 1 {
                inners += 1;
            } else if current == b'#' {
                let row_above = &space[r-1];
                let row_below = &space[r+1];
                if previous == b'.' {
                    if row_above[col] == b'#' && row_below[col] == b'#' {
                        // simple wall intersection
                        intersections += 1;
                    }
                    else if row_above[col] == b'#' && row_below[col] == b'.' {
                        // no intersection happening
                    }
                    else if row_above[col] == b'.' && row_below[col] == b'#' {
                        intersections += 1;
                    }
                }
                else {
                    // previous = '#' = current; looking for: _| ‾|
                    if row_above[col] == b'#' && row_below[col] == b'.' {
                        // no intersection happening
                    }
                    else if row_above[col] == b'.' && row_below[col] == b'#' {
                        intersections += 1;
                    }
                }
            }
            previous = current;
        }
        println!("Inners: {inners}");
        cubic_meters += inners;
    }
    cubic_meters
}

pub fn parse_input(lines: Vec<String>, colors_as_instructions: bool) -> Vec<Dig> {
    let mut digs = vec![];
    for line in lines {
        let dir = match line.bytes().next().unwrap() {
            b'U' => Direction::Up,
            b'D' => Direction::Down,
            b'L' => Direction::Left,
            b'R' => Direction::Right,
            _ => panic!("Unknown dig")
        };
        let num_color_slice = &line[2..line.len()];
        let num_end = num_color_slice.find(' ').unwrap();
        if colors_as_instructions {
            let color_slice = &num_color_slice[num_end+3..num_color_slice.len()-1];
            // TODO
            let dig = Dig::new(dir, 0);
            digs.push(dig);
            continue
        }
        let meters = num_color_slice[0..num_end].parse::<usize>().unwrap();
        let dig = Dig::new(dir, meters);
        digs.push(dig);
    }
    digs
}

pub struct Dig {
    direction: Direction,
    meters: usize,
}

/*
    y ^
      |
      |
      * — — > x
 (0, 0)
*/
type X = usize;
type Y = usize;

#[derive(Default)]
pub struct DigBounds {
    left: usize,
    right: usize,
    up: usize,
    down: usize,
}

impl DigBounds {
    // Optimized bounds
    pub fn from(digs: &Vec<Dig>) -> Self {
        let mut bounds = DigBounds::default();
        let mut x_axis: isize = 0;
        let mut y_axis: isize = 0;
        for dig in digs {
            match dig.direction {
                Direction::Up => {
                    y_axis += dig.meters as isize;
                    if y_axis > 0 {
                        bounds.up = max(y_axis as usize, bounds.up);
                    }
                }
                Direction::Down => {
                    y_axis -= dig.meters as isize;
                    if y_axis < 0 {
                        bounds.down = max(-y_axis as usize, bounds.down);
                    }
                }
                Direction::Left => {
                    x_axis -= dig.meters as isize;
                    if x_axis < 0 {
                        bounds.left = max(-x_axis as usize, bounds.left);
                    }
                }
                Direction::Right => {
                    x_axis += dig.meters as isize;
                    if x_axis > 0 {
                        bounds.right = max(x_axis as usize, bounds.right);
                    }
                }
            }
        }
        bounds.extend_by(2); // gets rid of bound checks
        bounds
    }
    pub fn extend_by(&mut self, extension: usize) {
        self.left += extension;
        self.right += extension;
        self.up += extension;
        self.down += extension;
    }
    // movement center so that all moves stay within bounds
    pub fn origin(&self) -> (X, Y) {
        (self.left, self.down)
    }
}

impl Dig {
    pub fn new(direction: Direction, meters: usize) -> Self {
        Self { direction, meters }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn print_map(map: &Vec<Vec<u8>>) {
    for row in map {
        for val in row {
            print!("{}", char::from(*val));
        }
        println!();
    }
}

pub fn lagoon2(lines: Vec<String>) -> usize {
    let digs = parse_input(lines, true);
    0
}