use regex::Regex;
use std::collections::HashMap;

const STARTING_NODE: &str = "AAA";
const ENDING_NODE: &str = "ZZZ";

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn parse(input: &str) -> Node {
        let re = Regex::new(r"(...) = \((...), (...)\)").unwrap();
        let captures = re.captures(input).unwrap();
        Node {
            name: captures[1].to_owned(),
            left: captures[2].to_owned(),
            right: captures[3].to_owned(),
        }
    }

    fn get_next_node_name(&self, d: Direction) -> &str {
        match d {
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
}

fn parse_directions(input: &str) -> Vec<Direction> {
    input
        .lines()
        .nth(0)
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Unexpected panda in the bagging area!"),
        })
        .collect()
}

fn parse_nodes(input: &str) -> HashMap<String, Node> {
    input
        .lines()
        .skip(2)
        .map(|l| {
            let node = Node::parse(l);
            (node.name.clone(), node)
        })
        .collect()
}

fn process(input: &str) -> usize {
    let directions = parse_directions(input);
    let nodes = parse_nodes(input);
    print!("Nodes are: {:?}\n", nodes);

    let mut steps: usize = 0;
    let mut current_node = nodes.get(STARTING_NODE).expect("Starting node not found!");

    while current_node.name != ENDING_NODE {
        for d in &directions {
            print!(
                "At node {:?}. Applying direction {:?}\n",
                current_node.name, d
            );
            let next_node_name = current_node.get_next_node_name(*d);
            current_node = nodes.get(next_node_name).unwrap();
            steps += 1;
        }
    }
    steps
}

fn main() {
    let data = std::fs::read_to_string("data/day8_test_1.txt").unwrap();
    let result = process(&data);
    print!("Result is {}\n", result);
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

    #[test]
    fn test() {
        let data = std::fs::read_to_string("data/day8.txt").unwrap();
        let result = process(&data);
        assert_eq!(result, 20659);
    }
}
