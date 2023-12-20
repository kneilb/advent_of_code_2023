#[derive(Debug)]
struct Number {
    val: u32,
    row: usize,
    left_col: usize,
    right_col: usize,
}

impl Number {
    fn is_adjacent(&self, symbol: &Symbol) -> bool {
        // 0:     0  1  2  3  4
        // 1:     0  1a 2a 3  4
        // 2:     0  1  2  3  4

        let row_start = if self.row > 0 { self.row - 1 } else { self.row };
        let col_start = if self.left_col > 0 {
            self.left_col - 1
        } else {
            self.left_col
        };
        symbol.row >= row_start
            && symbol.row <= self.row + 1
            && symbol.col >= col_start
            && symbol.col <= self.right_col + 1
    }

    fn any_adjacent(&self, symbols: &Vec<Symbol>) -> bool {
        for s in symbols {
            if self.is_adjacent(s) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
struct Symbol {
    char: char,
    row: usize,
    col: usize,
}

impl Symbol {
    fn find_adjacent(&self, numbers: &Vec<Number>) {

    }
}

fn parse_locations(data: &str) -> (Vec<Number>, Vec<Symbol>) {
    let mut numbers: Vec<Number> = vec![];
    let mut symbols: Vec<Symbol> = vec![];

    for (row, line) in data.lines().enumerate() {
        let mut left_col = 0;
        let mut num_chars = String::new();

        for (col, char) in line.chars().enumerate() {
            if char.is_numeric() {
                if num_chars.is_empty() {
                    left_col = col;
                }
                num_chars = format!("{}{}", num_chars, char);

                // TODO: ugly repetition
                if col == line.len() - 1 {
                    let val: u32 = num_chars.parse().unwrap();
                    numbers.push(Number {
                        val,
                        row,
                        left_col,
                        right_col: col, // we're at the end of the number
                    });
                }
            } else {
                if num_chars.len() > 0 {
                    let val: u32 = num_chars.parse().unwrap();
                    num_chars = String::new();
                    numbers.push(Number {
                        val,
                        row,
                        left_col,
                        right_col: col - 1, // we're one character past the end of the number
                    });
                }

                if char != '.' {
                    symbols.push(Symbol { char, row, col });
                }
            }
        }
    }
    (numbers, symbols)
}

fn process(data: &str) -> u32 {
    let (numbers, symbols) = parse_locations(data);

    numbers.iter().map(|n| {
        if n.any_adjacent(&symbols) {
            n.val
        } else { 0 }
    }).sum()
}

fn process_b(data: &str) -> u32 {
    let (numbers, symbols) = parse_locations(data);

    let mut sum = 0;

    for s in symbols {
        let adjacent_numbers: Vec<&Number> = numbers.iter().filter(|n| n.is_adjacent(&s)).collect();
        if adjacent_numbers.len() == 2 {
            let first = adjacent_numbers[0].val;
            let second = adjacent_numbers[1].val;
            sum += first * second;
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

    let sum = process_b(&data);
    print!("Gear ratio sum is {}\n", sum);
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
    fn testicle_2() {
        let data = std::fs::read_to_string("data/day3_test_2.txt").unwrap();

        let sum = process(&data);
        assert_eq!(sum, 4362);
    }

    #[test]
    fn test_larger() {
        let data = std::fs::read_to_string("data/day3.txt").unwrap();

        let sum = process(&data);
        assert_ne!(sum, 531318); // too high! (caused by off by one)
        assert_ne!(sum, 525642); // too low! (caused by missing numbers that ended at the end of the row)
        assert_eq!(sum, 527144); // yay!
    }

    #[test]
    fn testicle_b() {
        let data = std::fs::read_to_string("data/day3b_test.txt").unwrap();

        let sum = process_b(&data);
        assert_eq!(sum, 467835);
    }
}
