use crate::days::{self, day1, InputType};

#[test]
fn test_demo_part1() {
    let lines = days::read_lines(1, InputType::DemoPart1);
    let computed_sum = day1::trebuchet_1(lines);
    assert_eq!(computed_sum, 142)
}

#[test]
fn test_full_part1() {
    let lines = days::read_lines(1, InputType::Full);
    let computed_sum = day1::trebuchet_1(lines);
    assert_eq!(computed_sum, 56397)
}

#[test]
fn test_demo_part2() {
    let lines = days::read_lines(1, InputType::DemoPart2);
    let computed_sum = day1::trebuchet_2(lines);
    assert_eq!(computed_sum, 281)
}

#[test]
fn test_full_part2() {
    let lines = days::read_lines(1, InputType::Full);
    let computed_sum = day1::trebuchet_2(lines);
    assert_eq!(computed_sum, 55701)
}

#[test]
fn test_custom_part1() {
    let lines = days::read_lines(1, InputType::Custom("one_line1.txt"));
    assert_eq!(day1::trebuchet_1(lines), 11)
}

#[test]
fn test_custom_part2() {
    let lines = days::read_lines(1, InputType::Custom("one_line1.txt"));
    assert_eq!(day1::trebuchet_2(lines), 29)
}

#[test]
fn other_full_input_p1() {
    let lines = days::read_lines(1, InputType::Custom("other_full_input.txt"));
    let computed_sum = day1::trebuchet_1(lines);
    assert_eq!(computed_sum, 54239)
}

#[test]
fn other_full_input_p2() {
    let lines = days::read_lines(1, InputType::Custom("other_full_input.txt"));
    let computed_sum = day1::trebuchet_2(lines);
    assert_eq!(computed_sum, 55343)
}