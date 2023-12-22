#[derive(Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn parse(line: &str) -> Option<Game> {
        let mut parts = line.split(":");

        let mut game_parts = parts.next().unwrap().split_whitespace();
        game_parts.next();
        let id: u32 = game_parts.next().unwrap().parse().unwrap();

        let remains = parts.next().unwrap();
        let mut rounds = vec![];

        for round_string in remains.split(";") {
            rounds.push(Round {
                red: get_colour(round_string, "red"),
                green: get_colour(round_string, "green"),
                blue: get_colour(round_string, "blue"),
            });
        }

        Some(Game { id, rounds })
    }

    fn is_possible(&self, red: u32, green: u32, blue: u32) -> bool {
        for r in &self.rounds {
            if r.red > red || r.green > green || r.blue > blue {
                return false;
            }
        }
        true
    }

    fn get_minima(&self) -> Round {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for r in &self.rounds {
            if r.red > red {
                red = r.red;
            }
            if r.green > green {
                green = r.green;
            }
            if r.blue > blue {
                blue = r.blue;
            }
        }
        Round { red, green, blue }
    }
}

fn get_colour(source: &str, colour: &str) -> u32 {
    for colour_string in source.split(",") {
        let mut colour_parts = colour_string.split_whitespace();
        let num = colour_parts.next().unwrap().parse().unwrap();
        let found_colour = colour_parts.next().unwrap().trim();
        if colour == found_colour {
            return num;
        }
    }
    0
}

fn process(data: &str) -> u32 {
    data.lines()
        .filter_map(|line| Game::parse(&line))
        .filter_map(|game| {
            if game.is_possible(12, 13, 14) {
                Some(game.id)
            } else {
                None
            }
        })
        .sum()
}

fn process_b(data: &str) -> u32 {
    data.lines()
        .filter_map(|line| Game::parse(&line))
        .map(|game| game.get_minima())
        .map(|round| round.power())
        .sum()
}

fn main() {
    let data = std::fs::read_to_string("data/day2.txt").unwrap();

    let sum = process(&data);
    print!("The answer is {}\n", sum);

    let power_sum = process_b(&data);
    print!("The second answer is {}\n", power_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_parse_one() {
        let data = "Game 1: 9 red, 5 blue, 6 green; 6 red, 13 blue; 2 blue, 7 green, 5 red";
        let game = Game::parse(&data).unwrap();

        assert_eq!(game.id, 1);
        assert_eq!(game.rounds.len(), 3);

        assert_eq!(game.rounds[0].red, 9);
        assert_eq!(game.rounds[0].green, 6);
        assert_eq!(game.rounds[0].blue, 5);

        assert_eq!(game.rounds[1].red, 6);
        assert_eq!(game.rounds[1].green, 0);
        assert_eq!(game.rounds[1].blue, 13);

        assert_eq!(game.rounds[2].red, 5);
        assert_eq!(game.rounds[2].green, 7);
        assert_eq!(game.rounds[2].blue, 2);
    }

    #[test]
    fn test_example_data() {
        let data = std::fs::read_to_string("data/day2_test.txt").unwrap();
        let sum = process(&data);

        assert_eq!(sum, 8);
    }

    #[test]
    fn test_day2_data() {
        let data = std::fs::read_to_string("data/day2.txt").unwrap();
        let sum = process(&data);

        assert_eq!(sum, 2278);
    }

    #[test]
    fn test_example_data_b() {
        let data = std::fs::read_to_string("data/day2_test.txt").unwrap();
        let power_sum = process_b(&data);

        assert_eq!(power_sum, 2286);
    }

    #[test]
    fn test_day2_data_b() {
        let data = std::fs::read_to_string("data/day2.txt").unwrap();
        let sum = process_b(&data);

        assert_eq!(sum, 67953);
    }
}
