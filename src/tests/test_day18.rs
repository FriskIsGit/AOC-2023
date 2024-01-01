use crate::days::{self, day18, InputType};

#[test]
fn test_demo_part1() {
    let lines = days::read_lines(18, InputType::DemoPart1);
    let cubic_meters = day18::lagoon1(lines);
    assert_eq!(cubic_meters, 62)
}
#[test]
fn custom_test_part1() {
    let lines = days::read_lines(18, InputType::Custom("day18_p1.txt"));
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
fn test_demo_part2() {
    let lines = days::read_lines(18, InputType::DemoPart1);
    let cubic_meters = day18::lagoon2(lines);
    assert_eq!(cubic_meters, 952408144115)
}

#[test]
fn test_full_part2() {
    let lines = days::read_lines(18, InputType::Full);
    let cubic_meters = day18::lagoon2(lines);
    // 40343141815516 answer too low
    assert_eq!(cubic_meters, 40343619199142)
}
