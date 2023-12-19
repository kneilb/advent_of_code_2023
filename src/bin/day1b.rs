mod day1;
use crate::day1::REAL_INPUT;

const TEST_INPUT: &str = "
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

const NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn string_to_digit(input: &str) -> String {
    let mut res = String::new();

    for i in 0..input.len() {
        let slice = &input[i..];
        let mut found = 0;
        for (val, num) in NUMS.iter().enumerate() {
            if slice.starts_with(num) {
                found = val + 1;
            }
        }
        if found != 0 {
            res = format!("{}{}", res, found);
            // print!("NAB: found number {}, adding {} -> {}\n", slice, found + 1, res);
        }
        else {
            res = format!("{}{}", res, slice.chars().next().unwrap());
        }
    }
    
    // println!("NAB: returning {}", res);
    res
}


fn day1b(input: &str) -> u32 {
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
    let test_res = day1b(TEST_INPUT);
    print!("test: {test_res}\n");

    let real_res = day1b(REAL_INPUT);
    print!("real: {real_res}\n");
}
