fn process(input: &str) -> u32 {
    let times = input
        .lines()
        .next()
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap());

    let distances = input
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap());

    let data: Vec<(u32, u32)> = times.zip(distances).collect();
    print!("data: {:?}\n", data);
    0
}

fn main() {
    let data = std::fs::read_to_string("data/day6_test.txt").unwrap();

    let result = process(&data);
    print!("Result a is: {}\n", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testicle() {
        let data = std::fs::read_to_string("data/day5_test.txt").unwrap();
        let result = process(&data);
        assert_eq!(result, 288);
    }
}
