use crate::days::{self, day18, InputType};

#[test]
fn test_demo_part1() {
    let lines = days::read_lines(18, InputType::DemoPart1);
    let cubic_meters = day18::lagoon1(lines);
    assert_eq!(cubic_meters, 62)
}
#[test]
fn custom_test() {
    let lines = days::read_lines(18, InputType::Custom("custom/mine.txt".into()));
    let cubic_meters = day18::lagoon1(lines);
    assert_eq!(cubic_meters, 111)
}

#[test]
fn test_full_part1() {
    let lines = days::read_lines(18, InputType::Full);
    let cubic_meters = day18::lagoon1(lines);
    // answers: 61629 - 85052
    assert_eq!(cubic_meters, 61865)
}

#[test]
fn test_full_part2() {
    let lines = days::read_lines(18, InputType::Full);
    let cubic_meters = day18::lagoon2(lines);
    assert_eq!(cubic_meters, 1)
}