use crate::days::{self, day4, InputType};

#[test]
fn test_demo_part1() {
    let lines = days::read_lines(4, InputType::DemoPart1);
    let points = day4::scratchcards1(lines);
    assert_eq!(points, 13)
}

#[test]
fn test_full_part1() {
    let lines = days::read_lines(4, InputType::Full);
    let points = day4::scratchcards1(lines);
    assert_eq!(points, 21919)
}

#[test]
fn test_demo_part2() {
    let lines = days::read_lines(4, InputType::DemoPart1);
    let scratchcards = day4::scratchcards2(lines);
    assert_eq!(scratchcards, 30)
}

#[test]
fn test_full_part2() {
    let lines = days::read_lines(4, InputType::Full);
    let scratchcards = day4::scratchcards2(lines);
    assert_eq!(scratchcards, 9881048)
}

#[test]
fn test_full_custom_part1() {
    let lines = days::read_lines(4, InputType::Custom("custom4.txt"));
    let scratchcards = day4::scratchcards1(lines);
    assert_eq!(scratchcards, 23441)
}
#[test]
fn test_full_custom_part2() {
    let lines = days::read_lines(4, InputType::Custom("custom4.txt"));
    let scratchcards = day4::scratchcards2(lines);
    assert_eq!(scratchcards, 5923918)
}