
pub fn aplenty1(lines: Vec<String>) -> usize {
    let (workflows, ratings) = parse_input(lines);
    0
}

fn parse_input(mut lines: Vec<String>) -> (Vec<Workflow>, Vec<Vec<Rating>>) {
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
    let mut ratings: Vec<Vec<Rating>> = vec![];
    let mut rating_index = 0;
    for line in lines.iter().skip(empty_index + 1) {
        ratings.push(vec![]);
        let raw_ratings: Vec<&str> = line[1..line.len()-1].split(',').collect();
        for raw_rating in raw_ratings {
            let category = raw_rating.bytes().next().unwrap();
            let value = raw_rating[2..].parse::<usize>().unwrap();
            let rating = Rating::new(category, value);
            ratings[rating_index].push(rating)
        }
        rating_index += 1;
    }
    (workflows, ratings)
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

pub struct Condition {
    category: u8,
    is_more: bool,
    const_value: usize,
    send_to: String,
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
}
