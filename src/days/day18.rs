use std::cmp::max;
use std::fmt::{Display, Formatter};
use crate::days::day18::Intersection::{Horizontal, Vertical};

// The same blocks are never mined twice - predicate that makes most of these methods valid
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
    print_map(&space);
    interior + all_digs
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
                let row_above = &space[r - 1];
                let row_below = &space[r + 1];
                if previous == b'.' {
                    if row_above[col] == b'#' && row_below[col] == b'#' {
                        // simple wall intersection
                        intersections += 1;
                    } else if row_above[col] == b'#' && row_below[col] == b'.' {
                        // no intersection happening
                    } else if row_above[col] == b'.' && row_below[col] == b'#' {
                        intersections += 1;
                    }
                } else {
                    // previous = '#' = current; looking for: _| ‾|
                    if row_above[col] == b'#' && row_below[col] == b'.' {
                        // no intersection happening
                    } else if row_above[col] == b'.' && row_below[col] == b'#' {
                        intersections += 1;
                    }
                }
            }
            previous = current;
        }
        cubic_meters += inners;
    }
    cubic_meters
}

pub fn parse_input(lines: Vec<String>, colors_as_instructions: bool) -> Vec<Dig> {
    let mut digs = vec![];
    for line in lines {
        let num_color_slice = &line[2..line.len()];
        let num_end = num_color_slice.find(' ').unwrap();
        if colors_as_instructions {
            let color = &num_color_slice[num_end + 3..num_color_slice.len() - 1];
            let meters = usize::from_str_radix(&color[0..color.len() - 1], 16).unwrap();
            let dir = match color.bytes().rev().next().unwrap() {
                b'0' => Direction::Right,
                b'1' => Direction::Down,
                b'2' => Direction::Left,
                b'3' => Direction::Up,
                _ => panic!("Unknown dig")
            };
            let dig = Dig::new(dir, meters);
            digs.push(dig);
            continue;
        }
        let dir = match line.bytes().next().unwrap() {
            b'U' => Direction::Up,
            b'D' => Direction::Down,
            b'L' => Direction::Left,
            b'R' => Direction::Right,
            _ => panic!("Unknown dig")
        };
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
    pub fn height(&self) -> usize {
        self.up + self.down
    }
    pub fn width(&self) -> usize {
        self.left + self.right
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

const PARTITIONS: usize = 200;
// TODO Improvements:
pub fn lagoon2(lines: Vec<String>) -> usize {
    let digs = parse_input(lines, true);
    let mut bounds = DigBounds::from(&digs);
    let all_digs: usize = digs.iter().map(|dig| dig.meters).sum();
    println!("ALL DIGS {all_digs}");
    let vertices = get_vertices(&digs, bounds.origin());
    let segments = get_line_segments(&vertices);
    for seg in &segments {
        println!("{seg}");
    }
    let height = bounds.height();
    let width = bounds.width();
    let partition_size = height / PARTITIONS;
    println!("height:{height} width:{width}");
    println!("partition_size:{partition_size}");
    println!("segments.len():{}", segments.len());
    let mut sections: Vec<Vec<&LineSegment>> = vec![vec![]; PARTITIONS+1];
    println!("PARTITIONS:{PARTITIONS}; sections.len(): {}", sections.len());
    let mut sect_st = 0;
    // Assign segments to sections
    for section in sections.iter_mut() {
        let sect_end = sect_st + partition_size;
        for segment in &segments {
            let dir = segment.direction();
            match dir {
                Direction::Up => {
                    if segment.start.y <= sect_st && sect_end <= segment.end.y {
                        section.push(segment); // overlapping case
                    }
                    else if sect_st <= segment.end.y && segment.end.y <= sect_end {
                        section.push(segment); // end within section
                    }
                    else if sect_st <= segment.start.y && segment.start.y <= sect_end {
                        section.push(segment); // start within section
                    }
                }
                Direction::Down => {
                    if segment.end.y <= sect_st && sect_end <= segment.start.y {
                        section.push(segment); // overlapping case
                    }
                    else if sect_st <= segment.end.y && segment.end.y <= sect_end {
                        section.push(segment); // end within section
                    }
                    else if sect_st <= segment.start.y && segment.start.y <= sect_end {
                        section.push(segment); // start within section
                    }
                }
                Direction::Left | Direction::Right => {
                    if sect_st <= segment.start.y && segment.start.y <= sect_end {
                        section.push(segment);
                    }
                }
            }
        }
        sect_st = sect_end;
    }
    // Sort segments from left to right in every section ascendingly
    for section in sections.iter_mut() {
        section.sort_unstable_by(|seg1, seg2| {
            seg1.start.x.cmp(&seg2.start.x)
        });
    }

    // Find intersections and distances between inner walls
    let mut intersections: Vec<Intersection> = Vec::with_capacity(32);
    let mut interior = 0;
    for y in 0..height {
        let sect_index = y/partition_size;
        let section = &sections[sect_index];

        for segment in section {
            let dir = segment.direction();
            match dir {
                Direction::Up => {
                    if segment.start.y < y && y < segment.end.y {
                        intersections.push(Vertical(segment.start.x));
                    }
                }
                Direction::Down => {
                    if segment.end.y < y && y < segment.start.y {
                        intersections.push(Vertical(segment.start.x));
                    }
                }
                Direction::Right => {
                    if y != segment.start.y {
                        continue
                    }
                    let mut cross = CrossIntersect::new();
                    for conn_seg in &segments {
                        match conn_seg.direction() {
                            Direction::Up => {
                                if conn_seg.end.x == segment.start.x {
                                    cross.set_bot_left();
                                    continue
                                }
                                if conn_seg.start.x == segment.end.x {
                                    cross.set_top_right();
                                    continue
                                }
                            }
                            Direction::Down => {
                                if conn_seg.start.x == segment.end.x {
                                    cross.set_bot_right();
                                    continue
                                }
                                if conn_seg.end.x == segment.start.x {
                                    cross.set_top_left();
                                    continue
                                }
                            }
                            _ => continue
                        }
                    }
                    intersections.push(Horizontal(segment.start.x, segment.end.x, cross.intersects_once()));
                }
                Direction::Left => {
                    if y != segment.start.y {
                        continue
                    }
                    let mut cross = CrossIntersect::new();
                    for conn_seg in &segments {
                        match conn_seg.direction() {
                            Direction::Up => {
                                if conn_seg.start.x == segment.start.x {
                                    cross.set_bot_right();
                                    continue
                                }
                                if conn_seg.start.x == segment.end.x {
                                    cross.set_top_left();
                                    continue
                                }
                            }
                            Direction::Down => {
                                if conn_seg.end.x == segment.start.x {
                                    cross.set_top_right();
                                    continue
                                }
                                if conn_seg.start.x == segment.end.x {
                                    cross.set_bot_left();
                                    continue
                                }
                            }
                            _ => continue
                        }
                    }
                    intersections.push(Horizontal(segment.end.x, segment.start.x, cross.intersects_once()));
                }
            }
        }
        if intersections.len() < 2 {
            intersections.clear();
            continue
        }
        let mut intersect_counter = 0;
        for i in 0..intersections.len() - 1 {
            let intersect_left = &intersections[i];
            let mut left_x = 0;
            match intersect_left {
                Horizontal(_, end_x, intersects) => {
                    left_x = *end_x;
                    if *intersects {
                        intersect_counter += 1;
                    }
                }
                Vertical(x) => {
                    left_x = *x;
                    intersect_counter += 1;
                }
            }
            let intersect_right = &intersections[i+1];
            let mut right_x = 0;
            match intersect_right {
                Horizontal(start_x, _, _) => {
                    right_x = *start_x;
                }
                Vertical(x) => {
                    right_x = *x;
                }
            }
            let mut length = 0;
            if intersect_counter % 2 == 1 {
                let x_length = right_x - left_x - 1;
                length += x_length;
            }
            interior += length;
        }
        intersections.clear();
    }
    println!("Interior:{interior}");
    interior + all_digs
}

fn get_line_segments(points: &Vec<Point>) -> Vec<LineSegment> {
    let mut sections = Vec::with_capacity(points.len());
    for i in 0..points.len()-1 {
        let point1 = points[i].clone();
        let point2 = points[i+1].clone();
        let line_section = LineSegment::new(point1, point2);
        sections.push(line_section);
    }
    sections.push(LineSegment::new(points[points.len()-1].clone(), points[0].clone()));
    sections
}

fn get_vertices(digs: &Vec<Dig>, origin: (X, Y)) -> Vec<Point> {
    let (mut x, mut y) = origin;
    println!("Origin: y={y}, x={x}");
    let mut points = Vec::with_capacity(digs.len());
    for dig in digs {
        match dig.direction {
            Direction::Up => {
                y += dig.meters;
            }
            Direction::Down => {
                y -= dig.meters;
            }
            Direction::Left => {
                x -= dig.meters;
            }
            Direction::Right => {
                x += dig.meters;
            }
        }
        points.push(Point::new(x, y));
    }
    points
}

#[derive(Clone, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize
}
impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = format!("({}, {})", self.x, self.y);
        f.write_str(&str)
    }
}
#[derive(Debug)]
pub struct LineSegment {
    start: Point,
    end: Point,
    direction: Direction
}
impl LineSegment {
    pub fn new(start: Point, end: Point) -> Self {
        let direction = Self::determine_direction(&start, &end);
        Self { start, end, direction }
    }
    pub fn distance(&self) -> usize {
        let start = &self.start;
        let end = &self.end;
        if start.x == end.x {
            if start.y < end.y {
                return end.y - start.y;
            }
            return start.y - end.y;
        } else if start.y == end.y {
            if start.x < end.x {
                return end.x - start.x;
            }
            return start.x - end.x;
        }
        panic!("Can't have unaligned points");
    }
    pub fn determine_direction(start: &Point, end: &Point) -> Direction {
        // vertical
        if start.x == end.x {
            if start.y < end.y {
                return Direction::Up;
            }
            return Direction::Down;
        }
        else if start.y == end.y {
            if start.x < end.x {
                return Direction::Right;
            }
            return Direction::Left;
        }
        panic!("Can't have unaligned points");
    }
    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
impl Display for LineSegment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let start = &self.start;
        let end = &self.end;
        let str = format!("({}, {})->({}, {})", start.x, start.y, end.x, end.y);
        f.write_str(&str)
    }
}

pub struct CrossIntersect {
    arr: [bool; 4]
}
impl CrossIntersect {
    pub fn new() -> Self {
        Self { arr: [false; 4] }
    }
    pub fn set_top_left(&mut self) { self.arr[0] = true; }
    pub fn set_top_right(&mut self) { self.arr[1] = true; }
    pub fn set_bot_left(&mut self) { self.arr[2] = true; }
    pub fn set_bot_right(&mut self) { self.arr[3] = true; }
    pub fn intersects_once(&self) -> bool {
        if self.arr[0] && self.arr[3] {
            return true
        }
        return self.arr[1] && self.arr[2]
    }
}

enum Intersection {
    Horizontal(X, X, bool), Vertical(X)
}