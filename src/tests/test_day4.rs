use crate::days::{self, day4, InputType};

#[test]
fn test_demo_part1() {
    let lines = days::read_lines(4, InputType::DemoPart1);
    let points = day4::scratchcards1(lines);
    assert_eq!(points, 13)
}

#[test]
fn test_full_part1() {
}