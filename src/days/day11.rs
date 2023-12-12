use std::fmt::{Display, Formatter, Write};
// two solutions can be merged into one with a parameter but that would mean different input parameters
pub fn galaxy1(lines: Vec<String>) -> usize {
    let galaxy = parse_input(lines, true);
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

pub fn parse_input(mut lines: Vec<String>, expand: bool) -> Vec<Vec<u8>> {
    let mut galaxy_map = Vec::with_capacity(lines.len());
    for line in lines.iter_mut() {
        let taken_line = std::mem::take(&mut *line);
        galaxy_map.push(taken_line.into_bytes());
    }
    if !expand {
        return galaxy_map;
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


const EXPANSION_SIZE: usize = 1_000_000;
pub fn galaxy2(lines: Vec<String>) -> usize {
    let galaxy = parse_input(lines, false);
    let mut galaxies = vec![];
    for (r, row) in galaxy.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if *col == b'#' {
                galaxies.push(Point::new(r, c));
            }
        }
    }
    let void_rows = find_void_rows(&galaxy);
    let void_columns = find_void_columns(&galaxy);
    let mut distance_sum = 0;
    for i in 0..galaxies.len() - 1 {
        for j in i + 1..galaxies.len() {
            let mut point1 = &galaxies[i];
            let mut point2 = &galaxies[j];
            if point1.row > point2.row {
                let temp = point1;
                point1 = point2;
                point2 = temp;
            }
            let mut row_voids_crossed = 0;
            for void_row in &void_rows {
                if point1.row < *void_row && *void_row < point2.row {
                    row_voids_crossed += 1;
                }
            }
            if point1.col > point2.col {
                let temp = point1;
                point1 = point2;
                point2 = temp;
            }
            let mut col_voids_crossed = 0;
            for void_col in &void_columns {
                if point1.col < *void_col && *void_col < point2.col {
                    col_voids_crossed += 1;
                }
            }

            let row_displacement = ((EXPANSION_SIZE - 1) * row_voids_crossed) as isize;
            let col_displacement = ((EXPANSION_SIZE - 1) * col_voids_crossed) as isize;
            let (mut row1, mut col1) = (point1.row as isize, point1.col as isize);
            if point1.row < point2.row {
                row1 -= row_displacement;
            } else {
                row1 += row_displacement;
            }
            if point1.col < point2.col {
                col1 -= col_displacement;
            } else {
                col1 += col_displacement;
            }
            let space_distance = isize_distance(row1, col1, point2.row as isize, point2.col as isize);
            distance_sum += space_distance;
        }
    }
    distance_sum
}

pub fn find_void_rows(galaxy_map: &Vec<Vec<u8>>) -> Vec<usize> {
    let mut rows = vec![];
    for (i, row) in galaxy_map.iter().enumerate() {
        if !row.contains(&b'#') {
            rows.push(i);
        }
    }
    rows
}
pub fn find_void_columns(galaxy_map: &Vec<Vec<u8>>) -> Vec<usize> {
    let mut columns = vec![];
    let const_col_length = galaxy_map[0].len();
    let const_row_length = galaxy_map.len();
    for col_index in 0..const_col_length {
        let mut has_galaxy = false;
        for row_index in 0..const_row_length {
            let value = &galaxy_map[row_index][col_index];
            if *value == b'#' {
                has_galaxy = true;
                break;
            }
        }
        if !has_galaxy {
            columns.push(col_index);
        }
    }
    columns
}

pub fn isize_distance(row1: isize, col1: isize, row2: isize, col2: isize) -> usize {
    let row_diff = row1 - row2;
    let col_diff = col1 - col2;
    (row_diff.abs() + col_diff.abs()) as usize
}