use crate::days::InputType;

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

// TODO: Improvement: Don't allocate every cell for stars
pub fn gear_ratios2(lines: Vec<String>) -> usize {
    let all_rows = lines.len();
    let all_columns = lines[0].len();
    println!("{all_rows}x{all_columns}");
    let mut schematic = vec![vec!['.'; all_columns]; all_rows];
    let mut stars = vec![vec![StarParts::new(); all_columns]; all_rows];
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
                            stars[star.0][star.1].add(number);
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
    for row in 0..schematic.len() {
        let row_arr = &schematic[row];
        let mut col = 0;
        while col < row_arr.len() {
            if let Some(factor) = stars[row][col].factor() {
                gear_sum += factor;
            }
            col += 1;
        }
    }
    gear_sum
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

// This will return the gear ratio of exactly two numbers OR 0
fn search_around_star(row: usize, col: usize, schematic: &Vec<Vec<char>>) -> usize {
    let all_columns = schematic[0].len();
    let mut number_count = 0;
    let mut top_left = 0;
    let mut top = 0;
    let mut top_right = 0;
    // Check top left
    if row != 0 && col != 0 && schematic[row - 1][col - 1].is_ascii_digit() {
        top_left = get_number(col - 1, &schematic[row - 1]);
        number_count += 1;
    }
    // Check top only if top_left is missing
    if top_left == 0 && row != 0 && schematic[row - 1][col].is_ascii_digit() {
        top = get_number(col, &schematic[row - 1]);
        number_count += 1;
    }
    // Check top right only if top is missing
    if top == 0 && row != 0 && col + 1 < all_columns && schematic[row - 1][col + 1].is_ascii_digit() {
        top_right = get_number(col + 1, &schematic[row - 1]);
        if top_left != top_right {
            number_count += 1;
        }
    }
    let mut left = 0;
    if col != 0 && schematic[row][col - 1].is_ascii_digit() { // Check left
        left = get_number(col - 1, &schematic[row]);
        number_count += 1;
    }
    if number_count > 2 {
        return 0;
    }
    let mut right = 0;
    if col + 1 < all_columns && schematic[row][col + 1].is_ascii_digit() { // Check right
        right = get_number(col + 1, &schematic[row]);
        number_count += 1;
    }
    if number_count > 2 {
        return 0;
    }
    let all_rows = schematic.len();
    let mut bottom_left = 0;
    let mut bottom = 0;
    let mut bottom_right = 0;
    // Check bottom left
    if row + 1 < all_rows && col != 0 && schematic[row + 1][col - 1].is_ascii_digit() {
        bottom_left = get_number(col - 1, &schematic[row + 1]);
        number_count += 1;
    }
    if number_count > 2 {
        return 0;
    }
    // Check bottom only if bot_left is missing
    if bottom_left == 0 && row + 1 < all_rows && schematic[row + 1][col].is_ascii_digit() {
        bottom = get_number(col, &schematic[row + 1]);
        number_count += 1;
    }
    if number_count > 2 {
        return 0;
    }
    // Check bot_right only if bottom is missing
    if bottom == 0 && row + 1 < all_rows && col + 1 < all_columns && schematic[row + 1][col + 1].is_ascii_digit() {
        bottom_right = get_number(col + 1, &schematic[row + 1]);
        if bottom_left != bottom_right {
            number_count += 1;
        }
    }
    if number_count != 2 {
        return 0;
    }
    multiply_two_non_zeroes(vec![top_left, top, top_right, left, right, bottom_left, bottom, bottom_right])
}

fn get_number(col: usize, row_arr: &Vec<char>) -> usize {
    let mut end_inclusive = col;
    let mut st_inclusive = col;
    for i in (0..col).rev() {
        if row_arr[i].is_ascii_digit() {
            st_inclusive = i;
        } else {
            break;
        }
    }
    // Find where number ends (col..=end)
    for i in col..row_arr.len() {
        if row_arr[i].is_ascii_digit() {
            end_inclusive = i;
        } else {
            break;
        }
    }
    let number: String = row_arr[st_inclusive..=end_inclusive].iter().collect();
    println!("Mapped number: {number}");
    number.parse::<usize>().expect("Failed to parse number")
}

fn multiply_two_non_zeroes(vec: Vec<usize>) -> usize {
    let mut factor1 = 0;
    let mut factor2 = 0;
    for num in vec {
        if num == 0 {
            continue;
        }
        if factor1 == 0 {
            factor1 = num;
            continue;
        }
        factor2 = num;
        break;
    }
    factor1 * factor2
}
