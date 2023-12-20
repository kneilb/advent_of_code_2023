#[derive(Debug)]
struct Number {
    val: u32,
    row: usize,
    left_col: usize,
    right_col: usize,
}

#[derive(Debug)]
struct Symbol {
    char: char,
    row: usize,
    col: usize,
}

fn process(data: &str) {
    let mut numbers: Vec<Number> = vec![];
    let mut symbols: Vec<Symbol> = vec![];

    for (line_index, line) in data.lines().enumerate() {
        let mut start_col = 0;
        let mut num_chars = String::new();
        // print!("NAB: found a lovely line at {}! {}\n", line_index, line);
        for (char_index, char) in line.chars().enumerate() {
            // print!(
            //     "NAB: found a lovely character at {}! {}\n",
            //     char_index, char
            // );
            if char.is_numeric() {
                if num_chars.is_empty() {
                    start_col = char_index;
                }
                num_chars = format!("{}{}", num_chars, char);
            } else {
                if num_chars.len() > 0 {
                    let num: u32 = num_chars.parse().unwrap();
                    num_chars = String::new();
                    numbers.push(Number {
                        val: num,
                        row: line_index,
                        left_col: start_col,
                        right_col: char_index,
                    });
                }

                if char != '.' {
                    symbols.push(Symbol {
                        char,
                        row: line_index,
                        col: char_index,
                    })
                }
            }
        }
    }

    print!("NAB: symbols are: {:?}", symbols);
    print!("NAB: numbers are: {:?}", numbers);
}

fn main() {
    let data = std::fs::read_to_string("data/day3_test.txt").unwrap();

    process(&data);
}

#[cfg(test)]
mod test {
    #[test]
    fn testicle() {}
}
