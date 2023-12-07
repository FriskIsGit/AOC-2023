use std::collections::HashMap;

pub fn camel_cards1(lines: Vec<String>) -> usize {
    let mut hands = parse_hands(lines);
    // sorting ascendingly because rank 1 is the worst (unintuitive)
    sort_hands(&mut hands);
    let mut rank = 0;
    let mut sum = 0;
    for hand in hands {
        rank += 1;
        println!("SORTED HAND rank:{rank}, cards:{}", hand.cards);
        sum += rank * hand.bid
    }
    sum
}

pub fn camel_cards2(lines: Vec<String>) -> usize {
    let mut hands = parse_hands(lines);
    for hand in hands.iter_mut() {
        if hand.has_joker() && hand.kind_strength < 7 {
            
        }
    }
    sort_hands(&mut hands);
    let mut rank = 0;
    let mut sum = 0;
    for hand in hands {
        rank += 1;
        println!("SORTED HAND rank:{rank}, cards:{}", hand.cards);
        sum += rank * hand.bid
    }
    sum
}

// sorting ascendingly because unintuitively rank 1 is the worst
fn sort_hands(hands: &mut Vec<Hand>) {
    hands.sort_by(|hand1, hand2| {
        let strength1 = hand1.kind_strength;
        let strength2 = hand2.kind_strength;
        if strength1 != strength2 {
            return strength1.cmp(&strength2);
        }
        let mut left_iter = hand1.cards.chars();
        let mut right_iter = hand2.cards.chars();
        for _i in 0..5 {
            let left_chr = left_iter.next().unwrap();
            let right_chr = right_iter.next().unwrap();
            let strength_left = left_chr.strength();
            let strength_right = right_chr.strength();
            if strength_left == strength_right {
                continue;
            }
            return strength_left.cmp(&strength_right);
        }
        panic!("Unreachable comparison")
    });
}

fn parse_hands(lines: Vec<String>) -> Vec<Hand> {
    let mut hands: Vec<Hand> = vec![];
    for mut line in lines {
        let cards_end = line.find(' ').unwrap();
        let bid_split = line.split_off(cards_end + 1);
        let cards = line[0..cards_end].to_owned();
        let bid = bid_split.parse::<usize>().expect("Failed to parse number");

        let hand = Hand::new(cards, bid);
        hands.push(hand);
    }
    hands
}

struct Hand {
    kind_strength: usize,
    cards: String,
    bid: usize,
}

impl Hand {
    pub fn new(cards: String, bid: usize) -> Self {
        Self {
            kind_strength: Self::determine_kind_strength(&cards),
            cards,
            bid
        }
    }
    pub fn has_joker(&self) -> bool {
        for label in self.cards.bytes() {
            if label == b'J' {
                return true
            }
        }
        false
    }
    pub fn determine_kind_strength(cards: &str) -> usize {
        let mut character_map = HashMap::<char, usize>::with_capacity(5);
        for chr in cards.chars() {
            if let Some(repeats) = character_map.get(&chr) {
                character_map.insert(chr, repeats + 1);
            } else {
                character_map.insert(chr, 1);
            }
        }
        if character_map.len() == 1 {
            return 7; // AAAAA Five of a kind,
        }
        let mut occurrences: Vec<usize> = vec![];
        for val in character_map.values() {
            occurrences.push(*val);
        }
        occurrences.sort_by(|a, b| b.cmp(a));
        if occurrences.len() == 2 && occurrences[0] == 4 && occurrences[1] == 1 {
            return 6; // AA8AA Four of a kind
        }
        if occurrences.len() == 2 && occurrences[0] == 3 && occurrences[1] == 2 {
            return 5; // 23332 Full house
        }
        if occurrences.len() == 3 && occurrences[0] == 3 && occurrences[1] == occurrences[2] {
            return 4; // TTT98 Three of a kind
        }
        if occurrences.len() == 3 && occurrences[0] == 2 && occurrences[1] == 2 && occurrences[2] == 1 {
            return 3; // 23432 Two pair
        }
        if occurrences.len() == 4 && occurrences[0] == 2
            && occurrences[1] == 1 && occurrences[2] == 1 && occurrences[3] == 1 {
            return 2; // A23A4 One pair
        }
        1 // 23456 High card
    }
}

impl LabelStrength for char {
    fn strength(&self) -> usize {
        return match self {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            '2'..='9' => *self as usize - 48,
            _ => panic!("This character has no strength assigned to it")
        };
    }
    fn strength_with_joker(&self) -> usize {
        return match self {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'T' => 10,
            '2'..='9' => *self as usize - 48,
            'J' => 1,
            _ => panic!("This character has no strength assigned to it")
        };
    }
}


trait LabelStrength {
    fn strength(&self) -> usize;
    fn strength_with_joker(&self) -> usize;
}
