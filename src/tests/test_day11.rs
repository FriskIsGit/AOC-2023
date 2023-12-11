use crate::days::{self, day11, InputType};
use crate::days::day11::Point;

#[test]
fn test_demo_part1() {
    let lines = days::read_lines(11, InputType::DemoPart1);
    let distance_sum = day11::galaxy1(lines);
    assert_eq!(distance_sum, 374)
}

#[test]
fn test_full_part1() {
    let lines = days::read_lines(11, InputType::Full);
    let distance_sum = day11::galaxy1(lines);
    assert_eq!(distance_sum, 9627977)
}

#[test]
fn galaxy_transformation() {
    let demo_lines = days::read_lines(11, InputType::DemoPart1);
    let galaxy = day11::parse_input_expanded(demo_lines);
    let correct_transform_input = InputType::Custom("custom/expanded_galaxy.txt".into());
    let expanded_lines = days::read_lines(11, correct_transform_input);
    assert_eq!(galaxy.len(), expanded_lines.len());
    assert_eq!(galaxy[0].len(), expanded_lines[0].len());
}
#[test]
fn distance_test() {
    let point1 = Point::new(6,6);
    let point2 = Point::new(8,8);
    assert_eq!(point1.manhattan_distance(&point2), 4)
}
#[test]
fn mysterious_zero_distance() {
    let point1 = Point::new(7, 12);
    let point2 = Point::new(10,9);
    assert_eq!(point1.manhattan_distance(&point2), 6)
}

#[test]
fn galaxy_distance() {
    let galaxy3 = Point::new(2, 0);
    let galaxy6 = Point::new(7, 12);
    assert_eq!(galaxy3.manhattan_distance(&galaxy6), 17);
    assert_eq!(galaxy6.manhattan_distance(&galaxy3), 17)
}