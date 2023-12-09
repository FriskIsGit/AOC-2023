use crate::days::{self, day9, InputType};

#[test]
fn test_demo_part1() {
    let lines = days::read_lines(9, InputType::DemoPart1);
    let val_sum = day9::mirage1(lines);
    assert_eq!(val_sum, 18 + 28 + 68)
}

#[test]
fn test_full_part1() {
    let lines = days::read_lines(9, InputType::Full);
    let val_sum = day9::mirage1(lines);
    assert_eq!(val_sum, 1916822650)
}

#[test]
fn test_demo_part2() {
    let lines = days::read_lines(9, InputType::DemoPart2);
    let single_history = day9::mirage2(lines);
    assert_eq!(single_history, 5)
}

#[test]
fn test_demo_part2_demo1() {
    let lines = days::read_lines(9, InputType::DemoPart1);
    let val_sum = day9::mirage2(lines);
    assert_eq!(val_sum, 2)
}

#[test]
fn test_full_part2() {
    let lines = days::read_lines(9, InputType::Full);
    let val_sum = day9::mirage2(lines);
    assert_eq!(val_sum, 966)
}

// ==== Sequence functions tests: ====

#[test]
fn test_sequence_implementation() {
    let sum = day9::sequence_sum(1, 2, 5);
    assert_eq!(25, sum)
}

#[test]
fn negative_seq() {
    // 1 -1 -3 -5
    let sum = day9::sequence_sum(1, -2, 4);
    assert_eq!(-8, sum)
}

#[test]
fn constant_sequence_test() {
    // 1 -1 -3 -5
    let sum = day9::sequence_sum(1, -2, 4);
    assert_eq!(-8, sum)
}