use std::iter::zip;

#[derive(Debug)]
struct Race {
    time: usize,
    record_distance: usize,
}

impl Race {
    fn calc_ways_to_win(&self) -> usize {
        let mut possibilities: Vec<usize> = vec![];

        for hold_time in 0..self.time {
            let speed = hold_time;
            let remaining_time = self.time - hold_time;
            let distance = speed * remaining_time;

            if distance > self.record_distance {
                possibilities.push(hold_time);
            }
        }
        possibilities.len()
    }
}

fn parse_numbers(line: &str) -> Vec<usize> {
    line.split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn parse_races(input: &str) -> Vec<Race> {
    let times = parse_numbers(input.lines().nth(0).unwrap());
    let distances = parse_numbers(input.lines().nth(1).unwrap());

    zip(times, distances)
        .map(|(time, record_distance)| Race {
            time,
            record_distance,
        })
        .collect()
}

fn process(input: &str) -> usize {
    parse_races(input)
        .iter()
        .map(|r| r.calc_ways_to_win())
        .product()
}

fn parse_races_b(input: &str) -> Vec<Race> {
    let times = parse_numbers(&input.lines().nth(0).unwrap().replace(' ', ""));
    let distances = parse_numbers(&input.lines().nth(1).unwrap().replace(' ', ""));

    zip(times, distances)
        .map(|(time, record_distance)| Race {
            time,
            record_distance,
        })
        .collect()
}

fn process_b(input: &str) -> usize {
    parse_races_b(input)
        .iter()
        .map(|r| r.calc_ways_to_win())
        .product()
}

fn main() {
    let data = std::fs::read_to_string("data/day6.txt").unwrap();

    let result = process(&data);
    print!("Result is: {}\n", result);

    let result_b = process_b(&data);
    print!("Result b is: {}\n", result_b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testicle() {
        let data = std::fs::read_to_string("data/day6_test.txt").unwrap();
        let result = process(&data);
        assert_eq!(result, 288);
    }

    #[test]
    fn test() {
        let data = std::fs::read_to_string("data/day6.txt").unwrap();
        let result = process(&data);
        assert_eq!(result, 74698);
    }

    #[test]
    fn testicle_b() {
        let data = std::fs::read_to_string("data/day6_test.txt").unwrap();
        let result = process_b(&data);
        assert_eq!(result, 71503);
    }

    #[test]
    fn test_b() {
        let data = std::fs::read_to_string("data/day6.txt").unwrap();
        let result = process_b(&data);
        assert_eq!(result, 27563421);
    }
}
