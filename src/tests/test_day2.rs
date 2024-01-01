use crate::days::{self, day2, InputType};

#[test]
fn test_demo_part1() {
    let lines = days::read_lines(2, InputType::DemoPart1);
    let id_sum = day2::conundrum_1(lines);
    assert_eq!(id_sum, 8)
}

#[test]
fn test_full_part1() {
    let lines = days::read_lines(2, InputType::Full);
    let id_sum = day2::conundrum_1(lines);
    assert_eq!(id_sum, 2285)
}

#[test]
fn test_demo_part2() {
    // Demo input doesn't change in part2
    let lines = days::read_lines(2, InputType::DemoPart1);
    let id_sum = day2::conundrum_2(lines);
    assert_eq!(id_sum, 2286)
}

#[test]
fn test_full_part2() {
    // Demo input doesn't change in part2
    let lines = days::read_lines(2, InputType::Full);
    let id_sum = day2::conundrum_2(lines);
    assert_eq!(id_sum, 77021)
}
#[test]
fn test_full_custom_p1() {
    let lines = days::read_lines(2, InputType::Custom("day2.txt"));
    let id_sum = day2::conundrum_1(lines);
    assert_eq!(id_sum, 2101)
}