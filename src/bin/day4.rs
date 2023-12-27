use std::collections::HashMap;

#[derive(Debug)]
struct Card {
    num: u32,
    winners: Vec<u32>,
    numbers: Vec<u32>,
}

impl FromIterator<Card> for HashMap<u32, Card> {
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
        let mut ret = HashMap::new();
        for c in iter {
            ret.insert(c.num, c);
        }
        ret
    }
}

impl Card {
    fn parse(line: &str) -> Card {
        let mut parts = line.split(":");
        let header = parts.next().unwrap().split_whitespace();
        let num = header.skip(1).next().unwrap().parse().unwrap();

        let mut numbers = parts.next().unwrap().split("|");

        let winners = numbers
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let numbers = numbers
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        Card {
            num,
            winners,
            numbers,
        }
    }

    fn get_num_winners(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|n| self.winners.contains(n))
            .collect::<Vec<_>>()
            .len() as u32
    }

    // 0 1 2 4 8 16 32
    // 0 1 2 3 4 5 6
    // 2^(n - 1)
    fn get_points(&self) -> u32 {
        match self.get_num_winners() {
            0 => 0,
            n => 2_u32.pow(n - 1),
        }
    }
}

fn process_a(data: &str) -> u32 {
    data.lines()
        .map(|line| Card::parse(line))
        .map(|card| card.get_points())
        .sum::<u32>()
}

fn process_b(data: &str) -> u32 {
    let mut cards: HashMap<u32, Card> = data.lines().map(|line| Card::parse(line)).collect();
    for (k, c) in cards {
        print!("key: {}, val: {:?}\n", k, c);
    }
    0
}

fn main() {
    let data = std::fs::read_to_string("data/day4_test.txt").unwrap();

    let result = process_a(&data);
    assert_eq!(result, 30);

    let result = process_b(&data);
    assert_eq!(result, 30);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testicle_1() {
        let data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let result = process_a(data);
        assert_eq!(result, 8);
    }

    #[test]
    fn testicle_2() {
        let data = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        let result = process_a(data);
        assert_eq!(result, 2);
    }

    #[test]
    fn testicle_3() {
        let data = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        let result = process_a(data);
        assert_eq!(result, 2);
    }

    #[test]
    fn testicle_4() {
        let data = "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";
        let result = process_a(data);
        assert_eq!(result, 1);
    }

    #[test]
    fn testicle_5() {
        let data = "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36";
        let result = process_a(data);
        assert_eq!(result, 0);
    }

    #[test]
    fn testicle_6() {
        let data = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = process_a(data);
        assert_eq!(result, 0);
    }

    #[test]
    fn testicle() {
        let data = std::fs::read_to_string("data/day4_test.txt").unwrap();
        let result = process_a(&data);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_a() {
        let data = std::fs::read_to_string("data/day4.txt").unwrap();
        let result = process_a(&data);
        assert_eq!(result, 17782);
    }

    // #[test]
    // fn testicle_b() {
    //     let data = std::fs::read_to_string("data/day4_test.txt").unwrap();
    //     let result = process_b(&data);
    //     assert_eq!(result, 30);
    // }
}
