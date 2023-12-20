#[derive(Debug)]
struct Number {
    val: u32,
    row: usize,
    left_col: usize,
    right_col: usize,
}

impl Number {
    fn is_adjacent(&self, symbols: &Vec<Symbol>) -> bool {
        // 0:     0  1  2  3  4
        // 1:     0  1a 2a 3  4
        // 2:     0  1  2  3  4
        for s in symbols {
            let row_start = if self.row > 0 { self.row - 1 } else { self.row };
            let col_start = if self.left_col > 0 {
                self.left_col - 1
            } else {
                self.left_col
            };
            if s.row >= row_start
                && s.row <= self.row + 1
                && s.col >= col_start
                && s.col <= self.right_col + 1
            {
                // print!("NAB: Decided that {:?} is adjacent to {:?}!\n", self, s);
                return true;
            }
        }
        return false;
    }
}

#[derive(Debug)]
struct Symbol {
    char: char,
    row: usize,
    col: usize,
}

fn process(data: &str) -> u32 {
    let mut numbers: Vec<Number> = vec![];
    let mut symbols: Vec<Symbol> = vec![];

    for (line_index, line) in data.lines().enumerate() {
        let mut start_col = 0;
        let mut num_chars = String::new();

        for (char_index, char) in line.chars().enumerate() {
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

    // print!("NAB: symbols are: {:?}", symbols);
    // print!("NAB: numbers are: {:?}", numbers);

    let mut sum = 0;

    for num in numbers {
        if num.is_adjacent(&symbols) {
            sum += num.val;
        }
    }

    sum
}

fn main() {
    // let data = std::fs::read_to_string("data/day3_test.txt").unwrap();

    // let sum = process(&data);
    // print!("Sum is {}\n", sum);

    let data = std::fs::read_to_string("data/day3.txt").unwrap();

    let sum = process(&data);
    print!("Sum is {}\n", sum);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testicle() {
        let data = std::fs::read_to_string("data/day3_test.txt").unwrap();

        let sum = process(&data);
        assert_eq!(sum, 4361);
    }

    #[test]
    fn test_larger() {
        let data = std::fs::read_to_string("data/day3.txt").unwrap();

        let sum = process(&data);
        assert_ne!(sum, 531318);
        // TODO! This is the wrong answer...!
    }
}
