use crate::days::{self, day20, InputType};

#[test]
fn test_demo_part1() {
    let lines = days::read_lines(20, InputType::DemoPart1);
    let output = day20::pulse1(lines);
    assert_eq!(output, 11687500)
}

#[test]
fn test_full_part1() {
    let lines = days::read_lines(20, InputType::Full);
    let output = day20::pulse1(lines);
    assert_eq!(output, 1)
}