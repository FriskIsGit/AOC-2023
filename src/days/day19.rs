use std::fmt::{Display, format, Formatter, Write};

pub fn aplenty1(lines: Vec<String>) -> usize {
    let (mut workflows, empty_index) = parse_workflows(&lines);
    simplify_workflows(&mut workflows); // Part_sum 6924
    let parts = parse_parts(lines, empty_index);
    println!("Parts count: {}", parts.len());
    let st_index = find_workflow("in", &workflows).expect("Nowhere to start");
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
                            let i = find_workflow(&send_to, &workflows)
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
                    let i = find_workflow(&send_to, &workflows)
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

fn find_workflow(name: &str, workflows: &Vec<Workflow>) -> Option<usize> {
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

pub fn aplenty2(lines: Vec<String>) -> usize {
    let (mut workflows, _) = parse_workflows(&lines);
    simplify_workflows(&mut workflows);
    print_workflows(&workflows);
    println!("Possible combinations: {}", max_possible_combinations());
    let mut combinations = Accumulator { sum: 0 };
    walk_workflow("in", &workflows, Parts::new(), &mut combinations);
    combinations.sum
}

fn walk_workflow(name: &str, workflows: &Vec<Workflow>, mut parts: Parts, acc: &mut Accumulator) {
    let i = find_workflow(&name, &workflows)
        .expect(&format!("Workflow {name} not found"));
    println!("Workflow: {name} | {parts}");
    let workflow = &workflows[i];
    let mut rule_index = 0;
    while rule_index < workflow.rules.len() {
        let rule = &workflow.rules[rule_index];
        match rule {
            Rule::Condition(cond) => {
                // - completely invalid range
                // - can be sliced to be valid partially 323-644 > 500
                // - fully valid range
                let range = match cond.category {
                    b'x' => &parts.x,
                    b'm' => &parts.m,
                    b'a' => &parts.a,
                    b's' => &parts.s,
                    _ => panic!("Unrecognized category")
                };
                let mut fully_valid = false;
                if !range.contains(cond.const_value) {
                    if cond.is_more {
                        // fully valid: 3530-4000 > 2000
                        if range.start > cond.const_value {
                            fully_valid = true;
                        }
                        else {
                            // fully invalid: 323-644 > 1000
                            rule_index += 1;
                            continue
                        }
                    } else {
                        // fully valid: 323-644 < 2000
                        if range.end < cond.const_value {
                            fully_valid = true;
                        }
                        else {
                            // fully invalid: 3533-3900 < 2000
                            rule_index += 1;
                            continue
                        }
                    }
                    // if at this point fully_valid==false there's something wrong
                }

                match cond.get_result() {
                    Rule::Send(send_to) => {
                        if fully_valid {
                            walk_workflow(&send_to, workflows, parts.clone(), acc);
                        } else {
                            let (range1, range2) = range.split_at(
                                cond.is_more, cond.const_value
                            );
                            let mut shrunk_parts = parts.clone();
                            // split parts before passing based on is_more flag
                            if cond.is_more {
                                shrunk_parts.set_range(cond.category, range2);
                                parts.set_range(cond.category, range1);
                                walk_workflow(&send_to, workflows, shrunk_parts, acc);
                            } else {
                                shrunk_parts.set_range(cond.category, range1);
                                parts.set_range(cond.category, range2);
                                walk_workflow(&send_to, workflows, shrunk_parts, acc);
                            }
                        }
                    }
                    Rule::Accepted => {
                        if fully_valid {
                            acc.sum += parts.combinations();
                            break;
                        } else {
                            let (range1, range2) = range.split_at(
                                cond.is_more, cond.const_value
                            );
                            let mut shrunk_parts = parts.clone();
                            if cond.is_more {
                                shrunk_parts.set_range(cond.category, range2);
                                acc.sum += shrunk_parts.combinations();
                                println!("Accepted: {shrunk_parts}");
                                parts.set_range(cond.category, range1);
                            } else {
                                shrunk_parts.set_range(cond.category, range1);
                                acc.sum += shrunk_parts.combinations();
                                println!("Accepted: {shrunk_parts}");
                                parts.set_range(cond.category, range2);
                            }
                        }
                    }
                    Rule::Rejected => {
                        if fully_valid {
                            break;
                        } else {
                            let (range1, range2) = range.split_at(
                                cond.is_more, cond.const_value
                            );
                            if cond.is_more {
                                parts.set_range(cond.category, range1);
                            } else {
                                parts.set_range(cond.category, range2);
                            }
                        }
                    }
                    Rule::Condition(_) => panic!("Unreachable: Condition within condition.")
                }
            }
            Rule::Send(send_to) => {
                walk_workflow(&send_to, workflows, parts, acc);
                break;
            }
            Rule::Accepted => {
                acc.sum += parts.combinations();
                println!("Accepted: {parts}");
                break;
            }
            Rule::Rejected => {
                break;
            }
        }
        rule_index += 1;
    }
}

pub fn max_possible_combinations() -> usize {
    MAX_VALUE * MAX_VALUE * MAX_VALUE * MAX_VALUE
}

#[derive(Clone)]
pub struct Parts {
    pub x: Range,
    pub m: Range,
    pub a: Range,
    pub s: Range,
}
impl Parts {
    pub fn new() -> Self {
        Self {
            x: Range::default(),
            m: Range::default(),
            a: Range::default(),
            s: Range::default()
        }
    }
    pub fn set_range(&mut self, name: u8, range: Range) {
        match name {
            b'x' => self.x = range,
            b'm' => self.m = range,
            b'a' => self.a = range,
            b's' => self.s = range,
            _ => panic!("Only x, m, a and s are accepted")
        }
    }
    pub fn combinations(&self) -> usize {
        self.x.length() * self.m.length() * self.a.length() * self.s.length()
    }
}
impl Display for Parts {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let str = format!("x:{} m:{} a:{} s:{}", self.x, self.m, self.a, self.s);
        formatter.write_str(&str)
    }
}
const MAX_VALUE: usize = 4000;
const MIN_VALUE: usize = 1;
#[derive(Clone)]
pub struct Range {
    // both start and end are inclusive
    pub start: usize,
    pub end: usize,
}
impl Range {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
    pub fn default() -> Self {
        Self { start: MIN_VALUE, end: MAX_VALUE }
    }
    pub fn length(&self) -> usize {
        self.end - self.start + 1
    }
    pub fn contains(&self, value: usize) -> bool {
        self.start <= value && value <= self.end
    }
    // For range 1-4000, condition: s > 2053, 1-2053, 2054-4000,
    // For range 1-4000, condition: s < 2053, 1-2052, 2053-4000,
    // yields lower and higher range always in order
    pub fn split_at(&self, more_than: bool, value: usize) -> (Range, Range) {
        if value < self.start {
            panic!("Split value is less than lower bound");
        } else if value > self.end {
            panic!("Split value is more than higher bound");
        }
        let lower_range = if more_than {
            Range::new(self.start, value)
        } else {
            Range::new(self.start, value-1)
        };
        let higher_range = if more_than {
            Range::new(value+1, self.end)
        } else {
            Range::new(value, self.end)
        };
        (lower_range, higher_range)
    }
}
impl Display for Range {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let str = format!("<{}-{}>", self.start, self.end);
        formatter.write_str(&str)
    }
}
struct Accumulator {
    pub sum: usize
}

