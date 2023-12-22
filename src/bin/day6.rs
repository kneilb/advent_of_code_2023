use std::iter::zip;

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn calc_ways_to_win(&self) -> usize {
        let mut possibilities: Vec<usize> = vec![];

        for hold_time in 0..self.time {
            let speed = hold_time;
            let remaining_time = self.time - hold_time;
            let distance = speed * remaining_time;
            print!(
                "holding for {}; remaining {}, speed {}, distance {}\n",
                hold_time, remaining_time, speed, distance
            );
            if distance > self.distance {
                possibilities.push(hold_time);
                print!("It worked...!\n");
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
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

fn process(input: &str) -> usize {
    parse_races(input)
        .iter()
        .map(|r| r.calc_ways_to_win())
        .product()
}

fn main() {
    let data = std::fs::read_to_string("data/day6.txt").unwrap();
    let result = process(&data);
    print!("Result a is: {}\n", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testicle() {
        let data = std::fs::read_to_string("data/day6_test.txt").unwrap();
        let result = process(&data);
        assert_eq!(result, 288);
    }
}
