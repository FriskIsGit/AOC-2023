use crate::days::{self, day16, InputType};

#[test]
fn test_demo_part1() {
    let lines = days::read_lines(16, InputType::DemoPart1);
    let power = day16::beam1(lines);
    assert_eq!(power, 46)
}

#[test]
fn test_full_part1() {
    let lines = days::read_lines(16, InputType::Full);
    let result = day16::beam1(lines);
    assert_eq!(result, 6883)
}

#[test]
fn test_demo_part2() {
    let lines = days::read_lines(16, InputType::DemoPart1);
    let power = day16::beam2(lines);
    assert_eq!(power, 51)
}

#[test]
fn test_full_part2() {
    let lines = days::read_lines(16, InputType::Full);
    let result = day16::beam2(lines);
    assert_eq!(result, 7228)
}