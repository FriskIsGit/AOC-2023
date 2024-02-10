use std::fmt::{Display, Formatter};

const DEMO_MIN: f64 = 7f64;
const DEMO_MAX: f64 = 27f64;

const MIN_COORDINATE: f64 = 200000000000000f64;
const MAX_COORDINATE: f64 = 400000000000000f64;

// For this part - disregard Z axis entirely
pub fn hailstones1(lines: Vec<String>) -> usize {
    hailstones1_bounds(lines, MIN_COORDINATE, MAX_COORDINATE)
}
pub fn hailstones_demo1(lines: Vec<String>) -> usize {
    hailstones1_bounds(lines, DEMO_MIN, DEMO_MAX)
}
fn hailstones1_bounds(lines: Vec<String>, min_coordinate: f64, max_coordinate: f64) -> usize {
    let hailstones = parse_input(lines);
    let lines = create_line_trajectories(&hailstones);
    let intersects = find_intersects(&lines, &hailstones, min_coordinate, max_coordinate);
    // println!("Intersects: {intersects}");
    intersects
}

fn create_line_trajectories(hails: &Vec<Hailstone>) -> Vec<Line2D> {
    let mut lines = Vec::with_capacity(hails.len());
    for stone in hails {
        let p1 = Point2D::new_isize(stone.pos.x, stone.pos.y);
        let p2 = Point2D::new(p1.x + stone.vel.x as f64, p1.y + stone.vel.y as f64);

        let slope = (p1.y - p2.y) / (p1.x - p2.x);
        let b = p1.y - slope * p1.x;
        // println!("y = {slope}x + {b}");
        lines.push(Line2D::new(slope, b));
    }
    lines
}

fn create_2d_segment(p1: Point2D, p2: Point2D) -> Segment2D {
    let slope = (p1.y - p2.y) / (p1.x - p2.x);
    let b = p1.y - slope * p1.x;
    let line = Line2D::new(slope, b);
    Segment2D::new(line, p1, p2)
}

fn find_intersects(lines: &[Line2D], hailstones: &[Hailstone], min_coordinate: f64, max_coordinate: f64) -> usize {
    let mut count = 0;
    let line_count = lines.len();
    for i in 0..line_count {
        for j in i+1..line_count {
            let line1 = &lines[i];
            let line2 = &lines[j];

            if let Some(intersect) = line1.intersect(line2) {
                if intersect.x >= min_coordinate && intersect.x <= max_coordinate
                && intersect.y >= min_coordinate && intersect.y <= max_coordinate {
                    if is_approaching(&hailstones[i], &intersect) &&
                        is_approaching(&hailstones[j], &intersect) {
                        count += 1;
                        continue
                    }
                }
                // !("{intersect}");
            } else {
                // println!("No intersect: {line1}, {line2}");
            }
        }
    }
    count
}

// px py pz @ vx vy vz
fn parse_input(lines: Vec<String>) -> Vec<Hailstone> {
    let mut hailstones = vec![];
    for line in lines {
        let at_sign = line.find('@').unwrap();
        let pos_numbers = parse_numbers(&line[0..at_sign-1]);
        let vel_numbers = parse_numbers(&line[at_sign+2..line.len()]);
        let position = Position3D::new(pos_numbers[0], pos_numbers[1], pos_numbers[2]);
        let velocity = Velocity3D::new(vel_numbers[0], vel_numbers[1], vel_numbers[2]);
        hailstones.push(Hailstone::new(position, velocity));
    }
    hailstones
}

fn parse_numbers(slice: &str) -> Vec<isize> {
    let mut numbers = vec![];
    let mut number: isize = 0;
    let mut apply_negative = false;
    for byte in slice.bytes() {
        match byte {
            b'0'..=b'9' => {
                number *= 10;
                number += byte as isize - 48;
            }
            b',' => {
                if apply_negative {
                    numbers.push(-number);
                    apply_negative = false;
                } else {
                    numbers.push(number);
                }
                number = 0;
            }
            b'-' => {
                apply_negative = true;
            }
            _ => {}
        }
    }
    if apply_negative {
        numbers.push(-number);
    } else {
        numbers.push(number);
    }
    numbers
}

type Speed2D = Point2D;
pub fn is_approaching(hailstone: &Hailstone, target: &Point2D) -> bool {
    let origin = Point2D::new(hailstone.pos.x as f64, hailstone.pos.y as f64);
    let initial_distance = target.distance_no_sqrt(&origin);

    let speed = Speed2D::new(hailstone.vel.x as f64, hailstone.vel.y as f64);
    let transform = origin.transform2d(&speed);

    let new_distance = target.distance_no_sqrt(&transform);
    initial_distance > new_distance
}

pub struct Hailstone {
    pos: Position3D,
    vel: Velocity3D
}
impl Hailstone {
    pub fn new(position: Position3D, velocity: Velocity3D) -> Self {
        Self { pos: position, vel: velocity }
    }
    pub fn to_point_2d(&self) -> Point2D {
        Point2D::new_isize(self.pos.x, self.pos.y)
    }
    pub fn at_time_2d(&self, time: usize) -> Point2D {
        let x = self.pos.x + self.vel.x * time as isize;
        let y = self.pos.y + self.vel.y * time as isize;
        Point2D::new_isize(x, y)
    }
}
impl Display for Hailstone {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let pos = &self.pos;
        let v = &self.vel;
        let display = &format!("[{},{},{}] | [{},{},{}]", pos.x, pos.y, pos.z, v.x, v.y, v.z);
        formatter.write_str(display)
    }
}
type Position3D = Point3D;
type Velocity3D = Point3D;
type Vector3D = Point3D;

pub struct Point3D {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}
impl Point3D {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }
}

pub struct Point2D {
    pub x: f64,
    pub y: f64,
}
impl Point2D {
    pub fn new_isize(x: isize, y: isize) -> Self {
        Self { x: x as f64, y: y as f64 }
    }
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn transform2d(&self, transform: &Self) -> Self {
        Point2D::new(self.x + transform.x, self.y + transform.y)
    }
    pub fn distance_no_sqrt(&self, point: &Self) -> f64 {
        let x_diff = self.x - point.x;
        let y_diff = self.y - point.y;
        x_diff * x_diff + y_diff * y_diff
    }
}
impl Display for Point2D {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&format!("({}, {})", self.x, self.y))
    }
}

pub struct Segment2D {
    line: Line2D,
    p1: Point2D,
    p2: Point2D,
}
impl Segment2D {
    pub fn new(line: Line2D, p1: Point2D, p2: Point2D) -> Self {
        Self { line, p1, p2 }
    }
}

pub struct Line2D {
    // y = sx + b
    // 1 = s*-2 + b
    pub s: f64,
    pub b: f64,
}
impl Line2D {
    pub fn new(slope: f64, b: f64) -> Self {
        Self { s: slope, b }
    }
    pub fn intersect(&self, line2: &Self) -> Option<Point2D> {
        let line1 = self;
        if line1.s == line2.s {
            return None
        }
        let x = (line1.b - line2.b) / (line2.s - line1.s);
        let y = line1.s * x + line1.b;
        Some(Point2D::new(x, y))
    }
}
impl Display for Line2D {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&format!("y={}x + {}]", self.s, self.b))
    }
}

const TIME_BOUNDARY: usize = 20; // Complexity = n^2 * TIME_BOUNDARY^2

// Assuming YOU CAN hit every hailstone in a single throw!
pub fn hailstones2(lines: Vec<String>) -> usize {
    let hailstones = parse_input(lines);
    let lines = create_line_trajectories(&hailstones);

    let length = hailstones.len();
    // Cycle through all possible combinations of start and end hailstones (length^2) =~ 90000
    for i in 0..length - 1 {
        let start = &hailstones[i];
        for j in i + 1..length {
            let end = &hailstones[j];
            // Find time t at start and end; Premises: t_start >= 0 && t_end >= t_start + length
            for t_start in 0..TIME_BOUNDARY {
                let max_time = length + t_start + TIME_BOUNDARY;
                for t_end in length+t_start..max_time {
                    let p1 = start.at_time_2d(t_start);
                    let p2 = end.at_time_2d(t_end);
                    let segment = create_2d_segment(p1, p2);
                    // Make sure other hailstones cut through these segments
                    // and that the time of intersection is never repeated twice
                    for h in 0..length {
                        if h == i || h == j {
                            continue
                        }
                    }
                    // If there are is more than one matching line Z axis checks need to be implemented
                }
            }

        }
    }
    0
}