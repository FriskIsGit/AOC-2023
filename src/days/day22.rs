use std::collections::{HashMap, HashSet};
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
    settle_bricks(&mut bricks);

    // Determine which bricks lay on top of which
    let mut below_to_above: HashMap<usize, Vec<usize>> = HashMap::with_capacity(bricks.len());
    let mut above_to_below: HashMap<usize, Vec<usize>> = HashMap::with_capacity(bricks.len());

    for a in 0..bricks.len() {
        for b in 0..bricks.len() {
            if a == b {
                continue;
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
    for (_, above_vec) in &below_to_above {
        let mut supported_elsewhere = true;
        for above_el in above_vec {
            let Some(level_vec) = above_to_below.get(&above_el) else {
                panic!("Unreachable, must be contained in map.");
            };

            if level_vec.len() == 1 {
                supported_elsewhere = false;
                break;
            }
        }
        if supported_elsewhere {
            removable += 1;
            continue;
        }
    }
    println!("removables {removable}");
    // Access: space[x][y][z]
    // let space: Vec<Vec<Vec<u8>>> = vec![vec![vec![b' ';bounds.max_z]; bounds.max_y]; bounds.max_x];
    removable + bricks.len() - below_to_above.len()
}

fn settle_bricks(bricks: &mut Vec<Brick>) {
    let mut comparisons = 0;
    loop {
        let mut drops = 0;
        for i in 0..bricks.len() {
            let mut can_move_down = true;
            for j in 0..bricks.len() {
                if i == j {
                    continue;
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
}

//remove later
pub fn get_demo_brick_label(brick: &Brick) -> String {
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
        "Unknown"
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
    pub kind: Kind,
    pub removable: bool,
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

        Self { start, end, length, kind, removable: false }
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
            return false;
        }
        self.start.z -= 1;
        self.end.z -= 1;
        true
    }

    pub fn is_directly_above(&self, other: &Brick) -> bool {
        if self.start.z < other.end.z || self.start.z - other.end.z != 1 {
            return false;
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

pub fn slabs2(lines: Vec<String>) -> usize {
    let mut bricks = parse_input(lines);
    let mut bounds = Bounds::new_from(&bricks);
    bounds.extend_by(1);
    println!("bricks.len(): {} ;bounds ({},{},{})", bricks.len(), bounds.max_x, bounds.max_y, bounds.max_z);
    bricks.sort_by(|brick1, brick2|
        brick1.start.z.partial_cmp(&brick2.start.z).unwrap()
    );
    settle_bricks(&mut bricks);

    // Determine which bricks lay on top of which
    let mut below_to_above: HashMap<usize, Vec<usize>> = HashMap::with_capacity(bricks.len());
    let mut above_to_below: HashMap<usize, Vec<usize>> = HashMap::with_capacity(bricks.len());

    for a in 0..bricks.len() {
        for b in 0..bricks.len() {
            if a == b {
                continue;
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
    // Access: space[x][y][z]
    // let space: Vec<Vec<Vec<u8>>> = vec![vec![vec![b' ';bounds.max_z]; bounds.max_y]; bounds.max_x];

    for i in 0..bricks.len() {
        let Some(above_vec) = below_to_above.get(&i) else {
            bricks[i].removable = true;
            continue;
        };
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
            bricks[i].removable = true;
            continue;
        }
    }
    // How many bricks fall if a brick is removed
    let mut fallen_sum = 0;
    for origin_i in 0..bricks.len() {
        if bricks[origin_i].removable {
            continue;
        }

        //println!("Considering {} = {}", &bricks[origin_i], get_demo_brick_label(&bricks[origin_i]));
        let origin_height = bricks[origin_i].end.z;
        let mut fallen_levels: Vec<FallenLevel> = vec![];
        let mut call_stack: Vec<FallenLevel> = vec![FallenLevel::new(origin_i)];
        // finds combinations of levels which are destroyed
        while let Some(current_level) = call_stack.pop() {
            let Some(above_vec) = below_to_above.get(&current_level.brick_index) else {
                fallen_levels.push(current_level);
                continue;
            };
            // if an above_piece above is supported elsewhere it doesn't fall
            let mut immediate_falls = vec![];
            for above_piece in above_vec {
                let Some(below_vec) = above_to_below.get(&above_piece) else {
                    panic!("Unreachable, must be contained in map.");
                };
                // it maps to the brick we came from so it falls along
                if below_vec.len() == 1 {
                    immediate_falls.push(*above_piece);
                    continue;
                }
                // if has more elements below, it needs a check to see if it maps back to the origin brick
                // by mapping to below bricks down to origin_height level and not lower
                let has_alternative = find_alternative_origin_for(
                    *above_piece, origin_i, origin_height, &bricks, &above_to_below
                );
                if !has_alternative {
                    immediate_falls.push(*above_piece);
                }
            }

            if immediate_falls.len() == 0 {
                // no more falls, current is the last brick that falls
                // assuming the pieces above were checked
                fallen_levels.push(current_level);
                continue;
            }

            for fall in immediate_falls {
                let mut any_level = FallenLevel::new(fall);
                any_level.fallen_below.push(current_level.brick_index);
                any_level.fallen_below.extend_from_slice(&current_level.fallen_below);
                call_stack.push(any_level);
            }
        }
        for lvl in fallen_levels.iter_mut() {
            lvl.fallen_below.pop();
        }
        let mut unique: HashSet<usize> = HashSet::new();
        for fallen_level in fallen_levels {
            unique.insert(fallen_level.brick_index);
            for brick_below in &fallen_level.fallen_below {
                unique.insert(*brick_below);
            }
        }
        /*println!("Fallen: {} unique_set: {:?}", unique.len(), unique);
        for el in unique.iter() {
            let the_brick = &bricks[*el];
            println!("El: {the_brick} = {}", get_demo_brick_label(the_brick));
        }*/
        fallen_sum += unique.len();
    }
    fallen_sum
}

// this really ruins the performance but it works
fn find_alternative_origin_for(
    piece_index: usize,
    origin_index: usize,
    origin_height: usize,
    bricks: &Vec<Brick>,
    above_to_below: &HashMap<usize, Vec<usize>>) -> bool {

    let current_z = bricks[piece_index].end.z;
    if current_z == 0 || current_z < origin_height {
        return true;
    }

    let Some(below_vec) = above_to_below.get(&piece_index) else {
        panic!("Unreachable, must be contained in map.");
    };

    for below_i in below_vec {
        if *below_i == origin_index {
            continue;
        }
        if find_alternative_origin_for(*below_i, origin_index, origin_height, &bricks, &above_to_below) {
            return true;
        }
    }

    false
}

struct FallenLevel {
    brick_index: usize,
    fallen_below: Vec<usize>,
}

impl FallenLevel {
    pub fn new(brick_index: usize) -> Self {
        Self { brick_index, fallen_below: vec![] }
    }
}