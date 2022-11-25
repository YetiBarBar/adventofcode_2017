struct DebugBank {
    registers: Vec<usize>,
}

impl DebugBank {
    #[must_use]
    pub fn new(values: &[usize]) -> Self {
        Self {
            registers: values.into(),
        }
    }

    pub fn redistribute(&mut self) {
        let (position, max) =
            self.registers
                .iter()
                .enumerate()
                .fold((0, 0), |(old_pos, old_value), (pos, value)| {
                    if *value > old_value {
                        (pos, *value)
                    } else {
                        (old_pos, old_value)
                    }
                });
        let len = self.registers.len();
        self.registers[position] = 0;
        for idx in 1..=max {
            self.registers[(idx + position) % len] += 1;
        }
    }
}

fn main() {
    let input = include_str!("../data/day6.data").trim();

    let input: Vec<usize> = input
        .split_ascii_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    let mut dbg_bank = DebugBank::new(&input);
    let mut count_step_1 = 0;
    let mut hset = std::collections::HashSet::new();
    let mut value_to_search = vec![];
    for _ in 0.. {
        dbg_bank.redistribute();
        count_step_1 += 1;
        if hset.contains(&dbg_bank.registers) {
            value_to_search = dbg_bank.registers.clone();
            break;
        }
        hset.insert(dbg_bank.registers.clone());
    }
    println!("Part 1: {}", count_step_1);

    let mut dbg_bank2 = DebugBank::new(&input);
    let mut count_step_2 = 0;
    for _ in 0.. {
        dbg_bank2.redistribute();
        count_step_2 += 1;
        if dbg_bank2.registers == value_to_search {
            break;
        }
    }
    println!("Part 2: {}", count_step_1 - count_step_2);
}
