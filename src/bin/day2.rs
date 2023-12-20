#[derive(Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    fn new(red: u32, green: u32, blue: u32) -> Round {
        Round { red, green, blue }
    }

    fn parse(section: &str) -> Round {
        Round {
            red: 0,
            green: 0,
            blue: 0,
        }
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
            rounds.push(
                Round {
                    red: get_colour(round_string, "red"),
                    green: get_colour(round_string, "green"),
                    blue: get_colour(round_string, "blue"),
                }
            );
        }

        Some(Game {
            id,
            rounds,
        })
    }
}

fn get_colour(source: &str, colour: &str) -> u32 {
    for colour_string in source.split(",") {
        let mut colour_parts = colour_string.split_whitespace();
        let num: u32 = colour_parts.next().unwrap().parse().unwrap();
        let found_colour = colour_parts.next().unwrap().trim();
        if colour == found_colour {
            return num;
        }
    }
    0
}

fn main() {}

#[cfg(test)]
mod test {
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
        let mut games = vec![];
        for line in data.lines() {
            games.push(Game::parse(&line));
        }
        
        print!("{:?}", games);
    }
}
