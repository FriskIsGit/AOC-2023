use crate::days::{self, day20, InputType};

#[test]
fn first_demo_part1() {
    let lines = days::read_lines(20, InputType::DemoPart1);
    let output = day20::pulse1(lines);
    assert_eq!(output, 32000000)
}
#[test]
fn second_demo_part1() {
    let lines = days::read_lines(20, InputType::DemoPart2);
    let output = day20::pulse1(lines);
    assert_eq!(output, 11687500)
}

#[test]
fn test_full_part1() {
    let lines = days::read_lines(20, InputType::Full);
    let output = day20::pulse1(lines);
    assert_eq!(output, 925955316)
}
#[test]
fn test_full_custom_part1() {
    let lines = days::read_lines(20, InputType::Custom("day20.txt"));
    let output = day20::pulse1(lines);
    assert_eq!(output, 712543680)
}