const TEST_INPUT: &str = "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

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

fn main() {
    let test_res = day1(TEST_INPUT);
    print!("test: {test_res}\n");

    let test_data = std::fs::read_to_string("data/day1.txt").unwrap();
    let real_res = day1(&test_data);
    print!("real: {real_res}\n");
}
