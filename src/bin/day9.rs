fn get_difference_sequence(sequence: &Vec<u32>) -> Vec<u32> {
    sequence.windows(2).map(|w| w[1] - w[0]).collect()
}

fn get_next_value(line: &str) -> u32 {
    let starting_sequence: Vec<u32> = line
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

    let extended_sequences: Vec<Vec<u32>> = sequences
        .iter()
        .rev()
        .collect::<Vec<&Vec<u32>>>()
        .windows(2)
        .map(|w| {
        This doesn't work, because we don't get the modified version as our next input...
            let diff = w[0].last().unwrap();
            let last = w[1].last().unwrap();
            let mut new = w[1].clone();
            new.push(last + diff);
            new
        })
        .collect();

    let mut predictions = vec![0];


    print!("Seq: {:?}\n", sequences);
    print!("Extended: {:?}\n", extended_sequences);
    *extended_sequences.last().unwrap().last().unwrap()
}

fn process(input: &str) -> u32 {
    input.lines().map(|l| get_next_value(l)).sum()
}

fn main() {
    let data = std::fs::read_to_string("data/day9_test_1.txt").unwrap();
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
        assert_eq!(result, 0);
    }
}
