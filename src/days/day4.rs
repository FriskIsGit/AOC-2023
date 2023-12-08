use std::fmt::Display;

pub fn scratchcards1(lines: Vec<String>) -> usize {
    let mut total_score = 0;
    for line in lines {
        let colon = line.find(':').expect("Every line should contain a colon");
        let vertical_bar = line.find('|').expect("Every line should contain a vertical bar");
        let left_slice = &line[colon+1..vertical_bar-1];
        let right_slice = &line[vertical_bar+1..line.len()];

        let left_split = left_slice.split(' ');
        let right_split = right_slice.split(' ');
        let mut numbers_left: Vec<usize> = Vec::with_capacity(10);
        let mut numbers_right: Vec<usize> = Vec::with_capacity(25);
        for token in left_split {
            if token.trim().is_empty() {
                continue;
            }
            let number = token.parse::<usize>().unwrap();
            numbers_left.push(number);
        }
        for token in right_split {
            if token.trim().is_empty() {
                continue;
            }
            let number = token.parse::<usize>().unwrap();
            numbers_right.push(number);
        }
        let mut score = 0;
        let mut first_match = true;
        for winning_num in numbers_left {
            for num in &numbers_right {
                if winning_num != *num {
                    continue;
                }
                if first_match {
                    first_match = false;
                    score += 1;
                    continue;
                }
                score *= 2;
            }
        }
        total_score += score;
    }
    total_score
}

// TODO: Switch to a stack-based implementation
pub fn scratchcards2(lines: Vec<String>) -> usize {
    let mut card_id = 0;
    // parse
    let mut cards: Vec<Card> = Vec::with_capacity(lines.len());
    cards.push(Card::empty(0));
    for line in lines {
        card_id += 1;
        let colon = line.find(':').expect("Every line should contain a colon");
        let vertical_bar = line.find('|').expect("Every line should contain a vertical bar");
        let left_slice = &line[colon+1..vertical_bar-1];
        let right_slice = &line[vertical_bar+1..line.len()];

        let left_split = left_slice.split(' ');
        let right_split = right_slice.split(' ');
        let mut numbers_left: Vec<usize> = Vec::with_capacity(10);
        let mut numbers_right: Vec<usize> = Vec::with_capacity(25);
        for token in left_split {
            if token.trim().is_empty() {
                continue;
            }
            let number = token.parse::<usize>().unwrap();
            numbers_left.push(number);
        }
        for token in right_split {
            if token.trim().is_empty() {
                continue;
            }
            let number = token.parse::<usize>().unwrap();
            numbers_right.push(number);
        }
        let card = Card::new(card_id, numbers_left, numbers_right);
        cards.push(card)
    }
    let borrow_checker_is_fucking_stupid = cards.len() - 1;
    let scratchcards = process_cards(&mut cards, borrow_checker_is_fucking_stupid, 0);
    scratchcards
}
pub fn process_cards(cards: &mut Vec<Card>, next: usize, at_id: usize) -> usize {
    let mut scratchcards = 0;
    scratchcards += next;
    for i in at_id + 1..at_id + 1 + next {
        let card = &mut cards[i];
        let matches = card.get_matches();
        scratchcards += process_cards(cards, matches, i);
    }
    scratchcards
}

pub struct Card {
    id: usize,
    matches: Option<usize>,
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
}
impl Card {
    pub fn new(id: usize, winning_numbers: Vec<usize>, numbers: Vec<usize>) -> Self {
        Self{
            id,
            matches: None,
            winning_numbers,
            numbers,
        }
    }
    pub fn empty(id: usize) -> Self {
        Self{
            id,
            matches: None,
            winning_numbers: vec![],
            numbers: vec![]
        }
    }
    pub fn get_matches(&mut self) -> usize {
        if self.matches.is_some() {
            return self.matches.unwrap();
        }
        let mut matches = 0;
        for winning_num in &self.winning_numbers {
            for num in &self.numbers {
                if *winning_num == *num {
                    matches += 1;
                }
            }
        }
        self.matches = Some(matches);
        matches
    }
}
impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let format = format!("{} {:?} | {:?}", self.id, self.winning_numbers, self.numbers);
        f.write_str(&format)
    }
}