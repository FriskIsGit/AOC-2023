use crate::days::{self, day17, InputType};

#[test]
fn test_demo_part1() {
    let lines = days::read_lines(17, InputType::DemoPart1);
    let heat_loss = day17::crucible1(lines);
    assert_eq!(heat_loss, 102)
}

#[test]
fn test_full_part1() {
    let lines = days::read_lines(17, InputType::Full);
    let heat_loss = day17::crucible1(lines);
    assert_eq!(heat_loss, 1)
}

#[test]
fn test_custom_part1() {
    let lines = days::read_lines(17, InputType::Custom("custom/custom17.txt".into()));
    let heat_loss = day17::crucible1(lines);
    assert_eq!(heat_loss, 1)
}
