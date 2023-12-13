
// Part1 can be massively improved by translating each mirror into two binary representations
// O(N^2) => O(N) [two number comparisons are made (.=0,#=1)]
pub fn mirrors1(lines: Vec<String>) -> usize {
    let mirrors = parse_input(lines);
    let mut sum = 0;
    for mirror in &mirrors {
        // Perform horizontal checks
        let mut horizontal_mirror = false;
        for r_index in 0..mirror.len()-1 {
            if reflects_horizontally(mirror, r_index, r_index + 1) {
                sum += 100 * (r_index+1);
                horizontal_mirror = true;
                break;
            }
        }
        // If any horizontal mirror was found don't search vertically
        if horizontal_mirror {
            continue
        }
        for col_length in 0..mirror[0].len()-1 {
            if reflects_vertically(mirror, col_length, col_length + 1) {
                sum += col_length+1;
                break;
            }
        }
    }
    sum
}

fn reflects_vertically(mirror: &Vec<Vec<u8>>, index_left: usize, mut index_right: usize) -> bool {
    // index_left = left_col_index; index_right = right_col_index
    let mut index_left = index_left as isize;
    let column_length = mirror[0].len();
    while index_left > -1 && index_right < column_length {
        for row in mirror {
            if row[index_left as usize] != row[index_right] {
                return false;
            }
        }
        index_left -= 1;
        index_right += 1;
    }
    true
}

fn reflects_horizontally(mirror: &Vec<Vec<u8>>, row_index_above: usize, mut row_index_below: usize) -> bool {
    let column_length = mirror[0].len();
    let mut row_index_above = row_index_above as isize;
    while row_index_above > -1 && row_index_below < mirror.len() {
        let row_above = &mirror[row_index_above as usize];
        let row_below = &mirror[row_index_below];
        for i in 0..column_length {
            if row_above[i] != row_below[i] {
                return false;
            }
        }
        row_index_above -= 1;
        row_index_below += 1;
    }
    true
}

pub fn parse_input(mut lines: Vec<String>) -> Vec<Vec<Vec<u8>>> {
    let mut mirrors = vec![];
    let mut mirror_map: Vec<Vec<u8>> = vec![];
    let line_count = lines.len();
    for (i, line) in lines.iter_mut().enumerate() {
        if line.is_empty() {
            mirrors.push(std::mem::take(&mut mirror_map));
            continue;
        }
        let taken_line = std::mem::take(&mut *line);
        mirror_map.push(taken_line.into_bytes());
        if i == line_count - 1 {
            mirrors.push(mirror_map);
            break;
        }
    }
    mirrors
}

pub fn print_table(arr: &Vec<Vec<u8>>) {
    for row in arr {
        for el in row {
            print!("{}", char::from(*el));
        }
        println!()
    }
}

// Executed part2 (bruteforce) in 57ms
pub fn mirrors2(lines: Vec<String>) -> usize {
    let mut mirrors = parse_input(lines);
    let mut reflection_lines = vec![];
    // Find initial reflections to avoid them
    for mirror in &mirrors {
        let mut horizontal_mirror = false;
        for r_index in 0..mirror.len()-1 {
            if reflects_horizontally(mirror, r_index, r_index + 1) {
                reflection_lines.push(Reflection::Horizontal(r_index));
                // store always the upper index
                horizontal_mirror = true;
                break;
            }
        }
        if horizontal_mirror {
            continue
        }
        for col_length in 0..mirror[0].len()-1 {
            if reflects_vertically(mirror, col_length, col_length + 1) {
                reflection_lines.push(Reflection::Vertical(col_length));
                break;
            }
        }
    }
    let mut sum = 0;
    for (i, mirror) in mirrors.iter_mut().enumerate() {
        let mut candidates: Vec<Point> = vec![];
        // Find smudge candidates
        for (r_index, row) in mirror.iter_mut().enumerate() {
            for (c_index, col) in row.iter().enumerate() {
                if *col == b'.' {
                    let point = Point::new(r_index, c_index);
                    candidates.push(point);
                    continue
                }
            }
        }

        let reflection = &reflection_lines[i];
        'candidate_loop: for candidate in candidates {
            mirror[candidate.row_index][candidate.col_index] = b'#';

            let mut reflected = false;
            for r_index in 0..mirror.len()-1 {
                match reflection {
                    Reflection::Horizontal(row_index) => {
                        if r_index == *row_index {
                            continue
                        }
                    }
                    _ => {}
                }
                if reflects_horizontally(mirror, r_index, r_index + 1) {
                    sum += 100 * (r_index+1);
                    reflected = true;
                    break;
                }
            }
            if reflected {
                break 'candidate_loop;
            }
            for col_length in 0..mirror[0].len()-1 {
                match reflection {
                    Reflection::Vertical(col_index) => {
                        if col_length == *col_index {
                            continue
                        }
                    }
                    _ => {}
                }
                if reflects_vertically(mirror, col_length, col_length + 1) {
                    sum += col_length+1;
                    reflected = true;
                    break;
                }
            }
            if reflected {
                break 'candidate_loop;
            }
            mirror[candidate.row_index][candidate.col_index] = b'.';
        }


    }
    sum
}

type Row = usize;
type Col = usize;
pub enum Reflection {
    Horizontal(Row), Vertical(Col)
}
pub struct Point {
    row_index: usize,
    col_index: usize,
}
impl Point {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row_index: row, col_index: col }
    }
}