use crate::days::{self, day10, InputType};

#[test]
fn test_demo_part1() {
    let lines = days::read_lines(10, InputType::DemoPart1);
    let farthest_pos = day10::maze1(lines);
    assert_eq!(farthest_pos, 8)
}

#[test]
fn test_full_part1() {
    let lines = days::read_lines(10, InputType::Full);
    let farthest_pos = day10::maze1(lines);
    assert_eq!(farthest_pos, 6897)
}

#[test]
fn test_custom_part1() {
    let lines = days::read_lines(10, InputType::Custom("simple_pipe.txt"));
    let farthest_pos = day10::maze1(lines);
    assert_eq!(farthest_pos, 4)
}

#[test]
fn test_part1_on_demo2() {
    let lines = days::read_lines(10, InputType::DemoPart2);
    let farthest_pos = day10::maze1(lines);
    assert_eq!(farthest_pos, 23)
}

#[test]
fn test_demo_part2() {
    let lines = days::read_lines(10, InputType::DemoPart2);
    let enclosed = day10::maze2(lines);
    assert_eq!(enclosed, 4)
}
#[test]
fn test_custom_part2() {
    let lines = days::read_lines(10, InputType::Custom("bigger_demo.txt"));
    let enclosed = day10::maze2(lines);
    assert_eq!(enclosed, 10)
}

#[test]
fn test_full_part2() {
    let lines = days::read_lines(10, InputType::Full);
    let enclosed = day10::maze2(lines);
    assert_eq!(enclosed, 367); // your answer is too low: 277
}