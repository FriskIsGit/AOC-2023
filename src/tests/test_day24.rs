use crate::days::{self, day24, InputType};

#[test]
fn test_demo_part1() {
    let lines = days::read_lines(24, InputType::DemoPart1);
    let output = day24::hailstones_demo1(lines);
    assert_eq!(output, 2)
}
#[test]
fn test_full_part1() {
    let lines = days::read_lines(24, InputType::Full);
    let output = day24::hailstones1(lines);
    assert_eq!(output, 20361)
}

#[test]
fn test_demo_part2() {
    let lines = days::read_lines(24, InputType::DemoPart1);
    let output = day24::hailstones2(lines);
    assert_eq!(output, 47)
}
#[test]
fn test_full_part2() {
    let lines = days::read_lines(24, InputType::Full);
    let output = day24::hailstones2(lines);
    assert_eq!(output, 1)
}

