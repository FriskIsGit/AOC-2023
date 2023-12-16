use crate::days::{self, day14, InputType};

#[test]
fn test_demo_part1() {
    let lines = days::read_lines(14, InputType::DemoPart1);
    let sum = day14::parabolic_dish1(lines);
    assert_eq!(sum, 136)
}

#[test]
fn test_full_part1() {
    let lines = days::read_lines(14, InputType::Full);
    let sum = day14::parabolic_dish1(lines);
    assert_eq!(sum, 110677)
}