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