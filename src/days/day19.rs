use std::fmt::{Display, Formatter};

pub fn aplenty1(lines: Vec<String>) -> usize {
    let (mut workflows, empty_index) = parse_workflows(&lines);
    simplify_workflows(&mut workflows); // Part_sum 6924
    let parts = parse_parts(lines, empty_index);
    println!("Parts count: {}", parts.len());
    let st_index = find_workflow(&workflows, "in").expect("Nowhere to start");
    let mut total_sum = 0;
    let mut current_workflow = &workflows[st_index];

    let mut part_index = 0;
    while part_index < parts.len() {
        let ratings = &parts[part_index];
        let mut part_sum = 0;
        let mut next_part = true;
        let mut rule_index = 0;
        while rule_index < current_workflow.rules.len() {
            let rule = &current_workflow.rules[rule_index];
            // println!("{part_index} Workflow name: {}", current_workflow.name);
            match rule {
                Rule::Condition(cond) => {
                    let rating = ratings.iter().find(|x| x.category == cond.category)
                        .expect("No category");
                    if !cond.satisfies(rating.value) {
                        rule_index += 1;
                        continue
                    }
                    match cond.get_result() {
                        Rule::Send(send_to) => {
                            let i = find_workflow(&workflows, &send_to)
                                .expect(&format!("Workflow {send_to} not found"));
                            current_workflow = &workflows[i];
                            next_part = false;
                            break;
                        }
                        Rule::Accepted => {
                            part_sum += ratings.iter().map(|rating| rating.value).sum::<usize>();
                            current_workflow = &workflows[st_index];
                            break;
                        }
                        Rule::Rejected => {
                            current_workflow = &workflows[st_index];
                            break;
                        }
                        Rule::Condition(_) => panic!("Unreachable: Condition within condition.")
                    }
                }
                Rule::Send(send_to) => {
                    let i = find_workflow(&workflows, &send_to)
                        .expect(&format!("Workflow {send_to} not found"));
                    current_workflow = &workflows[i];
                    next_part = false;
                    break;
                }
                Rule::Accepted => {
                    part_sum += ratings.iter().map(|rating| rating.value).sum::<usize>();
                    current_workflow = &workflows[st_index];
                    break;
                }
                Rule::Rejected => {
                    current_workflow = &workflows[st_index];
                    break;
                }
            }
        }
        if next_part {
            part_index += 1;
        }
        total_sum += part_sum;
    }
    total_sum
}

fn parse_parts(lines: Vec<String>, empty_index: usize) -> Vec<Vec<Rating>> {
    let mut parts: Vec<Vec<Rating>> = vec![];
    let mut rating_index = 0;
    for line in lines.iter().skip(empty_index + 1) {
        parts.push(vec![]);
        let raw_ratings: Vec<&str> = line[1..line.len()-1].split(',').collect();
        for raw_rating in raw_ratings {
            let category = raw_rating.bytes().next().unwrap();
            let value = raw_rating[2..].parse::<usize>().unwrap();
            let rating = Rating::new(category, value);
            parts[rating_index].push(rating)
        }
        rating_index += 1;
    }
    parts
}

fn find_workflow(workflows: &Vec<Workflow>, name: &str) -> Option<usize> {
    for (i, workflow) in workflows.iter().enumerate() {
        if workflow.name == name {
            return Some(i);
        }
    }
    None
}

fn simplify_workflows(workflows: &mut Vec<Workflow>) {
    loop {
        let mut removed_count = 0;
        for workflow in workflows.iter_mut() {
            workflow.simplify_rules();
        }
        let mut i = 0;
        while i < workflows.len() {
            let workflow = &workflows[i];
            if workflow.rules.len() > 1 {
                i += 1;
                continue
            }

            let mut removed = false;
            let only_rule = &workflow.rules[0];
            match only_rule {
                Rule::Accepted | Rule::Rejected => {
                    removed_count += 1;
                    println!("Removing workflow \"{}\"", workflow.name);
                    replace_workflow_with(workflows, workflow.name.clone(), only_rule.clone());
                    workflows.remove(i);
                    removed = true;
                }
                Rule::Condition(_) | Rule::Send(_) => {}
            }
            if !removed {
                i += 1;
            }
        }
        println!("removed_count {removed_count}");
        if removed_count == 0 {
            break;
        }
    }
}

fn print_workflows(workflows: &Vec<Workflow>) {
    for workflow in workflows {
        print!("{}= ", workflow.name);
        for rule in &workflow.rules {
            print!("{rule} ")
        }
        println!()
    }
}

// Substitutes a workflow with a rule
fn replace_workflow_with(workflows: &mut Vec<Workflow>, removable_workflow: String, rule: Rule) {
    for target_workflow in workflows {
        for target_rule in target_workflow.rules.iter_mut() {
            match target_rule {
                Rule::Condition(cond) => {
                    if cond.send_to != *removable_workflow {
                        continue
                    }
                    cond.send_to = match rule {
                        Rule::Accepted => "A".to_string(),
                        Rule::Rejected => "R".to_string(),
                        _ => panic!("No other options expected"),
                    };
                    break;
                },
                Rule::Send(workflow_name) => {
                    if *workflow_name != removable_workflow {
                        continue
                    }
                    *target_rule = rule.clone();
                    break;
                }
                _ => {}
            }
        }
    }
}

fn parse_workflows(lines: &Vec<String>) -> (Vec<Workflow>, usize) {
    let mut workflows = Vec::with_capacity(12);
    let mut empty_index = 0;
    for (i, line) in lines.iter().enumerate() {
        if line.is_empty() {
            empty_index = i;
            break;
        }
        let curl_open = line.find('{').unwrap();
        let name = line[0..curl_open].to_string();
        let rules_raw: Vec<&str> = line[curl_open+1..line.len()-1].split(',').collect();
        let mut rules = vec![];

        for raw_rule in rules_raw {
            if raw_rule.len() == 1 {
                match raw_rule.bytes().next().unwrap() {
                    b'R' => rules.push(Rule::Rejected),
                    b'A' => rules.push(Rule::Accepted),
                    _ => panic!("What is it?")
                }
                continue
            }
            match raw_rule.find(':') {
                Some(colon) => {
                    let mut iter = raw_rule.bytes();
                    let send_to = raw_rule[colon+1..].to_string();
                    let condition = Condition::new(
                        iter.next().unwrap(),
                        iter.next().unwrap() == b'>',
                        raw_rule[2..colon].parse::<usize>().unwrap(),
                        send_to
                    );
                    rules.push(Rule::Condition(condition))
                }
                None => {
                    rules.push(Rule::Send(raw_rule.to_string()))
                }
            }
        }
        let workflow = Workflow::new(name, rules);
        workflows.push(workflow);
    }
    (workflows, empty_index)
}

pub struct Workflow {
    pub name: String,
    pub rules: Vec<Rule>,
}

impl Workflow {
    pub fn new(name: String, rules: Vec<Rule>) -> Self {
        Self { name, rules }
    }
    // Remove conditions defaulting to the same outcome
    pub fn simplify_rules(&mut self) {
        self.simplify_accepted();
        self.simplify_rejected();
    }
    fn simplify_accepted(&mut self) {
        if self.rules.len() < 2 || *self.rules.last().unwrap() != Rule::Accepted {
            return
        }
        let mut last_index = self.rules.len()-1;
        let mut last_valid_index = None;
        for i in (0..=last_index).rev() {
            match &self.rules[i] {
                Rule::Condition(cond) => {
                    match cond.get_result() {
                        Rule::Send(_) | Rule::Rejected => {
                            last_valid_index = Some(i+1);
                            break;
                        }
                        Rule::Accepted => {
                            continue
                        }
                        Rule::Condition(_) => panic!("Unreachable")
                    }
                }
                Rule::Send(_) | Rule::Rejected => {
                    last_valid_index = Some(i+1);
                    break;
                }
                Rule::Accepted => {
                    continue
                }
            }
        }
        if let Some(index) = last_valid_index {
            unsafe { self.rules.set_len(index) }
        } else {
            self.rules.clear();
        }
        self.rules.push(Rule::Accepted);
    }
    pub fn simplify_rejected(&mut self) {
        if self.rules.len() < 2 || *self.rules.last().unwrap() != Rule::Rejected {
            return
        }
        let mut last_index = self.rules.len()-1;
        let mut last_valid_index = None;
        for i in (0..=last_index).rev() {
            match &self.rules[i] {
                Rule::Condition(cond) => {
                    match cond.get_result() {
                        Rule::Send(_) | Rule::Accepted => {
                            last_valid_index = Some(i+1);
                            break;
                        }
                        Rule::Rejected => {
                            continue
                        }
                        Rule::Condition(_) => panic!("Unreachable")
                    }
                }
                Rule::Send(_) | Rule::Accepted => {
                    last_valid_index = Some(i+1);
                    break;
                }
                Rule::Rejected => {
                    continue
                }
            }
        }
        // println!("last_valid_index:{:?}", last_valid_index);
        if let Some(index) = last_valid_index {
            unsafe { self.rules.set_len(index) }
        } else {
            self.rules.clear();
        }
        self.rules.push(Rule::Rejected);
    }
}

type WorkflowName = String;

#[derive(PartialEq, Clone)]
pub enum Rule {
    Condition(Condition),
    Send(WorkflowName),
    Accepted,
    Rejected,
}
impl Display for Rule {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Rule::Condition(condition) => format!("{condition}"),
            Rule::Send(send_to) => format!("SendTo:{send_to}"),
            Rule::Accepted => "Accepted".to_string(),
            Rule::Rejected => "Rejected".to_string()
        };
        formatter.write_str(&str)
    }
}
#[derive(PartialEq, Clone)]
pub struct Condition {
    category: u8,
    is_more: bool,
    const_value: usize,
    send_to: String
}
impl Condition {
    pub fn new(category: u8, is_more: bool, const_value: usize, send_to: String) -> Self {
        Self { category, is_more, const_value, send_to }
    }
    pub fn satisfies(&self, value: usize) -> bool {
        if self.is_more {
            return value > self.const_value;
        }
        return value < self.const_value;
    }
    pub fn get_result(&self) -> Rule {
        let mut result = if self.send_to.len() == 1 {
            match self.send_to.bytes().next().unwrap() {
                b'R' => Rule::Rejected,
                b'A' => Rule::Accepted,
                _ => panic!("Single letter workflow name"),
            }
        } else {
            Rule::Send(self.send_to.clone())
        };
        result
    }
}
impl Display for Condition {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let chr = if self.is_more { '>' } else { '<' };
        let str = format!("{}{chr}{}:{}", char::from(self.category), self.const_value, self.send_to);
        formatter.write_str(&str)
    }
}

pub struct Rating {
    pub category: u8,
    pub value: usize,
}
impl Rating {
    pub fn new(category: u8, value: usize) -> Self {
        Self { category, value }
    }
}

const MAX_VALUE: usize = 4000;
const MIN_VALUE: usize = 1;
pub fn aplenty2(lines: Vec<String>) -> usize {
    let (mut workflows, _) = parse_workflows(&lines);
    println!("Initial count: {}", workflows.len());
    simplify_workflows(&mut workflows);
    println!("After count: {}", workflows.len());
    print_workflows(&workflows);
    let st_index = find_workflow(&workflows, "in").expect("Nowhere to start");
    println!("Possible combinations: {}", max_possible_combinations());
    let mut workflow = &workflows[st_index];
    let mut combinations = 0;
    for rule in &workflow.rules {

    }
    combinations
}
pub fn max_possible_combinations() -> usize {
    MAX_VALUE * MAX_VALUE * MAX_VALUE * MAX_VALUE
}

pub struct Parts {
    pub x: usize,
    pub m: usize,
    pub a: usize,
    pub s: usize,
}
impl Parts {
    pub fn new() -> Self {
        Self { x: 0, m: 0, a: 0, s: 0 }
    }
    pub fn combinations(&self) -> usize {
        self.x * self.m * self.a * self.s
    }
}

