use crate::days::{self, day8, InputType};
use crate::days::day8::CircularBuffer;

#[test]
fn test_demo_part1() {
    let lines = days::read_lines(8, InputType::DemoPart1);
    let steps = day8::wasteland1(lines);
    assert_eq!(steps, 6)
}
#[test]
fn test_custom() {
    let lines = days::read_lines(8, InputType::Custom("two_step.txt"));
    let steps = day8::wasteland1(lines);
    assert_eq!(steps, 2)
}

#[test]
fn test_full_part1() {
    let lines = days::read_lines(8, InputType::Full);
    let steps = day8::wasteland1(lines);
    assert_eq!(steps, 12599)
}

#[test]
fn test_demo_part2() {
    let lines = days::read_lines(8, InputType::DemoPart2);
    let steps = day8::wasteland2(lines);
    assert_eq!(steps, 6)
}

#[test]
fn test_full_part2() {
    let lines = days::read_lines(8, InputType::Full);
    let steps = day8::wasteland2(lines);
    // no LCM answer: 890271798660171933042269950000
    assert_eq!(steps, 8245452805243)
}

#[test]
fn test1_circular_buffer() {
    let mut buffer = CircularBuffer::new(10, 5);
    assert_eq!(buffer.cycle_detection(), 2)
}
#[test]
fn test2_circular_buffer() {
    let mut buffer = CircularBuffer::new(6, 2);
    assert_eq!(buffer.cycle_detection(), 3)
}

#[test]
fn test3_circular_buffer() {
    let mut buffer = CircularBuffer::new(6, 12);
    assert_eq!(buffer.cycle_detection(), 1)
}
#[test]
fn test4_circular_buffer() {
    let mut buffer = CircularBuffer::new(4526, 12);
    assert_eq!(buffer.cycle_detection(), 2263)
}