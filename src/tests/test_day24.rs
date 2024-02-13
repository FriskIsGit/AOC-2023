use crate::days::{self, day24, InputType};
use crate::days::day24::{Hailstone, Point3D};

#[test]
fn test_demo_part1() {
    let lines = days::read_lines(24, InputType::DemoPart1);
    let output = day24::hailstones_demo1(lines);
    assert_eq!(output, 2)
}
#[test]
fn test_full_part1() {
    let lines = days::read_lines(24, InputType::Full);
    let output = day24::hailstones1(lines);
    assert_eq!(output, 20361)
}

#[test]
fn test_demo_part2() {
    let lines = days::read_lines(24, InputType::DemoPart1);
    let output = day24::hailstones2(lines);
    assert_eq!(output, 47)
}
#[test]
fn test_full_part2() {
    let lines = days::read_lines(24, InputType::Full);
    let output = day24::hailstones2(lines);
    assert_eq!(output, 1)
}

#[test]
fn float_is_integer_test1() {
    let num1 = -47.000000000000014;
    let int = day24::is_integer(num1);
    assert!(int)
}
#[test]
fn float_is_integer_test2() {
    let num1 = 41.999999999999914;
    let int = day24::is_integer(num1);
    assert!(int)
}
#[test]
fn float_is_integer_test3() {
    let num1 = 24.000005000014;
    let not_int = day24::is_integer(num1);
    assert!(!not_int)
}
#[test]
fn float_is_integer_test4() {
    let whole = 24.0;
    let is_int = day24::is_integer(whole);
    assert!(is_int)
}

#[test]
fn hailstone_at_time_test() {
    let pos = Point3D::new(20, 19, 15);
    let vel = Point3D::new( 1, -5, -3);
    let hailstone = Hailstone::new(pos, vel);
    let time1 = hailstone.at_time_2d(1);
    assert_eq!(time1.x, 21.0);
    assert_eq!(time1.y, 14.0);
}

#[test]
fn round_f64_positive_test() {
    let positive: f64 = 5.000000000000001;
    assert_eq!(positive.round(), 5.0)
}

#[test]
fn round_f64_negative_test() {
    let negative: f64 = -5.000000000000001;
    assert_eq!(negative.round(), -5.0)
}

#[test]
fn round_f64_negative_down() {
    let negative: f64 = -5.999999991;
    assert_eq!(negative.round(), -6.0)
}

