use std::usize;
use crate::days::{self, day12, InputType};

#[test]
fn test_demo_part1() {
    let lines = days::read_lines(12, InputType::DemoPart1);
    let arrangements = day12::hot_springs1(lines);
    assert_eq!(arrangements, 21)
}

#[test]
fn test_full_part1() {
    let lines = days::read_lines(12, InputType::Full);
    let arrangements = day12::hot_springs1(lines);
    assert_eq!(arrangements, 7460)
}

#[test]
fn bit_flipper_test() {
    let bits = 3;
    let combinations = usize::pow(2, bits as u32);
    println!("Combinations: {}", combinations);
    let mut flipper = day12::BitFlipper::new(bits);
    for i in 0..combinations+1 {
        print!("[");
        for b in 0..bits {
            let on = flipper.bits[b];
            if on {
                print!("1");
            } else {
                print!("0");
            }
            if b != bits-1 {
                print!(", ");
            }
        }
        println!("]");
        flipper.next();
    }
}

#[test]
fn matching_groups_test1() {
    let springs = vec![b'.', b'.', b'#', b'.', b'#', b'#', b'#'];
    let groups = vec![1,1,3];
    let is_matching = day12::matching_groups(&springs, &groups);
    assert!(!is_matching)
}

#[test]
fn matching_groups_test2() {
    let springs = vec![b'.', b'#', b'.', b'#', b'#', b'#', b'.', b'#', b'.', b'#', b'#', b'#', b'#', b'#', b'#'];
    let groups = vec![1,1,3];
    let is_matching = day12::matching_groups(&springs, &groups);
    assert!(!is_matching)
}

#[test]
fn matching_groups_test3() {
    let springs = vec![b'.', b'#', b'.', b'#', b'.', b'#', b'.', b'#'];
    let groups = vec![1,1,1];
    let is_matching = day12::matching_groups(&springs, &groups);
    assert!(!is_matching)
}

#[test]
fn vec_eq_test() {
    let groups1 = vec![3,3,3];
    let groups2 = vec![3,3,3];
    let eq = groups1.eq(&groups2);
    assert!(eq)
}

#[test]
fn vec_nq_test() {
    let groups1 = vec![9,9,4];
    let groups2 = vec![9,9,3];
    let eq = groups1.eq(&groups2);
    assert!(!eq)
}