pub fn trebuchet_1(lines: Vec<String>) -> usize {
    // initially dummy values
    let mut left_digit = 0;
    let mut right_digit = 0;
    let mut sum = 0;
    for line in lines {
        for byte in line.bytes() {
            if byte.is_ascii_digit() {
                left_digit = byte - 48;
                break;
            }
        };
        for byte in line.bytes().rev() {
            if byte.is_ascii_digit() {
                right_digit = byte - 48;
                break;
            }
        };
        let calibration_value = left_digit * 10 + right_digit;
        sum += calibration_value as usize;
    }
    println!("Total sum: {sum}");
    sum
}

const NUMBERS_STR: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
const NUMBERS: [usize; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

pub fn trebuchet_2(lines: Vec<String>) -> usize {
    // initially dummy values
    let mut left_digit = 0;
    let mut right_digit = 0;
    let mut sum = 0;
    for line in lines {
        for (i, byte) in line.bytes().enumerate() {
            if byte.is_ascii_digit() {
                left_digit = byte - 48;
                break;
            } else if let Some(digit) = match_for_digit(&line, i) {
                left_digit = digit as u8;
                break;
            }
        };
        for (i, byte) in line.bytes().enumerate().rev() {
            if byte.is_ascii_digit() {
                right_digit = byte - 48;
                break;
            } else if let Some(digit) = match_for_digit(&line, i) {
                right_digit = digit as u8;
                break;
            }
        };
        let calibration_value = left_digit * 10 + right_digit;
        sum += calibration_value as usize;
    }
    println!("Total sum: {sum}");
    sum
}

#[allow(non_snake_case)]
pub fn match_for_digit(line: &str, index: usize) -> Option<usize> {
    for (i, NUMBER) in NUMBERS_STR.iter().enumerate() {
        let end = index + NUMBER.len();
        if end > line.len() {
            continue;
        }
        if &line[index..end] == *NUMBER {
            return Some(NUMBERS[i]);
        }
    }
    None
}