use std::fmt::{Display, Formatter};

const MIN_COORDINATE: f64 = 200000000000000f64;
const MAX_COORDINATE: f64 = 400000000000000f64;

// For this part - disregard Z axis entirely
pub fn hailstones1(lines: Vec<String>) -> usize {
    let hailstones = parse_input(lines);
    let lines = create_lines(&hailstones);
    println!("Intersects-----");
    // TODO: recognize direction when determining intersections
    let intersects = find_intersects(&lines);
    0
}

fn create_lines(hails: &Vec<Hailstone>) -> Vec<Line2D> {
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

fn find_intersects(lines: &Vec<Line2D>) -> usize {
    let mut count = 0;
    let line_count = lines.len();
    for i in 0..line_count {
        for j in i+1..line_count {
            let line1 = &lines[i];
            let line2 = &lines[j];
            if let Some(intersect) = line1.intersect(line2) {
                if intersect.x >= MIN_COORDINATE && intersect.x <= MAX_COORDINATE
                && intersect.y >= MIN_COORDINATE && intersect.y <= MAX_COORDINATE {
                    count += 1;
                }
                println!("{intersect}");
            } else {
                println!("No intersect: {line1}, {line2}");
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

pub struct Hailstone {
    pos: Position3D,
    vel: Velocity3D
}
impl Hailstone {
    pub fn new(position: Position3D, velocity: Velocity3D) -> Self {
        Self { pos: position, vel: velocity }
    }
    pub fn transform3d(&mut self, times: usize) {
        self.pos.x += self.vel.x * times as isize;
        self.pos.y += self.vel.y * times as isize;
        self.pos.z += self.vel.z * times as isize;
    }
    pub fn transform2d(&mut self, times: usize) {
        self.pos.x += self.vel.x * times as isize;
        self.pos.y += self.vel.y * times as isize;
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
}
impl Display for Point2D {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&format!("({}, {})", self.x, self.y))
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