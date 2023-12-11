use std::fmt::{Display, Formatter, Write};

pub fn galaxy1(lines: Vec<String>) -> usize {
    let galaxy = parse_input_expanded(lines);
    let mut galaxies = vec![];
    for (r, row) in galaxy.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if *col == b'#' {
                galaxies.push(Point::new(r, c));
            }
        }
    }
    let pairs = unique_pairs_count(galaxies.len());
    println!("Pairs: {pairs}");
    let mut distance_sum = 0;

    for i in 0..galaxies.len() - 1 {
        for j in i + 1..galaxies.len() {
            let point1 = &galaxies[i];
            let point2 = &galaxies[j];
            let distance = point1.manhattan_distance(point2);
            distance_sum += distance;
        }
    }

    distance_sum
}

fn print_table(arr: &Vec<Vec<u8>>) {
    for row in arr {
        for el in row {
            print!("{}", char::from(*el));
        }
        println!()
    }
}

pub fn parse_input_expanded(mut lines: Vec<String>) -> Vec<Vec<u8>> {
    let mut galaxy_map = Vec::with_capacity(lines.len());
    for line in lines.iter_mut() {
        let taken_line = std::mem::take(&mut *line);
        galaxy_map.push(taken_line.into_bytes());
    }
    let mut r_index = 0;
    let const_column_size = galaxy_map[0].len();
    while r_index < galaxy_map.len() {
        let row = &galaxy_map[r_index];
        if !row.contains(&b'#') {
            let dot_vec = vec![b'.'; const_column_size];
            galaxy_map.insert(r_index, dot_vec);
            r_index += 1;
        }
        r_index += 1;
    }

    let mut c_index = 0;
    while c_index < galaxy_map[0].len() {
        let mut has_galaxy = false;
        for row in 0..galaxy_map.len() {
            let value = &galaxy_map[row][c_index];
            if *value == b'#' {
                has_galaxy = true;
                break;
            }
        }
        if !has_galaxy {
            for row in galaxy_map.iter_mut() {
                row.insert(c_index, b'.');
            }
            c_index += 1;
        }
        c_index += 1;
    }

    galaxy_map
}

pub fn unique_pairs_count(n: usize) -> usize {
    n * (n - 1) / 2
}

#[derive(Clone)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl Point {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
    pub fn is(&self, point: &Point) -> bool {
        self.row == point.row && self.col == point.col
    }
    pub fn manhattan_distance(&self, point: &Point) -> usize {
        let row_diff = self.row as isize - point.row as isize;
        let col_diff = self.col as isize - point.col as isize;
        (row_diff.abs() + col_diff.abs()) as usize
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("[{}, {}]", self.row, self.col))
    }
}