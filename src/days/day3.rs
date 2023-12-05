use std::collections::HashMap;

pub fn gear_ratios1(lines: Vec<String>) -> usize {
    let mut schematic = vec![vec!['.'; lines[0].len()]; lines.len()];
    for (row, line) in lines.iter().enumerate() {
        for (col, ascii_char) in line.chars().enumerate() {
            schematic[row][col] = ascii_char;
        }
    }
    let mut sum = 0;
    for row in 0..schematic.len() {
        let row_arr = &schematic[row];
        let mut col = 0;
        while col < row_arr.len() {
            let element = row_arr[col];
            let mut end_inclusive = col;
            match element {
                '1'..='9' => {
                    // Find where number ends (col..=end)
                    for i in col..row_arr.len() {
                        if row_arr[i].is_ascii_digit() {
                            end_inclusive = i;
                        } else {
                            break;
                        }
                    }
                    // Check adjacent symbols
                    if is_part_number(row, col, end_inclusive, &schematic) {
                        let digits = &lines[row][col..=end_inclusive];
                        let number = digits.parse::<usize>().unwrap();
                        sum += number;
                    }
                    // Move iterator
                    col = end_inclusive;
                }
                _ => {}
            }
            col += 1;
        }
    }
    sum
}

fn is_symbol(chr: char) -> bool {
    chr != '.' && !chr.is_ascii_digit()
}

fn is_part_number(row: usize, col: usize, end: usize, schematic: &Vec<Vec<char>>) -> bool {
    let mut is_part_number = row != 0 && col != 0 && is_symbol(schematic[row - 1][col - 1]); // top left
    if is_part_number {
        return true;
    }
    is_part_number = col != 0 && is_symbol(schematic[row][col - 1]); // left
    if is_part_number {
        return true;
    }
    is_part_number = row + 1 < schematic.len() && col != 0 && is_symbol(schematic[row + 1][col - 1]); // bottom left
    if is_part_number {
        return true;
    }
    //Check top
    if row != 0 {
        for i in col..=end {
            if is_symbol(schematic[row - 1][i]) {
                return true;
            }
        }
    }
    // Check bottom
    if row + 1 < schematic.len() {
        for i in col..=end {
            if is_symbol(schematic[row + 1][i]) {
                return true;
            }
        }
    }
    let all_rows = schematic.len();
    let all_cols = schematic[0].len();
    is_part_number = row != 0 && end + 1 < all_cols && is_symbol(schematic[row - 1][end + 1]); // top right
    if is_part_number {
        return true;
    }
    is_part_number = end + 1 < all_cols && is_symbol(schematic[row][end + 1]); // right
    if is_part_number {
        return true;
    }
    row + 1 < all_rows && end + 1 < all_cols && is_symbol(schematic[row + 1][end + 1]) // bottom right
}

fn is_star(chr: char) -> bool {
    chr == '*'
}

fn adjacent_star(row: usize, col: usize, end: usize, schematic: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    if row != 0 && col != 0 && is_star(schematic[row - 1][col - 1]) {
        return Some((row - 1, col - 1));
    }
    if col != 0 && is_symbol(schematic[row][col - 1]) {
        return Some((row, col - 1));
    }
    if row + 1 < schematic.len() && col != 0 && is_symbol(schematic[row + 1][col - 1]) {
        return Some((row + 1, col - 1));
    }
    //Check top
    if row != 0 {
        for i in col..=end {
            if is_star(schematic[row - 1][i]) {
                return Some((row - 1, i));
            }
        }
    }
    // Check bottom
    if row + 1 < schematic.len() {
        for i in col..=end {
            if is_star(schematic[row + 1][i]) {
                return Some((row + 1, i));
            }
        }
    }
    let all_rows = schematic.len();
    let all_cols = schematic[0].len();
    if row != 0 && end + 1 < all_cols && is_star(schematic[row - 1][end + 1]) {
        return Some((row - 1, end + 1));
    }
    if end + 1 < all_cols && is_star(schematic[row][end + 1]) {
        return Some((row, end + 1));
    }
    if row + 1 < all_rows && end + 1 < all_cols && is_star(schematic[row + 1][end + 1]) {
        return Some((row + 1, end + 1));
    }
    return None;
}

pub fn gear_ratios2(lines: Vec<String>) -> usize {
    let all_rows = lines.len();
    let all_columns = lines[0].len();
    let mut schematic = vec![vec!['.'; all_columns]; all_rows];
    let mut star_map: HashMap<u16, StarParts> = HashMap::new();
    for (row, line) in lines.iter().enumerate() {
        for (col, ascii_char) in line.chars().enumerate() {
            schematic[row][col] = ascii_char;
        }
    }
    for row in 0..schematic.len() {
        let row_arr = &schematic[row];
        let mut col = 0;
        while col < row_arr.len() {
            let element = row_arr[col];
            let mut end_inclusive = col;
            match element {
                '1'..='9' => {
                    // Find where number ends (col..=end)
                    for i in col..row_arr.len() {
                        if row_arr[i].is_ascii_digit() {
                            end_inclusive = i;
                        } else {
                            break;
                        }
                    }
                    // Check adjacent symbols
                    if is_part_number(row, col, end_inclusive, &schematic) {
                        if let Some(star) = adjacent_star(row, col, end_inclusive, &schematic) {
                            let digits = &lines[row][col..=end_inclusive];
                            let number = digits.parse::<usize>().unwrap();
                            let hash = calculate_hash(star.0 as u16, star.1 as u16);
                            if !star_map.contains_key(&hash) {
                                star_map.insert(hash, StarParts::new());
                            }
                            let star_key = star_map.get_mut(&hash);
                            let mut star = star_key.expect("Must exist at this point");
                            star.add(number);
                        }
                    }
                    // Move iterator
                    col = end_inclusive;
                }
                _ => {}
            }
            col += 1;
        }
    }
    let mut gear_sum = 0;
    for star in star_map.values() {
        if let Some(factor) = star.factor() {
            gear_sum += factor;
        }
    }
    gear_sum
}

fn calculate_hash(x: u16, y: u16) -> u16 {
    let mut hash = 0u16;
    hash |= x;
    hash |= y << 8;
    hash
}
fn get_position(hash: u16) -> (u16, u16) {
    let x = hash & 0xFF;
    let y = hash >> 8;
    (x, y)
}

#[derive(Clone)]
struct StarParts {
    size: usize,
    num1: usize,
    num2: usize,
}
impl StarParts {
    pub fn new() -> Self {
        Self { size: 0, num1: 0, num2: 0 }
    }
    // accepts any number adjacent to a symbol (part numbers)
    pub fn add(&mut self, num: usize) {
        match self.size {
            0 => self.num1 = num,
            1 => self.num2 = num,
            _ => {},
        }
        self.size += 1;
    }
    // having exactly two part numbers otherwise None
    pub fn factor(&self) -> Option<usize> {
        if self.size == 2 {
            return Some(self.num1 * self.num2);
        }
        None
    }
}
