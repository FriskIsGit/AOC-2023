use crate::days::{self, day3, InputType};

#[test]
fn test_demo_part1() {
    let lines = days::read_lines(3, InputType::DemoPart1);
    let part_number_sum = day3::gear_ratios1(lines);
    assert_eq!(part_number_sum, 4361)
}

#[test]
fn test_full_part1() {
    let lines = days::read_lines(3, InputType::Full);
    let part_number_sum = day3::gear_ratios1(lines);
    assert_eq!(part_number_sum, 529618)
}

#[test]
fn test_demo_part2() {
    let lines = days::read_lines(3, InputType::DemoPart1);
    let gear_sum = day3::gear_ratios2(lines);
    assert_eq!(gear_sum, 467835)
}

#[test]
fn test_full_part2() {
    let lines = days::read_lines(3, InputType::Full);
    let gear_sum = day3::gear_ratios2(lines);
    // 82898011 is too high
    assert_eq!(gear_sum, 77509019)
}

#[test]
fn test_custom_part2() {
    let lines = days::read_lines(3, InputType::Custom("custom/minimal.txt".into()));
    let gear_sum = day3::gear_ratios2(lines);
    assert_eq!(gear_sum, 48708 + 6)
}