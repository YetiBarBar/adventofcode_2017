use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
struct Scanner(usize, usize);

impl FromStr for Scanner {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(':').unwrap();
        Ok(Self(
            left.trim().parse::<usize>()?,
            right.trim().parse::<usize>()?,
        ))
    }
}

fn main() {
    let input = include_str!("../data/day13.data").trim();
    let scanners = input
        .lines()
        .map(str::parse::<Scanner>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let count = scanners
        .iter()
        .filter_map(|scan| (scan.0 % (scan.1 * 2 - 2) == 0).then_some(scan.0 * scan.1))
        .sum::<usize>();
    println!("Part 1: {}", count);

    // Should have used CRT but bruteforce worked...
    for delay in 0.. {
        if scanners
            .iter()
            .all(|scan| (delay + scan.0) % (scan.1 * 2 - 2) != 0)
        {
            println!("Part 2: {}", delay);
            break;
        }
    }
}
