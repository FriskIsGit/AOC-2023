use crate::days::{self, day21, InputType};

#[test]
fn test_demo_part1() {
    let lines = days::read_lines(21, InputType::DemoPart1);
    let output = day21::step_counter1(lines);
    assert_eq!(output, 16)
}
#[test]
fn test_full_part1() {
    let lines = days::read_lines(21, InputType::Full);
    let output = day21::step_counter1(lines);
    assert_eq!(output, 1)
}