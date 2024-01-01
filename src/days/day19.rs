use std::fmt::{Display, Formatter};

pub fn aplenty1(lines: Vec<String>) -> usize {
    let (workflows, empty_index) = parse_workflows(&lines);
    let parts = parse_parts(lines, empty_index);
    let st_index = find_workflow(&workflows, "in").expect("Nowhere to start");
    let mut total_sum = 0;
    let mut current_workflow = &workflows[st_index];

    let mut part_index = 0;
    while part_index < parts.len() {
        let ratings = &parts[part_index];
        let mut part_sum = 0;
        let mut next_part = true;
        for rule in &current_workflow.rules {
            match rule {
                Rule::Condition(cond) => {
                    let rating = ratings.iter().find(|x| x.category == cond.category).expect("No category");
                    if !cond.satisfies(rating.value) {
                        continue
                    }
                    match cond.get_result() {
                        Rule::Send(send_to) => {
                            let i = find_workflow(&workflows, &send_to).expect("Workflow not found");
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
                    let i = find_workflow(&workflows, &send_to).expect("Workflow not found");
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
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    pub fn new(name: String, rules: Vec<Rule>) -> Self {
        Self { name, rules }
    }
}

type WorkflowName = String;

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
            Rule::Send(send_to) => format!("Send to {send_to}"),
            Rule::Accepted => "Accepted".to_string(),
            Rule::Rejected => "Rejected".to_string()
        };
        formatter.write_str(&str)
    }
}
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
    let (workflows, _) = parse_workflows(&lines);
    let st_index = find_workflow(&workflows, "in").expect("Nowhere to start");
    println!("Possible combinations: {}", max_possible_combinations());
    let mut current_workflow = &workflows[st_index];
    let mut combinations = 0;
    for rule in &current_workflow.rules {

    }
    combinations
}
pub fn max_possible_combinations() -> usize {
    MAX_VALUE * MAX_VALUE * MAX_VALUE * MAX_VALUE
}

