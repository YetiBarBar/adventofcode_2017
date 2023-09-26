use std::{collections::HashSet, str::FromStr};

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Bridge {
    ports: [u64; 2],
}

impl Bridge {
    fn has_port(&self, port: u64) -> bool {
        self.ports[0] == port || self.ports[1] == port
    }

    fn value(&self) -> u64 {
        self.ports.iter().sum()
    }

    fn get_free_port(&self, current: u64) -> u64 {
        if self.ports[0] == current {
            self.ports[1]
        } else {
            self.ports[0]
        }
    }
}

impl FromStr for Bridge {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once('/').unwrap();
        let port_a = a.parse().unwrap();

        let port_b = b.parse().unwrap();

        Ok(Bridge {
            ports: [port_a, port_b],
        })
    }
}

fn main() {
    let bridges: HashSet<Bridge> = include_str!("../data/day24.data")
        // let bridges: HashSet<Bridge> = include_str!("../data/day24_test")
        .lines()
        .map(str::parse::<Bridge>)
        .map(Result::unwrap)
        .collect();

    let visited = HashSet::new();

    let max = recursive_search(0, &bridges, visited, 0);
    println!("Part 1: {max}");

    let visited = HashSet::new();
    let max = recursive_search_2(0, &bridges, visited, 0);
    println!("Part 2: {}", max.1);
    println!("Part 2: {}", max.0);
}

fn recursive_search(
    current: u64,
    world: &HashSet<Bridge>,
    visited: HashSet<Bridge>,
    depth: usize,
) -> u64 {
    // Get all next possible next bridges!
    let available_bridges: Vec<_> = world
        .iter()
        .filter(|bridge| !visited.contains(bridge))
        .filter(|bridge| bridge.has_port(current))
        .collect();

    let mut max = 0;
    for bridge in available_bridges {
        let mut visited_clone = visited.clone();
        visited_clone.insert(bridge.clone());
        let cur_value = bridge.value()
            + recursive_search(
                bridge.get_free_port(current),
                world,
                visited_clone,
                depth + 1,
            );
        if cur_value > max {
            max = cur_value;
        }
    }
    max
}

fn recursive_search_2(
    current: u64,
    world: &HashSet<Bridge>,
    visited: HashSet<Bridge>,
    depth: u64,
) -> (u64, u64) {
    // Get all next possible next bridges!
    let available_bridges: Vec<_> = world
        .iter()
        .filter(|bridge| !visited.contains(bridge))
        .filter(|bridge| bridge.has_port(current))
        .collect();

    let mut max_depth = depth;
    let mut max = 0;
    for bridge in available_bridges {
        let mut visited_clone = visited.clone();
        visited_clone.insert(bridge.clone());
        let (dep, cur_value) = recursive_search_2(
            bridge.get_free_port(current),
            world,
            visited_clone,
            depth + 1,
        );
        if dep > max_depth {
            max_depth = dep;

            max = cur_value + bridge.value();
        }
        if dep == max_depth {
            max_depth = dep;
            if cur_value + bridge.value() > max {
                max = cur_value + bridge.value();
            }
        }
    }
    // println!("{}, {}", max_depth, max);
    (max_depth, max)
}
