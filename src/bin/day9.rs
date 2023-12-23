fn get_difference_sequence(sequence: &Vec<i64>) -> Vec<i64> {
    sequence.windows(2).map(|w| w[1] - w[0]).collect()
}

fn get_next_value(line: &str) -> i64 {
    let starting_sequence: Vec<i64> = line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut sequences = vec![starting_sequence];

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
    input.lines().map(|l| get_next_value(l)).sum()
}

fn main() {
    let data = std::fs::read_to_string("data/day9.txt").unwrap();
    let result = process(&data);
    print!("Result 1 is : {}\n", result);
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
}
