fn day1(input: &str) -> u32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let test_data = std::fs::read_to_string("data/day1_test.txt").unwrap();
        let result = day1(&test_data);
        assert_eq!(result, 142);
    }

    #[test]
    fn real_input() {
        let test_data = std::fs::read_to_string("data/day1.txt").unwrap();
        let result = day1(&test_data);
        assert_eq!(result, 54561);
    }
}
