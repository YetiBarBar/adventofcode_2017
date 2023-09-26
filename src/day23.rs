use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

#[derive(Debug, Clone)]
enum Value {
    Reg(String),
    Int(isize),
}

#[derive(Debug, Clone)]
enum Command {
    Set(Value, Value),
    Sub(Value, Value),
    Mul(Value, Value),
    Jnz(Value, Value),
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().chars().all(char::is_alphabetic) {
            Ok(Self::Reg(s.trim().to_string()))
        } else {
            Ok(Self::Int(s.trim().parse::<isize>().unwrap()))
        }
    }
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_ascii_whitespace().collect();
        match parts[0].trim() {
            "set" => Ok(Self::Set(
                parts[1].parse().unwrap(),
                parts[2].parse().unwrap(),
            )),
            "sub" => Ok(Self::Sub(
                parts[1].parse().unwrap(),
                parts[2].parse().unwrap(),
            )),
            "mul" => Ok(Self::Mul(
                parts[1].parse().unwrap(),
                parts[2].parse().unwrap(),
            )),
            "jnz" => Ok(Self::Jnz(
                parts[1].parse().unwrap(),
                parts[2].parse().unwrap(),
            )),

            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Cpu<'a> {
    registers: HashMap<String, isize>,
    eip: isize,
    last_frequency: isize,
    program: &'a Vec<Command>,
    stall: bool,
    queue: VecDeque<isize>,
    count_mul: usize,
}

impl<'a> Cpu<'a> {
    fn step(&mut self) {
        match &self.program[self.eip as usize] {
            Command::Set(reg, val) => {
                if let Value::Reg(reg) = reg {
                    *self.registers.entry(reg.to_string()).or_default() = match val {
                        Value::Reg(rg) => *self.registers.entry(rg.to_string()).or_default(),
                        Value::Int(v) => *v,
                    }
                };
                self.eip += 1;
            }
            Command::Mul(reg, val) => {
                if let Value::Reg(reg) = reg {
                    *self.registers.entry(reg.to_string()).or_default() *= match val {
                        Value::Reg(rg) => *self.registers.entry(rg.to_string()).or_default(),
                        Value::Int(v) => *v,
                    }
                };
                self.count_mul += 1;
                self.eip += 1;
            }
            Command::Jnz(cond, offset) => {
                let cond = match cond {
                    Value::Reg(reg) => *self.registers.entry(reg.to_string()).or_insert(0) != 0,
                    Value::Int(val) => *val != 0,
                };
                if cond {
                    self.eip += match offset {
                        Value::Reg(reg) => *self.registers.entry(reg.to_string()).or_insert(0),
                        Value::Int(val) => *val,
                    };
                } else {
                    self.eip += 1;
                }
            }
            Command::Sub(reg, val) => {
                if let Value::Reg(reg) = reg {
                    *self.registers.entry(reg.to_string()).or_default() -= match val {
                        Value::Reg(rg) => *self.registers.entry(rg.to_string()).or_default(),
                        Value::Int(v) => *v,
                    }
                };
                self.eip += 1;
            }
        }
    }

    fn new(commands: &'a Vec<Command>) -> Self {
        Self {
            registers: HashMap::new(),
            eip: 0,
            last_frequency: 0,
            program: commands,
            stall: false,
            queue: VecDeque::new(),
            count_mul: 0,
        }
    }
}

fn main() {
    let input = include_str!("../data/day23.data");
    let commands: Vec<Command> = input.lines().map(str::parse).map(Result::unwrap).collect();
    let mut cpu = Cpu::new(&commands);
    loop {
        if cpu.program.get(cpu.eip as usize).is_some() {
            cpu.step();
        } else {
            break;
        }
    }

    println!("Step 1: {}", cpu.count_mul);

    /* Reversing engineering the prog
    If a == 1, the program counts the number of primes between
    of form 125_100 + 17 * idx for idx in 0..1001
    */

    let part2_res = (0..1001)
        .filter(|idx| !primal::is_prime(108_100 + idx * 17))
        .count();

    // Between 904 and 910
    // 907 autre input
    println!("Part 2: {}", part2_res);
}
