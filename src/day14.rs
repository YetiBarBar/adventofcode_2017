use std::{cell, fmt::Display};

use adventofcode_2017::knothasher::KnotHasher;

#[derive(Clone)]
struct Cell {
    value: bool,
    group: Option<usize>,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.value {
            match self.group {
                Some(g) => write!(f, "{:X}", g),
                None => write!(f, "#"),
            }
        } else {
            write!(f, ".")
        }
    }
}

fn main() {
    let base = "hfdlxzhv";
    // let base = "flqrgnkx";
    let knot_hashes: Vec<u128> = (0..128)
        .map(|idx| KnotHasher::hash(&format!("{}-{}", base, idx)))
        .map(|s| u128::from_str_radix(&s, 16).unwrap())
        .collect();

    let ones: u32 = knot_hashes.iter().map(|val| val.count_ones()).sum();

    println!("Part 1: {}", ones);

    let mut current_group = 0;
    let mut cells: Vec<Vec<Cell>> = knot_hashes
        .iter()
        .map(|s| {
            let st = format!("{:128b}", s);
            st.chars()
                .map(|v| {
                    if v == '1' {
                        Cell {
                            value: true,
                            group: None,
                        }
                    } else {
                        Cell {
                            value: false,
                            group: None,
                        }
                    }
                })
                .collect()
        })
        .collect();
    for y in 0..128 {
        for x in 0..128 {
            if mark_cell(x, y, current_group, 0, &mut cells) {
                current_group += 1;
            }
        }
    }

    println!("Part 2: {}", current_group);
}

fn mark_cell(x: usize, y: usize, group: usize, level: usize, table: &mut Vec<Vec<Cell>>) -> bool {
    if table.get(y).unwrap().get(x).unwrap().value {
        if table.get(y).unwrap().get(x).unwrap().group.is_none() {
            table[y][x].group = Some(group);
            // Move up!
            if y != 0 {
                mark_cell(x, y - 1, group, level + 1, table);
            }
            // Then right
            if x != 127 {
                mark_cell(x + 1, y, group, level + 1, table);
            }
            // Then down
            if y != 127 {
                mark_cell(x, y + 1, group, level + 1, table);
            }
            // Then left
            if x != 0 {
                mark_cell(x - 1, y, group, level + 1, table);
            }
            if level == 0 {
                return true;
            }
        }
    }
    false
}
