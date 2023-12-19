use crate::days::{self, day15, InputType};

#[test]
fn test_demo_part1() {
    let lines = days::read_lines(15, InputType::DemoPart1);
    let result = day15::lens_library1(lines);
    assert_eq!(result, 1320)
}

#[test]
fn test_full_part1() {
    let lines = days::read_lines(15, InputType::Full);
    let result = day15::lens_library1(lines);
    assert_eq!(result, 508498)
}

#[test]
fn test_demo_part2() {
    let lines = days::read_lines(15, InputType::DemoPart1);
    let focus_power = day15::lens_library2(lines);
    assert_eq!(focus_power, 145)
}

#[test]
fn test_full_part2() {
    let lines = days::read_lines(15, InputType::Full);
    let focus_power = day15::lens_library2(lines);
    assert_eq!(focus_power, 279116)
}