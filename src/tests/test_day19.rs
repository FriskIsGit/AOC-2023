use crate::days::{self, day19, InputType};

#[test]
fn test_demo_part1() {
    let lines = days::read_lines(19, InputType::DemoPart1);
    let output = day19::aplenty1(lines);
    assert_eq!(output, 19114)
}

#[test]
fn test_full_part1() {
    let lines = days::read_lines(19, InputType::Full);
    let output = day19::aplenty1(lines);
    assert_eq!(output, 1)
}
