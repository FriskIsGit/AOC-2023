use std::collections::HashMap;
use std::fmt::{Display, Formatter};

// z=0 is perfectly flat ground
pub fn slabs1(lines: Vec<String>) -> usize {
    let mut bricks = parse_input(lines);
    let mut bounds = Bounds::new_from(&bricks);
    bounds.extend_by(1);
    println!("bricks.len(): {} ;bounds ({},{},{})", bricks.len(), bounds.max_x, bounds.max_y, bounds.max_z);
    bricks.sort_by(|brick1, brick2|
        brick1.start.z.partial_cmp(&brick2.start.z).unwrap()
    );
    let mut initial_heights = Vec::with_capacity(bricks.len());
    for brick in &bricks {
        initial_heights.push(brick.start.z);
        println!("{brick}");
    }
    println!("DONE PRINTING");
    let mut comparisons = 0;
    // Settle bricks
    loop {
        let mut drops = 0;
        for i in 0..bricks.len() {
            let mut can_move_down = true;
            for j in 0..bricks.len() {
                if i == j {
                    continue
                }
                let brick1 = &bricks[i];
                let brick2 = &bricks[j];
                comparisons += 1;
                if brick1.is_directly_above(brick2) {
                    can_move_down = false;
                    break;
                }
            }
            if can_move_down {
                let brick = &mut bricks[i];
                if brick.drop_by_one() {
                    drops += 1;
                }
            }
        }
        // println!("Dropped bricks: {drops}");
        if drops == 0 {
            break;
        }
    }
    println!("Comparisons performed: {comparisons}");
    // Some bricks fall as many as 155 on the Z axis
    /*for (i, brick) in bricks.iter().enumerate() {
        let h = initial_heights[i];
        println!("{}", h - brick.start.z)
    }*/
    // Determine which bricks lay on top of which
    let mut below_to_above: HashMap<usize, Vec<usize>> = HashMap::with_capacity(bricks.len());
    let mut above_to_below: HashMap<usize, Vec<usize>> = HashMap::with_capacity(bricks.len());

    for a in 0..bricks.len() {
        for b in 0..bricks.len() {
            if a == b {
                continue
            }
            if bricks[a].is_directly_above(&bricks[b]) {
                if let Some(above_vec) = below_to_above.get_mut(&b) {
                    above_vec.push(a);
                } else {
                    below_to_above.insert(b, vec![a]);
                }
                if let Some(below_vec) = above_to_below.get_mut(&a) {
                    below_vec.push(b);
                } else {
                    above_to_below.insert(a, vec![b]);
                }
            }
        }
    }
    // How many bricks can be disintegrated
    let mut removable = 0;
    for (main_key, above_vec) in &below_to_above {
        if above_vec.len() == 0 {
            println!("nothing on top");
            removable += 1;
            continue
        }
        let mut supported_elsewhere = true;
        for above_el in above_vec {
            let Some(level_vec) = above_to_below.get(&above_el) else {
                panic!("Unreachable, must be contained in map.");
            };

            if level_vec.len() == 1 {
                // main_key is the only brick that supports above element
                supported_elsewhere = false;
                break;
            }
        }
        if supported_elsewhere {
            // let label = get_brick_label(&bricks[*main_key]);
            // println!("Removable {} = {label}", bricks[*main_key]);
            removable += 1;
            continue
        }
        /*let label = get_brick_label(&bricks[*main_key]);
        println!("Cannot remove {} = {label} because of: ", bricks[*main_key]);
        for above_brick_i in above_vec {
            let above_label = get_brick_label(&bricks[*above_brick_i]);
            println!("{} = {above_label}, ", bricks[*above_brick_i]);
        }
        println!("=================");*/
    }
    println!("removables {removable}");
    // Access: space[x][y][z]
    // let space: Vec<Vec<Vec<u8>>> = vec![vec![vec![b' ';bounds.max_z]; bounds.max_y]; bounds.max_x];
    removable + bricks.len() - below_to_above.len()
}

//remove later
pub fn get_brick_label(brick: &Brick) -> String {
    let start = &brick.start;
    let end = &brick.end;
    let label = if start.x == 1 && start.y == 0 && end.x == 1 && end.y == 2 {
        "A"
    } else if start.x == 0 && start.y == 0 && end.x == 2 && end.y == 0 {
        "B"
    } else if start.x == 0 && start.y == 2 && end.x == 2 && end.y == 2 {
        "C"
    } else if start.x == 0 && start.y == 0 && end.x == 0 && end.y == 2 {
        "D"
    } else if start.x == 2 && start.y == 0 && end.x == 2 && end.y == 2 {
        "E"
    } else if start.x == 0 && start.y == 1 && end.x == 2 && end.y == 1 {
        "F"
    } else if start.x == 1 && start.y == 1 && end.x == 1 && end.y == 1 {
        "G"
    } else {
        panic!("OK MAN");
    };
    label.into()
}

pub fn parse_input(lines: Vec<String>) -> Vec<Brick> {
    let mut slabs = Vec::with_capacity(lines.len());
    for line in &lines {
        let tilde = line.find('~').unwrap();
        let mut start_str = line[0..tilde].split(',');
        let brick_start = Point::new(
            start_str.next().unwrap().parse::<usize>().unwrap(),
            start_str.next().unwrap().parse::<usize>().unwrap(),
            start_str.next().unwrap().parse::<usize>().unwrap(),
        );
        let mut end_str = line[tilde + 1..].split(',');
        let brick_end = Point::new(
            end_str.next().unwrap().parse::<usize>().unwrap(),
            end_str.next().unwrap().parse::<usize>().unwrap(),
            end_str.next().unwrap().parse::<usize>().unwrap(),
        );
        let slab = Brick::new(brick_start, brick_end);
        slabs.push(slab);
    }
    slabs
}

struct Point {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Point {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("({}, {}, {})", self.x, self.y, self.z))
    }
}

struct Bounds {
    pub max_x: usize,
    pub max_y: usize,
    pub max_z: usize,
}

struct Brick {
    pub start: Point,
    pub end: Point,
    pub length: usize,
    pub kind: Kind
}

impl Brick {
    pub fn new(start: Point, end: Point) -> Self {
        let length = Self::length(&start, &end);
        let kind = if start.z != end.z {
            Kind::Vertical
        } else if start.x != end.x {
            Kind::HorizontalX
        } else if start.y != end.y {
            Kind::HorizontalY
        } else {
            Kind::Singular
        };

        Self { start, end, length, kind }
    }
    pub fn recalculate_length(&mut self) {
        self.length = Self::length(&self.start, &self.end);
    }
    pub fn length(start: &Point, end: &Point) -> usize {
        if start.x != end.x {
            return end.x - start.x + 1;
        }
        if start.y != end.y {
            return end.y - start.y + 1;
        }
        if start.z != end.z {
            return end.z - start.z + 1;
        }
        return 1;
    }

    pub fn drop_by_one(&mut self) -> bool {
        if self.start.z == 0 {
            return false
        }
        self.start.z -= 1;
        self.end.z -= 1;
        true
    }

    pub fn is_directly_above(&self, other: &Brick) -> bool {
        if self.start.z < other.end.z || self.start.z - other.end.z != 1 {
            return false
        }
        match self.kind {
            Kind::HorizontalX => {
                match other.kind {
                    Kind::HorizontalX => {
                        self.start.y == other.start.y &&
                            other.start.x <= self.end.x && self.start.x <= other.end.x
                    }
                    Kind::HorizontalY => {
                        self.start.x <= other.start.x && other.start.x <= self.end.x
                            && other.start.y <= self.start.y && self.start.y <= other.end.y
                    }
                    Kind::Vertical => {
                        self.start.y == other.start.y &&
                            self.start.x <= other.start.x && other.start.x <= self.end.x
                    }
                    Kind::Singular => {
                        self.start.y == other.start.y &&
                            self.start.x <= other.start.x && other.start.x <= self.end.x
                    }
                }
            }
            Kind::HorizontalY => {
                match other.kind {
                    Kind::HorizontalX => {
                        self.start.y <= other.start.y && other.start.y <= self.end.y
                            && other.start.x <= self.start.x && self.start.x <= other.end.x
                    }
                    Kind::HorizontalY => {
                        self.start.x == other.start.x &&
                            other.start.y <= self.end.y && self.start.y <= other.end.y
                    }
                    Kind::Vertical => {
                        self.start.x == other.start.x &&
                            self.start.y <= other.start.y && other.start.y <= self.end.y
                    }
                    Kind::Singular => {
                        self.start.x == other.start.x &&
                            self.start.y <= other.start.y && other.start.y <= self.end.y
                    }
                }
            }
            Kind::Vertical => {
                match other.kind {
                    Kind::HorizontalX => {
                        self.start.y == other.start.y &&
                            other.start.x <= self.start.x && self.start.x <= other.end.x
                    }
                    Kind::HorizontalY => {
                        self.start.x == other.start.x &&
                            other.start.y <= self.start.y && self.start.y <= other.end.y
                    }
                    Kind::Vertical | Kind::Singular => {
                        self.start.x == other.start.x &&
                            self.start.y == other.start.y
                    }
                }
            }
            Kind::Singular => {
                match other.kind {
                    Kind::HorizontalX => {
                        self.start.y == other.start.y &&
                            other.start.x <= self.start.x && self.start.x <= other.end.x
                    }
                    Kind::HorizontalY => {
                        self.start.x == other.start.x &&
                            other.start.y <= self.start.y && self.start.y <= other.end.y
                    }
                    Kind::Vertical | Kind::Singular => {
                        self.start.x == other.start.x &&
                            self.start.y == other.start.y
                    }
                }
            }
        }
    }
}

impl Display for Brick {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}~{} v:{:?}", self.start, self.end, self.kind))
    }
}

impl Bounds {
    pub fn new_from(bricks: &Vec<Brick>) -> Self {
        let mut bounds = Self {
            max_x: 0,
            max_y: 0,
            max_z: 0,
        };
        for brick in bricks {
            let points = vec![&brick.start, &brick.end];
            for point in points {
                bounds.max_x = std::cmp::max(point.x, bounds.max_x);
                bounds.max_y = std::cmp::max(point.y, bounds.max_y);
                bounds.max_z = std::cmp::max(point.z, bounds.max_z);
            }
        }
        bounds
    }
    pub fn extend_by(&mut self, value: usize) {
        self.max_x += value;
        self.max_y += value;
        self.max_z += value;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    HorizontalX,
    HorizontalY,
    Vertical,
    Singular,
}