use std::{cmp::Ordering, collections::HashMap, iter::zip};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandStrength {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Clone, Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: usize,
}

impl Hand {
    fn parse(line: &str) -> Hand {
        let mut tokens = line.split_whitespace();
        let cards: Vec<Card> = tokens
            .next()
            .unwrap()
            .chars()
            .map(|c| Card::new(c))
            .collect();
        let bid = tokens.next().unwrap().parse().unwrap();
        Hand { cards, bid }
    }

    fn get_num_distinct_cards(&self) -> usize {
        let mut cards_clone = self.cards.clone();
        cards_clone.sort();
        cards_clone.dedup();
        cards_clone.len()
    }

    fn get_max_matching_cards(&self) -> usize {
        let mut result = HashMap::new();

        for item in self.cards.iter() {
            *result.entry(item).or_insert(0) += 1;
        }

        *result.values().max().unwrap()
    }

    fn get_strength(&self) -> HandStrength {
        match self.get_num_distinct_cards() {
            1 => HandStrength::FiveOfAKind,
            2 => match self.get_max_matching_cards() {
                4 => HandStrength::FourOfAKind,
                3 => HandStrength::FullHouse,
                _ => panic!("What happen?!"),
            },
            3 => match self.get_max_matching_cards() {
                3 => HandStrength::ThreeOfAKind,
                2 => HandStrength::TwoPairs,
                _ => panic!("What happen?!"),
            },
            4 => HandStrength::OnePair,
            5 => HandStrength::HighCard,
            _ => panic!("Invalid hand!"),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cards == other.cards {
            return Ordering::Equal;
        }

        match self.get_strength().cmp(&other.get_strength()) {
            Ordering::Equal => {
                for (c1, c2) in zip(&self.cards, &other.cards) {
                    if c1 != c2 {
                        return c1.cmp(&c2);
                    }
                }
                panic!("Unexpected panda in the bagging area!\n");
            }
            res => res,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Card {
    strength: u32,
}

impl Card {
    fn new(card: char) -> Card {
        let strength = match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            x => x.to_digit(10).unwrap(),
        };
        Card { strength }
    }
}

fn process(input: &str) -> usize {
    let mut hands: Vec<Hand> = input.lines().map(|l| Hand::parse(l)).collect();

    // print!("hands! {:?}\n", hands);

    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid)
        .sum()
}

fn main() {
    let data = std::fs::read_to_string("data/day7_test.txt").unwrap();
    let result = process(&data);
    print!("Result is {}\n", result);

    let data = std::fs::read_to_string("data/day7.txt").unwrap();
    let result = process(&data);
    print!("Result is {}\n", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_card_new() {
        assert_eq!(Card::new('A').strength, 14);
        assert_eq!(Card::new('K').strength, 13);
        assert_eq!(Card::new('Q').strength, 12);
        assert_eq!(Card::new('J').strength, 11);
        assert_eq!(Card::new('T').strength, 10);
        assert_eq!(Card::new('9').strength, 9);
        assert_eq!(Card::new('1').strength, 1);
    }

    #[test]
    fn testicle() {
        let data = std::fs::read_to_string("data/day7_test.txt").unwrap();
        let result = process(&data);
        assert_eq!(result, 6440);
    }

    #[test]
    fn test() {
        let data = std::fs::read_to_string("data/day7.txt").unwrap();
        let result = process(&data);
        assert_eq!(result, 248812215);
    }
}
