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
fn test_demo_part2() {
    /*let lines = days::read_lines(11, InputType::DemoPart1);
    let distance_sum = day11::galaxy2(lines);
    // only for EXPANSION_SIZE of 10
    assert_eq!(distance_sum, 1030)*/
}

#[test]
fn test_full_part2() {
    let lines = days::read_lines(11, InputType::Full);
    let distance_sum = day11::galaxy2(lines);
    assert_eq!(distance_sum, 644248339497)
}

#[test]
fn test_custom_part2() {
    let lines = days::read_lines(11, InputType::Custom("simple_galaxy.txt"));
    let distance_sum = day11::galaxy2(lines);
    assert_eq!(distance_sum, 1000001)
}

#[test]
fn galaxy_transformation() {
    let demo_lines = days::read_lines(11, InputType::DemoPart1);
    let galaxy = day11::parse_input(demo_lines, true);
    let correct_transform_input = InputType::Custom("expanded_galaxy.txt");
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

#[test]
fn void_test() {
    let demo_lines = days::read_lines(11, InputType::DemoPart1);
    let galaxy = day11::parse_input(demo_lines, false);
    let vec_rows = day11::find_void_rows(&galaxy);
    let vec_cols = day11::find_void_columns(&galaxy);

    assert_eq!(vec_rows.len(), 2);
    assert_eq!(vec_rows[0], 3);
    assert_eq!(vec_rows[1], 7);
    assert_eq!(vec_cols.len(), 3);
    assert_eq!(vec_cols[0], 2);
    assert_eq!(vec_cols[1], 5);
    assert_eq!(vec_cols[2], 8);
}