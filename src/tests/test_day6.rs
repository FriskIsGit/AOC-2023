use crate::days::{self, day6, InputType};

#[test]
fn test_demo_part1() {
    let lines = days::read_lines(6, InputType::DemoPart1);
    let ways_factor = day6::boats1(lines);
    assert_eq!(ways_factor, 288)
}

#[test]
fn test_full_part1() {
    let lines = days::read_lines(6, InputType::Full);
    let ways_factor = day6::boats1(lines);
    assert_eq!(ways_factor, 6209190)
}

#[test]
fn test_demo_part2_spaced() {
    let lines = days::read_lines(6, InputType::DemoPart1);
    let better_ways = day6::boats2(lines);
    assert_eq!(better_ways, 71503)
}
#[test]
fn test_demo_part2_joined() {
    let lines = days::read_lines(6, InputType::DemoPart2);
    let better_ways = day6::boats2(lines);
    assert_eq!(better_ways, 71503)
}

#[test]
fn test_full_part2() {
    let lines = days::read_lines(6, InputType::Full);
    let better_ways = day6::boats2(lines);
    assert_eq!(better_ways, 28545089)
}