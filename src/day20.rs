use std::{collections::HashMap, str::FromStr};

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
struct Particle {
    acc: Acceleration,
    speed: Speed,
    position: Position,
}

impl PartialOrd for Particle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_pos = self.current_position(1_000_000);
        let other_pos = other.current_position(1_000_000);
        (self_pos.0.abs() + self_pos.1.abs() + self_pos.2.abs())
            .partial_cmp(&(other_pos.0.abs() + other_pos.1.abs() + other_pos.2.abs()))
    }
}
impl Ord for Particle {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl Particle {
    fn current_position(&self, t: isize) -> (isize, isize, isize) {
        (
            self.position.0 + self.speed.0 * t + self.acc.0 * t * t / 2,
            self.position.1 + self.speed.1 * t + self.acc.1 * t * t / 2,
            self.position.2 + self.speed.2 * t + self.acc.2 * t * t / 2,
        )
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
struct Position(isize, isize, isize);

impl FromStr for Particle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits = s.split_ascii_whitespace().collect::<Vec<_>>();
        Ok(Self {
            acc: splits[2].parse().unwrap(),
            speed: splits[1].parse().unwrap(),
            position: splits[0].parse().unwrap(),
        })
    }
}

impl FromStr for Position {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let payload = s
            .split_once('<')
            .unwrap()
            .1
            .trim_end_matches('>')
            .trim_end_matches(">,")
            .split(',')
            .map(str::parse)
            .map(Result::unwrap)
            .collect::<Vec<_>>();

        Ok(Self(payload[0], payload[1], payload[2]))
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
struct Acceleration(isize, isize, isize);

impl FromStr for Acceleration {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let payload = s
            .split_once('<')
            .unwrap()
            .1
            .trim_end_matches('>')
            .trim_end_matches(">,")
            .split(',')
            .map(str::parse)
            .map(Result::unwrap)
            .collect::<Vec<_>>();

        Ok(Self(payload[0], payload[1], payload[2]))
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
struct Speed(isize, isize, isize);

impl FromStr for Speed {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let payload = s
            .split_once('<')
            .unwrap()
            .1
            .trim_end_matches('>')
            .trim_end_matches(">,")
            .split(',')
            .map(str::parse)
            .map(Result::unwrap)
            .collect::<Vec<_>>();

        Ok(Self(payload[0], payload[1], payload[2]))
    }
}

fn main() {
    let mut particles: Vec<_> = include_str!("../data/day20.data")
        .lines()
        .map(str::parse::<Particle>)
        .map(Result::unwrap)
        .collect();

    let min = particles.iter().min().unwrap();
    let index = particles
        .iter()
        .enumerate()
        .find(|item| item.1 == min)
        .unwrap();
    println!("Part 1: {:?}", index);
}
