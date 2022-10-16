use std::str::FromStr;

pub enum Instruction {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (chr, rest) = s.trim().split_at(1);
        match chr {
            "s" => Ok(Self::Spin(rest.parse().unwrap())),
            "x" => {
                let (v1, v2) = rest.split_once('/').unwrap();
                let v1 = v1.parse().unwrap();
                let v2 = v2.parse().unwrap();
                Ok(Self::Exchange(v1, v2))
            }
            "p" => {
                let (v1, v2) = rest.split_once('/').unwrap();
                let v1 = v1.chars().next().unwrap();
                let v2 = v2.chars().next().unwrap();
                Ok(Self::Partner(v1, v2))
            }
            _ => Err(()),
        }
    }
}

pub struct Cpu {
    memory: Vec<char>,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            memory: ('a'..='p').collect(),
        }
    }

    pub fn spin(&mut self, index: usize) {
        let (left, right) = self.memory.split_at(self.memory.len() - index);
        self.memory = right.iter().chain(left.iter()).copied().collect();
    }

    pub fn exchange(&mut self, pos_1: usize, pos_2: usize) {
        self.memory.swap(pos_1, pos_2);
    }

    pub fn partner(&mut self, chr_1: char, chr_2: char) {
        let pos_1 = self
            .memory
            .iter()
            .enumerate()
            .find(|(_, &ch)| ch == chr_1)
            .map(|(pos, _)| pos)
            .unwrap();
        let pos_2 = self
            .memory
            .iter()
            .enumerate()
            .find(|(_, &ch)| ch == chr_2)
            .map(|(pos, _)| pos)
            .unwrap();
        self.memory.swap(pos_1, pos_2);
    }

    pub fn run_instructions(&mut self, instructions: &[&str]) {
        for instr in instructions {
            let instr: Instruction = instr.parse().unwrap();
            match instr {
                Instruction::Spin(v) => self.spin(v),
                Instruction::Exchange(v1, v2) => self.exchange(v1, v2),
                Instruction::Partner(chr_1, chr_2) => self.partner(chr_1, chr_2),
            }
        }
    }
}
fn main() {
    let mut cpu = Cpu::new();
    let instructions = include_str!("../data/day16.data");
    let instructions: Vec<_> = instructions.split(',').collect();

    // Part 1
    cpu.run_instructions(&instructions);
    println!("Part 1: {}", cpu.memory.iter().collect::<String>());

    // Part 2:
    // Look for a cycle if available!
    let mut cpu = Cpu::new();

    let initial_memory: Vec<char> = ('a'..='p').collect();
    let mut cycle = 0;
    let mut has_cycle = false;
    for _ in 0..1_000_000_000 {
        cpu.run_instructions(&instructions);
        cycle += 1;
        if cpu.memory == initial_memory {
            has_cycle = true;
            break;
        }
    }

    if has_cycle {
        println!("Part 2: {}", cpu.memory.iter().collect::<String>());
    } else {
        // Reset cpu!
        let mut cpu = Cpu::new();
        for _ in 0..(1_000_000_000 % cycle) {
            cpu.run_instructions(&instructions);
        }
        println!("Part 2: {}", cpu.memory.iter().collect::<String>());
    }
}
