#[derive(Debug)]
struct Range {
    from: u64,
    to: u64,
    len: u64,
}

impl Range {
    fn parse(line: &str) -> Range {
        let mut it = line.split_whitespace();
        let to = it.next().unwrap().parse().unwrap();
        let from = it.next().unwrap().parse().unwrap();
        let len = it.next().unwrap().parse().unwrap();
        Range { from, to, len }
    }
}

#[derive(Debug)]
struct Map {
    from: String,
    to: String,
    ranges: Vec<Range>,
}

impl Map {
    fn parse(line: &str) -> Map {
        let header: Vec<&str> = line.split_whitespace().next().unwrap().split("-").collect();

        let from = header[0].to_owned();
        let to = header[2].to_owned();

        Map {
            from,
            to,
            ranges: Vec::new(),
        }
    }

    fn apply_map(&self, val: u64) -> u64 {
        for r in &self.ranges {
            if val >= r.from && val < r.from + r.len {
                let new = val + r.to - r.from;
                // print!("{} to {}: {} -> {}\n", self.from, self.to, val, new);
                return new;
            }
        }
        // print!("{} to {}: {} unchanged\n", self.from, self.to, val);
        val
    }
}

fn parse_seeds(input: &str) -> Vec<u64> {
    let line = input.lines().next().unwrap();
    let mut tokens = line.split("seeds: ");
    tokens.next();
    let rhs = tokens.next().unwrap();
    rhs.split_whitespace().map(|v| v.parse().unwrap()).collect()
}

fn parse_maps(input: &str) -> Vec<Map> {
    let mut maps: Vec<Map> = vec![];

    for l in input.lines() {
        if l.contains(" map:") {
            maps.push(Map::parse(l));
        } else if !l.is_empty() && !maps.is_empty() {
            let map = maps.last_mut().unwrap();
            map.ranges.push(Range::parse(l));
        }
    }
    maps
}

fn process(input: &str) -> u64 {
    let seeds = parse_seeds(input);
    let maps = parse_maps(input);

    seeds
        .iter()
        .map(|s| {
            let mut val = *s;
            for m in &maps {
                val = m.apply_map(val);
            }
            val
        })
        .min()
        .unwrap()
}

fn process_b(input: &str) -> u64 {
    let seeds = parse_seeds(input);
    let maps = parse_maps(input);

    seeds
        .windows(2)
        .step_by(2)
        .map(|w| {
            (w[0]..w[0] + w[1])
                .map(|s| {
                    let mut val = s;
                    for m in &maps {
                        val = m.apply_map(val);
                    }
                    val
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

fn main() {
    let data = std::fs::read_to_string("data/day5.txt").unwrap();
    let res = process_b(&data);
    print!("Result is: {}\n", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testicle() {
        let data = std::fs::read_to_string("data/day5_test.txt").unwrap();
        let res = process(&data);
        assert_eq!(res, 35);
    }

    #[test]
    fn test() {
        let data = std::fs::read_to_string("data/day5.txt").unwrap();
        let res = process(&data);
        assert_eq!(res, 88151870);
    }

    #[test]
    fn testicle_b() {
        let data = std::fs::read_to_string("data/day5_test.txt").unwrap();
        let res = process_b(&data);
        assert_eq!(res, 46);
    }
}
