use crate::days::{self, day7, InputType};

#[test]
fn test_demo_part1() {
    let lines = days::read_lines(7, InputType::DemoPart1);
    let total_winnings = day7::camel_cards1(lines);
    assert_eq!(total_winnings, 6440)
}

#[test]
fn test_full_part1() {
    let lines = days::read_lines(7, InputType::Full);
    let total_winnings = day7::camel_cards1(lines);
    assert_eq!(total_winnings, 251106089)
}

#[test]
fn test_demo_part2() {
    let lines = days::read_lines(7, InputType::DemoPart1);
    let total_winnings = day7::camel_cards2(lines);
    assert_eq!(total_winnings, 5905)
}

#[test]
fn test_full_part2() {
    let lines = days::read_lines(7, InputType::Full);
    let total_winnings = day7::camel_cards2(lines);
    assert_eq!(total_winnings, 249620106)
}
