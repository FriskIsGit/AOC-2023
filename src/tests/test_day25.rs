use crate::days::{self, day25, InputType};

#[test]
fn test_demo_part1() {
    // 9 components: cmg, frs, lhk, lsr, nvd, pzl, qnr, rsh, and rzs.
    // 6 components: bvb, hfx, jqt, ntq, rhn, and xhk.
    let lines = days::read_lines(25, InputType::DemoPart1);
    let output = day25::wires1(lines);
    assert_eq!(output, 54)
}
#[test]
fn test_full_part1() {
    let lines = days::read_lines(25, InputType::Full);
    let output = day25::wires1(lines);
    assert_eq!(output, 1)
}