struct Card {
    num: u32,
    winners: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn parse(line: &str) -> Card {
        let num = 0;
        let mut parts = line.split(":");
        parts.next();
        let part_two = parts.next().unwrap();

        let mut things = part_two.split("|");

        let winners = things.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let numbers = things.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();

        Card {
            num,
            winners,
            numbers,
        }
    }

    fn get_points(&self) -> u32 {
        let mut points = 0;
        for n in &self.numbers {
            if self.winners.contains(&n) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }
        points
    }
}

fn process_a(data: &str) -> u32 {
    let mut cards: Vec<Card> = vec![];
    for line in data.lines() {
        cards.push(Card::parse(line));
    }

    data.lines().map(|line| Card::parse(line)).map(|card| card.get_points()).sum()
}

#[cfg(test)]
mod test {
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
}
