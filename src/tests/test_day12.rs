use crate::days::{self, day12, InputType};

#[test]
fn test_demo_part1() {
    let lines = days::read_lines(12, InputType::DemoPart1);
    let arrangements = day12::hot_springs1(lines);
    assert_eq!(arrangements, 21)
}

#[test]
fn test_full_part1() {
    let lines = days::read_lines(12, InputType::Full);
    let arrangements = day12::hot_springs1(lines);
    assert_eq!(arrangements, 1)
}
