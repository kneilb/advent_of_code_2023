fn process(input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testicle_1() {
        let data = std::fs::read_to_string("data/day8_test_1.txt").unwrap();
        let result = process(&data);
        assert_eq!(result, 2);
    }

    #[test]
    fn testicle_2() {
        let data = std::fs::read_to_string("data/day8_test_2.txt").unwrap();
        let result = process(&data);
        assert_eq!(result, 6);
    }
}