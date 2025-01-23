fn process(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|char| char.to_digit(10));
            let first = it.next().unwrap_or(0);

            let last = it.last().unwrap_or(first);

            first * 10 + last
        })
        .sum()
}

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn string_to_digit(input: &str) -> String {
    let mut res = String::new();

    for i in 0..input.len() {
        let slice = &input[i..];
        let mut found = 0;
        for (val, num) in NUMBERS.iter().enumerate() {
            if slice.starts_with(num) {
                found = val + 1;
            }
        }
        if found != 0 {
            res = format!("{}{}", res, found);
        } else {
            res = format!("{}{}", res, slice.chars().next().unwrap());
        }
    }

    res
}

fn process_b(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let line = string_to_digit(line);
            let mut it = line.chars().filter_map(|char| char.to_digit(10));
            let first = it.next().unwrap_or(0);

            let last = it.last().unwrap_or(first);

            first * 10 + last
        })
        .sum()
}

fn main() {
    let test_data = std::fs::read_to_string("data/day1.txt").unwrap();
    let result = process(&test_data);
    assert_eq!(result, 54561);

    let result = process_b(&test_data);
    assert_eq!(result, 54076);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let test_data = std::fs::read_to_string("data/day1_test.txt").unwrap();
        let result = process(&test_data);
        assert_eq!(result, 142);
    }

    #[test]
    fn real_input() {
        let test_data = std::fs::read_to_string("data/day1.txt").unwrap();
        let result = process(&test_data);
        assert_eq!(result, 54561);
    }

    #[test]
    fn test_input_b() {
        let test_data = std::fs::read_to_string("data/day1b_test.txt").unwrap();
        let result = process_b(&test_data);
        assert_eq!(result, 281);
    }

    #[test]
    fn real_input_b() {
        let test_data = std::fs::read_to_string("data/day1.txt").unwrap();
        let result = process_b(&test_data);
        assert_eq!(result, 54076);
    }
}
