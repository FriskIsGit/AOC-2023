use crate::days::{self, day5, InputType};

#[test]
fn test_demo_part1() {
    let lines = days::read_lines(5, InputType::DemoPart1);
    let lowest_number = day5::seeds1(lines);
    assert_eq!(lowest_number, 35)
}

#[test]
fn test_full_part1() {
    let lines = days::read_lines(5, InputType::Full);
    let lowest_number = day5::seeds1(lines);
    assert_eq!(lowest_number, 600279879)
}

#[test]
fn custom_part1() {
    let lines = days::read_lines(5, InputType::Custom("one_seed1.txt"));
    let lowest_number = day5::seeds1(lines);
    assert_eq!(lowest_number, 46)
}
#[test]
fn test_demo_part2() {
    let lines = days::read_lines(5, InputType::DemoPart1);
    let lowest_number = day5::seeds2(lines);
    assert_eq!(lowest_number, 46)
}

#[test]
fn test_full_part2() {
    // starting seed: 391_178_260
    let lines = days::read_lines(5, InputType::Full);
    let lowest_number = day5::seeds2(lines);
    assert_eq!(lowest_number, 20191102)
}

#[test]
fn test_full_part2_other() {
    let lines = days::read_lines(5, InputType::Custom("day5.txt"));
    let lowest_number = day5::seeds2(lines);
    assert_eq!(lowest_number, 104070862)
}