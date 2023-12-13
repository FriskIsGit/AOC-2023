use crate::days::{self, day13, InputType};

#[test]
fn test_demo_part1() {
    let lines = days::read_lines(13, InputType::DemoPart1);
    let sum = day13::mirrors1(lines);
    assert_eq!(sum, 405)
}

#[test]
fn test_full_part1() {
    let lines = days::read_lines(13, InputType::Full);
    let sum = day13::mirrors1(lines);
    // my answer is too low 35507
    assert_eq!(sum, 35538)
}

#[test]
fn test_demo_part2() {
    let lines = days::read_lines(13, InputType::DemoPart1);
    let sum = day13::mirrors2(lines);
    assert_eq!(sum, 400)
}

#[test]
fn test_full_part2() {
    let lines = days::read_lines(13, InputType::Full);
    let sum = day13::mirrors2(lines);
    assert_eq!(sum, 30442)
}

#[test]
fn test_faulty_mirror() {
    let lines = days::read_lines(13, InputType::Custom("custom/faulty_mirror.txt".into()));
    let sum = day13::mirrors1(lines);
    assert_eq!(sum > 0, true)
}

#[test]
fn single_mirror_parse_test() {
    let lines = days::read_lines(13, InputType::Custom("custom/faulty_mirror.txt".into()));
    let mirrors = day13::parse_input(lines);
    assert_eq!(mirrors.len(), 1);
}