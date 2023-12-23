fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|x| x.parse().unwrap()).collect())
        .collect()
}

fn get_difference_sequence(sequence: &Vec<i64>) -> Vec<i64> {
    sequence.windows(2).map(|w| w[1] - w[0]).collect()
}

fn get_next_value(sequence: Vec<i64>) -> i64 {
    let mut sequences = vec![sequence];

    loop {
        let next_seq = get_difference_sequence(&sequences.last().unwrap());
        if next_seq.iter().all(|x| *x == 0) {
            break;
        }
        sequences.push(next_seq);
    }

    let mut predictions = vec![0];

    for s in sequences.iter().rev() {
        predictions.push(s.last().unwrap() + predictions.last().unwrap());
    }

    // print!("Seq: {:?}\n", sequences);
    // print!("Predictions: {:?}\n", predictions);

    *predictions.last().unwrap()
}

fn process(input: &str) -> i64 {
    let sequences = parse_input(input);
    sequences.iter().map(|s| get_next_value(s.to_owned())).sum()
}

fn process_b(input: &str) -> i64 {
    let mut sequences = parse_input(input);
    sequences
        .iter_mut()
        .map(|s| {
            s.reverse();
            get_next_value(s.to_owned())
        })
        .sum()
}

fn main() {
    let data = std::fs::read_to_string("data/day9.txt").unwrap();

    let result_1 = process(&data);
    print!("Result 1 is : {}\n", result_1);

    let result_2 = process_b(&data);
    print!("Result 2 is : {}\n", result_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testicle() {
        let data = std::fs::read_to_string("data/day9_test_1.txt").unwrap();
        let result = process(&data);
        assert_eq!(result, 114);
    }

    #[test]
    fn test() {
        let data = std::fs::read_to_string("data/day9.txt").unwrap();
        let result = process(&data);
        assert_eq!(result, 1939607039);
    }

    #[test]
    fn testicle_b() {
        let data = std::fs::read_to_string("data/day9_test_1.txt").unwrap();
        let result = process_b(&data);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_b() {
        let data = std::fs::read_to_string("data/day9.txt").unwrap();
        let result = process_b(&data);
        assert_eq!(result, 1041);
    }
}
