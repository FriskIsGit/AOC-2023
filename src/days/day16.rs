pub fn beam1(lines: Vec<String>) -> usize {
    let map = parse_input(lines);
    // Too many arguments to pass around so it's better to create a struct
    let mut facility = Facility::new(map);
    facility.cast_beam(Point::new(0, 0), Direction::Right, true);
    print_energized(&facility.energized);
    facility.count_energized()
}

fn print_energized(energized: &Vec<Vec<bool>>) {
    for row in energized {
        for val in row {
            if *val {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub struct Facility {
    pub map: Vec<Vec<u8>>,
    pub energized: Vec<Vec<bool>>,
    pub all_rows: usize,
    pub all_columns: usize,
}
impl Facility {
    pub fn new(map: Vec<Vec<u8>>) -> Self {
        let energized = Self::new_bool_map(&map);
        Self { all_rows: map.len(), all_columns: map[0].len(), map, energized }
    }
    fn cast_beam(&mut self, mut point: Point, mut direction: Direction, mut entry_beam: bool) {
        self.energize(&point);
        loop {
            // Can't start from negative index due to usize type
            if !entry_beam {
                match direction {
                    Direction::Right => {
                        if point.col_index == self.all_columns-1 {
                            return
                        }
                        point.col_index += 1;
                    }
                    Direction::Left => {
                        if point.col_index == 0 {
                            return
                        }
                        point.col_index -= 1;
                    }
                    Direction::Down => {
                        if point.row_index == self.all_rows-1 {
                            return
                        }
                        point.row_index += 1;
                    }
                    Direction::Up => {
                        if point.row_index == 0 {
                            return
                        }
                        point.row_index -= 1;
                    }
                }
            }
            entry_beam = false;
            match self.map[point.row_index][point.col_index] {
                b'|' => {
                    if self.is_energized(&point) {
                        return;
                    }
                    match direction {
                        Direction::Right | Direction::Left => {
                            self.cast_beam(point.clone(), Direction::Up, false);
                            self.cast_beam(point.clone(), Direction::Down, false);
                            return;
                        }
                        Direction::Down | Direction::Up => {}
                    }
                }
                b'-' => {
                    if self.is_energized(&point) {
                        return;
                    }
                    match direction {
                        Direction::Right | Direction::Left => {}
                        Direction::Down | Direction::Up => {
                            self.cast_beam(point.clone(), Direction::Left, false);
                            self.cast_beam(point.clone(), Direction::Right, false);
                            return;
                        }
                    }
                }
                b'/' => {
                    match direction {
                        Direction::Right => direction = Direction::Up,
                        Direction::Left => direction = Direction::Down,
                        Direction::Down => direction = Direction::Left,
                        Direction::Up => direction = Direction::Right,
                    }
                }
                b'\\' => {
                    match direction {
                        Direction::Right => direction = Direction::Down,
                        Direction::Left => direction = Direction::Up,
                        Direction::Down => direction = Direction::Right,
                        Direction::Up => direction = Direction::Left,
                    }
                }
                b'.' => {},
                _ => panic!("What {}", self.map[point.row_index][point.col_index])
            }

            self.energize(&point); // always energize
        }
    }
    fn energize(&mut self, point: &Point) {
        self.energized[point.row_index][point.col_index] = true;
    }
    fn is_energized(&self, point: &Point) -> bool {
        self.energized[point.row_index][point.col_index]
    }
    fn count_energized(&self) -> usize {
        let mut count = 0;
        for row in &self.energized {
            for flag in row {
                if *flag {
                    count += 1;
                }
            }
        }
        count
    }
    fn new_bool_map(map: &Vec<Vec<u8>>) -> Vec<Vec<bool>> {
        vec![vec![false; map[0].len()]; map.len()]
    }
}

#[derive(Clone)]
pub struct Point {
    row_index: usize,
    col_index: usize,
}
impl Point {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row_index: row, col_index: col }
    }
}

enum Direction {
    Right, Left, Down, Up
}

pub fn parse_input(mut lines: Vec<String>) -> Vec<Vec<u8>> {
    let mut map = Vec::with_capacity(lines.len());
    for line in lines.iter_mut() {
        let taken_line = std::mem::take(&mut *line);
        map.push(taken_line.into_bytes());
    }
    map
}

pub fn beam2(lines: Vec<String>) -> usize {
    let map = parse_input(lines);
    let mut max = 0;
    // LEFT => RIGHT
    for row in 0..map.len() {
        let mut facility = Facility::new(map.clone());
        let start_point = Point::new(row, 0);
        facility.cast_beam(start_point, Direction::Right, true);
        max = std::cmp::max(facility.count_energized(), max);
    }
    // LEFT <= RIGHT
    for row in 0..map.len() {
        let mut facility = Facility::new(map.clone());
        let start_point = Point::new(row, facility.all_columns-1);
        facility.cast_beam(start_point, Direction::Left, true);
        max = std::cmp::max(facility.count_energized(), max);
    }
    let columns = map[0].len();
    // v DOWN v
    for starting_column in 0..columns {
        let mut facility = Facility::new(map.clone());
        let start_point = Point::new(0, starting_column);
        facility.cast_beam(start_point, Direction::Down, true);
        max = std::cmp::max(facility.count_energized(), max);
    }
    // ^ UP ^
    for starting_column in 0..columns {
        let mut facility = Facility::new(map.clone());
        let start_point = Point::new(facility.all_rows-1, starting_column);
        facility.cast_beam(start_point, Direction::Up, true);
        max = std::cmp::max(facility.count_energized(), max);
    }
    println!("Most energized: {max}");
    max
}