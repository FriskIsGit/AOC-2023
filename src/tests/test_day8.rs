use crate::days::{self, day8, InputType};

#[test]
fn test_demo_part1() {
    let lines = days::read_lines(8, InputType::DemoPart1);
    let steps = day8::wasteland1(lines);
    assert_eq!(steps, 6)
}
#[test]
fn test_custom() {
    let lines = days::read_lines(8, InputType::Custom("custom/two_step.txt".into()));
    let steps = day8::wasteland1(lines);
    assert_eq!(steps, 2)
}

#[test]
fn test_full_part1() {
    let lines = days::read_lines(8, InputType::Full);
    let steps = day8::wasteland1(lines);
    assert_eq!(steps, 12599)
}

#[test]
fn test_demo_part2() {
    let lines = days::read_lines(8, InputType::DemoPart2);
    let steps = day8::wasteland2(lines);
    assert_eq!(steps, 6)
}

#[test]
fn test_full_part2() {
    let lines = days::read_lines(8, InputType::Full);
    let steps = day8::wasteland2(lines);
    assert_eq!(steps, 1)
}