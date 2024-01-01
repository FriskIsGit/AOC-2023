use crate::days::{self, day19, InputType};

#[test]
fn test_demo_part1() {
    let lines = days::read_lines(19, InputType::DemoPart1);
    let output = day19::aplenty1(lines);
    assert_eq!(output, 19114)
}

#[test]
fn test_full_part1() {
    let lines = days::read_lines(19, InputType::Full);
    let output = day19::aplenty1(lines);
    assert_eq!(output, 350678)
}

#[test]
fn test_demo_part2() {
    let lines = days::read_lines(19, InputType::DemoPart1);
    let output = day19::aplenty2(lines);
    assert_eq!(output, 167409079868000)
}

#[test]
fn test_full_part2() {
    let lines = days::read_lines(19, InputType::Full);
    let output = day19::aplenty2(lines);
    assert_eq!(output, 1)
}

#[test]
fn test_rev() {
    let vec = vec![1,2,3,4,5,6,7,8];
    for i in (0..5-1).rev() {
        println!("{}", vec[i])
    }
}
