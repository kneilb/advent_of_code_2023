#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
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
}

#[derive(Debug, PartialEq)]
struct Card {
    card: char,
    strength: u32,
}

impl Card {
    fn new(card: char) -> Card {
        let strength = match card {
            'A' => Some(14),
            'K' => Some(13),
            'Q' => Some(12),
            'J' => Some(11),
            'T' => Some(10),
            x => x.to_digit(10),
        }
        .unwrap();
        Card { card, strength }
    }
}

fn process(input: &str) -> u32 {
    let hands: Vec<Hand> = input.lines().map(|l| Hand::parse(l)).collect();
    print!("hands! {:?}\n", hands);
    0
}

fn main() {
    let data = std::fs::read_to_string("data/day7_test.txt").unwrap();
    let result = process(&data);
    print!("Result is {}\n", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_card_new() {
        assert_eq!(
            Card::new('A'),
            Card {
                card: 'A',
                strength: 14
            }
        );
        assert_eq!(
            Card::new('K'),
            Card {
                card: 'K',
                strength: 13
            }
        );
        assert_eq!(
            Card::new('Q'),
            Card {
                card: 'Q',
                strength: 12
            }
        );
        assert_eq!(
            Card::new('J'),
            Card {
                card: 'J',
                strength: 11
            }
        );
        assert_eq!(
            Card::new('T'),
            Card {
                card: 'T',
                strength: 10
            }
        );
        assert_eq!(
            Card::new('9'),
            Card {
                card: '9',
                strength: 9
            }
        );
        assert_eq!(
            Card::new('1'),
            Card {
                card: '1',
                strength: 1
            }
        );
        assert_eq!(
            Card::new('0'),
            Card {
                card: '0',
                strength: 0
            }
        );
    }

    #[test]
    fn testicle() {
        let data = std::fs::read_to_string("data/day7_test.txt").unwrap();
        let result = process(&data);
        assert_eq!(result, 6440);
    }
}
