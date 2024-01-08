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
    assert_eq!(output, 1)
}