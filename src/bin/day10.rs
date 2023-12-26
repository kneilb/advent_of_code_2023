#[derive(Debug)]
enum Tile {
    NorthAndSouth,
    EastAndWest,
    NorthAndEast,
    NorthAndWest,
    SouthAndWest,
    SouthAndEast,
    Ground,
    Start,
}

impl TryFrom<&char> for Tile {
    type Error = String;

    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Tile::NorthAndSouth),
            '-' => Ok(Tile::EastAndWest),
            'L' => Ok(Tile::NorthAndEast),
            'J' => Ok(Tile::NorthAndWest),
            '7' => Ok(Tile::SouthAndWest),
            'F' => Ok(Tile::SouthAndEast),
            '.' => Ok(Tile::Ground),
            'S' => Ok(Tile::Start),
            _ => Err("Oh dear".to_owned()),
        }
    }
}

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<Tile>>,
}

fn parse_input(input: &str) -> Map {
    let grid = input
        .lines()
        .map(|l| l.chars().map(|c| Tile::try_from(&c).unwrap()).collect())
        .collect();
    Map { grid }
}

fn process(input: &str) -> usize {
    let map = parse_input(input);
    print!("map is: {:?}\n", map);
    0
}

fn main() {
    let input = std::fs::read_to_string("data/day10_test_1.txt").unwrap();
    let result = process(&input);
    print!("Result is {}\n", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testicle_1() {
        let input = std::fs::read_to_string("data/day10_test_1.txt").unwrap();
        let result = process(&input);
        assert_eq!(result, 0);
    }
}
