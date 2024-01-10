use crate::days::{self, day22, InputType};

#[test]
fn test_demo_part1() {
    let lines = days::read_lines(22, InputType::DemoPart1);
    let output = day22::slabs1(lines);
    assert_eq!(output, 5)
}
#[test]
fn test_full_part1() {
    let lines = days::read_lines(22, InputType::Full);
    let output = day22::slabs1(lines);
    assert_eq!(output, 441)
}

#[test]
fn test_demo_part2() {
    let lines = days::read_lines(22, InputType::DemoPart1);
    let output = day22::slabs2(lines);
    assert_eq!(output, 7)
}

#[test]
fn test_full_part2() {
    let lines = days::read_lines(22, InputType::Full);
    let output = day22::slabs2(lines);
    // my answer is too high 90132
    assert_eq!(output, 80778)
}