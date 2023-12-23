use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};

pub fn crucible1(lines: Vec<String>) -> usize {
    let map = parse_input(lines);
    let end = Point::new(map.len() - 1, map[0].len() - 1);
    let mut crucible = Crucible::new(map);
    let start_pos = &crucible.pos.clone();
    // Set start estimate
    crucible.path_points[start_pos.row][start_pos.col].update_estimate(start_pos.clone(), 0);
    loop {
        let current = &crucible.pos;
        println!("=========CURRENT: {current}");
        // Mark current position as explored
        crucible.path_points[current.row][current.col].was_visited = true;

        // Update estimates of immediate UNVISITED neighbors
        let current_distance = crucible.path_points[current.row][current.col].lowest_heat_loss;
        let neighbors = crucible.unvisited_neighbors();

        for neighbor in &neighbors {
            let path_point = &mut crucible.path_points[neighbor.row][neighbor.col];
            let tile_cost = crucible.map[neighbor.row][neighbor.col] as usize;
            path_point.update_estimate(current.clone(), current_distance + tile_cost)
        }
        // Find candidates (later include movement context which would disallow a certain movement)
        crucible.add_new_candidates();
        // Pick candidate by min distance
        let best_candidate = crucible.candidates.iter().min_by_key(|x| {
            crucible.path_points[x.row][x.col].lowest_heat_loss
        });
        let Some(candidate) = best_candidate else {
            println!("Exited due to lack of candidates");
            break;
        };
        let next_point = candidate.to_owned();
        crucible.candidates.remove(&next_point);
        print_path_map(&crucible.map, &crucible.path_points, crucible.pos.clone());
        let backtrack = crucible.back_track(crucible.pos.clone(), 4);
        let invalid = crucible.invalid_direction(&backtrack);
        println!("^^^ backtrack for map: {:?}", backtrack);
        println!("^^^ invalid dir for map: {:?}", invalid);
        println!("^^^ neighbors for  map: {:?}", neighbors);
        // Take the next_point
        crucible.pos = next_point;
    }
    let end_point = &crucible.path_points[end.row][end.col];
    println!("end_point loss: {:?}", end_point.lowest_heat_loss);
    print_path_map(&crucible.map, &crucible.path_points, Point::new(end.row, end.col));
    println!("====================");
    print_visited_map(&crucible.path_points);
    end_point.lowest_heat_loss
}

// There's a difference between immediate neighbors and candidates
struct Crucible {
    map: Vec<Vec<u8>>,
    path_points: Vec<Vec<PathPoint>>,
    candidates: HashSet<Point>,
    pos: Point,
    direction: Direction,
}

impl Crucible {
    pub fn new(map: Vec<Vec<u8>>) -> Self {
        let path_points = Self::create_path_map(&map);
        Self {
            map,
            path_points,
            candidates: HashSet::new(),
            pos: Point::new(0, 0),
            direction: Direction::None
        }
    }
    fn create_path_map(map: &Vec<Vec<u8>>) -> Vec<Vec<PathPoint>> {
        vec![vec![PathPoint::new(); map[0].len()]; map.len()]
    }
    fn add_new_candidates(&mut self) {
        let pos = &self.pos;
        let map = &self.path_points;
        // retrieve last 4 nodes to match 3 pairs to know if crucible has to take a turn
        let backtrack = self.back_track(pos.clone(), 4);
        let invalid_direction = self.invalid_direction(&backtrack);
        if pos.col > 0 {
            if !map[pos.row][pos.col - 1].was_visited() && invalid_direction != Direction::Left {
                let left = Point::new(pos.row, pos.col - 1);
                if !self.candidates.contains(&left) {
                    self.candidates.insert(left);
                }
            }
        }
        if pos.row > 0 {
            if !map[pos.row - 1][pos.col].was_visited() && invalid_direction != Direction::Up {
                let top = Point::new(pos.row - 1, pos.col);
                if !self.candidates.contains(&top) {
                    self.candidates.insert(top);
                }
            }
        }
        if pos.row + 1 < map.len() {
            if !map[pos.row + 1][pos.col].was_visited() && invalid_direction != Direction::Down {
                let down = Point::new(pos.row + 1, pos.col);
                if !self.candidates.contains(&down) {
                    self.candidates.insert(down);
                }
            }
        }
        if pos.col + 1 < map[0].len() {
            if !map[pos.row][pos.col + 1].was_visited() && invalid_direction != Direction::Right {
                let right = Point::new(pos.row, pos.col + 1);
                if !self.candidates.contains(&right) {
                    self.candidates.insert(right);
                }
            }
        }
    }
    pub fn unvisited_neighbors(&self) -> Vec<Point> {
        let mut neighbors = vec![];
        let pos = &self.pos;
        let map = &self.path_points;
        // retrieve last 4 nodes to match 3 pairs to know if crucible has to take a turn
        let backtrack = self.back_track(pos.clone(), 4);
        let invalid_direction = self.invalid_direction(&backtrack);
        if pos.col > 0 {
            if !map[pos.row][pos.col - 1].was_visited() && invalid_direction != Direction::Left {
                neighbors.push(Point::new(pos.row, pos.col - 1));
            }
        }
        if pos.row > 0 {
            if !map[pos.row - 1][pos.col].was_visited() && invalid_direction != Direction::Up {
                neighbors.push(Point::new(pos.row - 1, pos.col));
            }
        }
        if pos.row + 1 < map.len() {
            if !map[pos.row + 1][pos.col].was_visited() && invalid_direction != Direction::Down {
                neighbors.push(Point::new(pos.row + 1, pos.col));
            }
        }
        if pos.col + 1 < map[0].len() {
            if !map[pos.row][pos.col + 1].was_visited() && invalid_direction != Direction::Right {
                neighbors.push(Point::new(pos.row, pos.col + 1));
            }
        }
        neighbors
    }
    pub fn back_track(&self, point: Point, nodes: usize) -> Vec<Point> {
        let mut count = 0;
        let start = Point::new(0, 0);
        let mut path = vec![];
        let mut backtrack = point;
        while count < nodes {
            if backtrack.is(&start) {
                path.insert(0, backtrack);
                return path;
            }
            path.insert(0, backtrack.clone());
            let pp = &self.path_points[backtrack.row][backtrack.col];
            backtrack = pp.origin.clone().unwrap();
            count += 1;
        }
        path
    }
    pub fn invalid_direction(&self, backtrack: &Vec<Point>) -> Direction {
        if backtrack.len() < 4 {
            return Direction::None;
        }
        let first_dir = Self::determine_dir(&backtrack[0], &backtrack[1]);
        for i in 1..3 {
            let dir = Self::determine_dir(&backtrack[i], &backtrack[i+1]);
            if first_dir != dir {
                return Direction::None;
            }
        }
        first_dir
    }
    fn determine_dir(from: &Point, to: &Point) -> Direction {
        if from.row == to.row {
            if from.col < to.col {
                return Direction::Right;
            }
            return Direction::Left;
        }
        // atp columns should be equal unless from==to
        if from.row < to.row {
            return Direction::Down;
        }
        Direction::Up
    }
}

#[derive(Clone)]
pub struct PathPoint {
    origin: Option<Point>,
    lowest_heat_loss: usize,
    was_visited: bool,
}

impl PathPoint {
    pub fn new() -> Self {
        Self { origin: None, lowest_heat_loss: usize::MAX, was_visited: false }
    }
    pub fn was_visited(&self) -> bool {
        self.was_visited
    }
    // it will check if new_distance is better than current
    pub fn update_estimate(&mut self, from: Point, new_heat_loss: usize) {
        if self.lowest_heat_loss > new_heat_loss {
            self.origin = Some(from);
            self.lowest_heat_loss = new_heat_loss;
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Point {
    row: usize,
    col: usize,
}

impl Point {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
    pub fn is(&self, point: &Point) -> bool {
        self.row == point.row && self.col == point.col
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("[{}, {}]", self.row, self.col))
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("[{}, {}]", self.row, self.col))
    }
}

#[derive(PartialEq, Debug)]
enum Direction {
    Right,
    Left,
    Down,
    Up,
    None,
}

pub fn parse_input(mut lines: Vec<String>) -> Vec<Vec<u8>> {
    let mut map = Vec::with_capacity(lines.len());
    for line in lines.iter_mut() {
        let taken_line = std::mem::take(&mut *line);
        map.push(taken_line.into_bytes());
    }
    // Prepare input
    for row in map.iter_mut() {
        for col in 0..row.len() {
            row[col] -= 48;
        }
    }
    map[0][0] = 0; // remove heat as we won't re-enter it
    map
}

fn print_map(map: &Vec<Vec<u8>>) {
    for row in map {
        for val in row {
            print!("{}", *val);
        }
        println!();
    }
}

fn print_path_map(map: &Vec<Vec<u8>>, path_points: &Vec<Vec<PathPoint>>, mut backtrack: Point) {
    let mut path = vec![];
    let start = Point::new(0, 0);
    let backtrack_from = backtrack.clone();
    while !backtrack.is(&start) {
        path.push(backtrack.clone());
        let pp = &path_points[backtrack.row][backtrack.col];
        let from = pp.origin.as_ref().unwrap();
        backtrack = from.clone();
    }
    for (i, row) in map.iter().enumerate() {
        for (c, val) in row.iter().enumerate() {
            let a_point = &Point::new(i, c);
            if path.contains(a_point) {
                if a_point.is(&backtrack_from) {
                    print!("F");
                    continue
                }
                print!("U");
                continue;
            }
            print!("{}", *val);
        }
        println!();
    }
}
fn print_visited_map(path_points: &Vec<Vec<PathPoint>>) {
    for row in path_points {
        for col in row {
            if col.was_visited {
                print!("V")
            } else {
                print!("N")
            }
        }
        println!()
    }
}