use crate::days::{self, day23, InputType};

#[test]
fn test_demo_part1() {
    let lines = days::read_lines(23, InputType::DemoPart1);
    let output = day23::long_walk1(lines);
    assert_eq!(output, 94)
}
#[test]
fn test_full_part1() {
    let lines = days::read_lines(23, InputType::Full);
    let output = day23::long_walk1(lines);
    assert_eq!(output, 1)
}
