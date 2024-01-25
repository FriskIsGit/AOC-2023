use std::fmt::Display;

pub fn scratchcards1(lines: Vec<String>) -> usize {
    let mut total_score = 0;
    for line in lines {
        let colon = line.find(':').expect("Every line should contain a colon");
        let vertical_bar = line.find('|').expect("Every line should contain a vertical bar");
        let left_slice = &line[colon + 1..vertical_bar - 1];
        let right_slice = &line[vertical_bar + 1..line.len()];

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

pub fn scratchcards2(lines: Vec<String>) -> usize {
    let mut card_id = 0;
    // parse
    let mut cards: Vec<Card> = Vec::with_capacity(lines.len());
    cards.push(Card::empty(0));
    for line in lines {
        card_id += 1;
        let colon = line.find(':').expect("Every line should contain a colon");
        let vertical_bar = line.find('|').expect("Every line should contain a vertical bar");
        let left_slice = &line[colon + 1..vertical_bar - 1];
        let right_slice = &line[vertical_bar + 1..line.len()];

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
    // let borrow_checker_is_fucking_stupid = cards.len() - 1;
    // let scratchcards = process_cards(&cards, borrow_checker_is_fucking_stupid, 0);
    let scratchcards = OptimizedSolution::new(cards).calculate_wins();
    scratchcards
}

struct OptimizedSolution {
    cards: Vec<Card>,
    card_calculated: Vec<bool>,
    card_wins: Vec<usize>
}

impl OptimizedSolution {
    pub fn new(cards: Vec<Card>) -> Self {
        // Each card's id is also the index (this is already reflected in 'cards')
        let card_calculated: Vec<bool> = vec![false; cards.len()];
        let card_wins: Vec<usize> = vec![0; cards.len()];
        Self { cards, card_calculated, card_wins }
    }

    pub fn calculate_wins(&mut self) -> usize {
        self.count_cards(self.cards.len()-1, 0)
    }

    // this will also fill missing data
    fn count_cards(&mut self, next_count: usize, at_id: usize) -> usize {
        let mut scratchcards = next_count;
        for id in at_id+1..=at_id + next_count {
            if self.card_calculated[id] {
                scratchcards += self.card_wins[id];
                continue
            }
            let matches = self.cards[id].get_matches();
            let sub_cards = self.count_cards(matches, id);
            // Sub results are applied to the current element
            self.card_calculated[id] = true;
            self.card_wins[id] = sub_cards;

            scratchcards += sub_cards;
        }
        scratchcards
    }
}

pub fn process_cards(cards: &Vec<Card>, next: usize, at_id: usize) -> usize {
    let mut scratchcards = next;
    for i in at_id + 1..=at_id + next {
        let matches = cards[i].get_matches();
        scratchcards += process_cards(cards, matches, i);
    }
    scratchcards
}

pub struct Card {
    id: usize,
    matches: usize,
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
}

impl Card {
    pub fn new(id: usize, winning_numbers: Vec<usize>, numbers: Vec<usize>) -> Self {
        let matches = Self::calculate_matches(&winning_numbers, &numbers);
        Self {
            id,
            matches,
            winning_numbers,
            numbers,
        }
    }
    pub fn empty(id: usize) -> Self {
        Self {
            id,
            matches: 0,
            winning_numbers: vec![],
            numbers: vec![],
        }
    }
    pub fn calculate_matches(winning_numbers: &Vec<usize>, numbers: &Vec<usize>) -> usize {
        let mut matches = 0;
        for winning_num in winning_numbers {
            for num in numbers {
                if winning_num == num {
                    matches += 1;
                }
            }
        }
        matches
    }
    pub fn get_matches(&self) -> usize {
        self.matches
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let format = format!("{} {:?} | {:?}", self.id, self.winning_numbers, self.numbers);
        f.write_str(&format)
    }
}