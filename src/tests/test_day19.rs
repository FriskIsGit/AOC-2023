use std::ops::RangeInclusive;
use crate::days::{self, day19, InputType};
use crate::days::day19::{Condition, Rule, Workflow};

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
fn test_faulty_parts() {
    let lines = days::read_lines(19, InputType::Custom("faulty_parts.txt"));
    let output = day19::aplenty1(lines);
    assert_eq!(output, 0)
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
    assert_eq!(output, 124831893423809)
}

#[test]
fn test_rev() {
    let vec = vec![1,2,3,4,5,6,7,8];
    for i in (0..5-1).rev() {
        println!("{}", vec[i])
    }
}
#[test]
fn simple_workflows_part2() {
    let lines = days::read_lines(19, InputType::Custom("simple_workflows.txt"));
    let output = day19::aplenty2(lines);
    assert_eq!(output, 128704000000000)
}
#[test]
fn workflow_test() {
    let mut rules = vec![];
    let condition1 = Condition::new(b's', true, 1524, "A".to_string());
    let condition2 = Condition::new(b's', false, 1492, "R".to_string());
    let rule3 = Rule::Rejected;
    rules.push(Rule::Condition(condition1));
    rules.push(Rule::Condition(condition2));
    rules.push(rule3);
    let mut workflow = Workflow::new("pq".into(), rules);
    workflow.simplify_rejected();
    assert_eq!(workflow.rules.len(), 2);
    for rule in &workflow.rules {
        println!("{rule}" );
    }
    assert!(workflow.rules[1] == Rule::Rejected);
}
