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
    Snd(Value),
    Set(Value, Value),
    Add(Value, Value),
    Mul(Value, Value),
    Mod(Value, Value),
    Rcv(Value),
    Jgz(Value, Value),
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
            "snd" => Ok(Self::Snd(parts[1].parse().unwrap())),
            "set" => Ok(Self::Set(
                parts[1].parse().unwrap(),
                parts[2].parse().unwrap(),
            )),
            "add" => Ok(Self::Add(
                parts[1].parse().unwrap(),
                parts[2].parse().unwrap(),
            )),
            "mul" => Ok(Self::Mul(
                parts[1].parse().unwrap(),
                parts[2].parse().unwrap(),
            )),
            "mod" => Ok(Self::Mod(
                parts[1].parse().unwrap(),
                parts[2].parse().unwrap(),
            )),
            "rcv" => Ok(Self::Rcv(parts[1].parse().unwrap())),
            "jgz" => Ok(Self::Jgz(
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
}

impl<'a> Cpu<'a> {
    fn step(&mut self) {
        match &self.program[self.eip as usize] {
            Command::Snd(value) => {
                self.last_frequency = match value {
                    Value::Reg(reg) => *self.registers.entry(reg.to_string()).or_default(),
                    Value::Int(val) => *val,
                };
                self.eip += 1;
            }
            Command::Set(reg, val) => {
                if let Value::Reg(reg) = reg {
                    *self.registers.entry(reg.to_string()).or_default() = match val {
                        Value::Reg(rg) => *self.registers.entry(rg.to_string()).or_default(),
                        Value::Int(v) => *v,
                    }
                };
                self.eip += 1;
            }
            Command::Add(reg, val) => {
                if let Value::Reg(reg) = reg {
                    *self.registers.entry(reg.to_string()).or_default() += match val {
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
                self.eip += 1;
            }
            Command::Mod(reg, val) => {
                if let Value::Reg(reg) = reg {
                    *self.registers.entry(reg.to_string()).or_default() %= match val {
                        Value::Reg(rg) => *self.registers.entry(rg.to_string()).or_default(),
                        Value::Int(v) => *v,
                    }
                };
                self.eip += 1;
            }
            Command::Rcv(reg) => {
                self.eip += 1;
                if let Value::Reg(reg) = reg {
                    if *self.registers.entry(reg.to_string()).or_default() != 0 {
                        *self.registers.entry(reg.to_string()).or_default() = self.last_frequency;
                        self.stall = true;
                    }
                };
            }
            Command::Jgz(cond, offset) => {
                let cond = match cond {
                    Value::Reg(reg) => *self.registers.entry(reg.to_string()).or_insert(0) > 0,
                    Value::Int(val) => *val > 0,
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
        }
    }

    fn step2(&mut self) -> Option<isize> {
        match &self.program[self.eip as usize] {
            Command::Snd(value) => {
                self.eip += 1;
                Some(match value {
                    Value::Reg(reg) => *self.registers.entry(reg.to_string()).or_default(),
                    Value::Int(val) => *val,
                })
            }
            Command::Set(reg, val) => {
                if let Value::Reg(reg) = reg {
                    *self.registers.entry(reg.to_string()).or_default() = match val {
                        Value::Reg(rg) => *self.registers.entry(rg.to_string()).or_default(),
                        Value::Int(v) => *v,
                    }
                };
                self.eip += 1;
                None
            }
            Command::Add(reg, val) => {
                if let Value::Reg(reg) = reg {
                    *self.registers.entry(reg.to_string()).or_default() += match val {
                        Value::Reg(rg) => *self.registers.entry(rg.to_string()).or_default(),
                        Value::Int(v) => *v,
                    }
                };
                self.eip += 1;
                None
            }
            Command::Mul(reg, val) => {
                if let Value::Reg(reg) = reg {
                    *self.registers.entry(reg.to_string()).or_default() *= match val {
                        Value::Reg(rg) => *self.registers.entry(rg.to_string()).or_default(),
                        Value::Int(v) => *v,
                    }
                };
                self.eip += 1;
                None
            }
            Command::Mod(reg, val) => {
                if let Value::Reg(reg) = reg {
                    *self.registers.entry(reg.to_string()).or_default() %= match val {
                        Value::Reg(rg) => *self.registers.entry(rg.to_string()).or_default(),
                        Value::Int(v) => *v,
                    }
                };
                self.eip += 1;
                None
            }
            Command::Rcv(reg) => {
                if let Some(val) = self.queue.pop_front() {
                    self.eip += 1;
                    if let Value::Reg(reg) = reg {
                        *self.registers.entry(reg.to_string()).or_default() = val;
                    };
                    self.stall = false;
                } else {
                    self.stall = true;
                }
                None
            }
            Command::Jgz(cond, offset) => {
                let cond = match cond {
                    Value::Reg(reg) => *self.registers.entry(reg.to_string()).or_insert(0) > 0,
                    Value::Int(val) => *val > 0,
                };
                if cond {
                    self.eip += match offset {
                        Value::Reg(reg) => *self.registers.entry(reg.to_string()).or_insert(0),
                        Value::Int(val) => *val,
                    };
                } else {
                    self.eip += 1;
                }
                None
            }
        }
    }
}

fn main() {
    let input = include_str!("../data/day18.data");
    let commands: Vec<Command> = input.lines().map(str::parse).map(Result::unwrap).collect();
    let mut cpu = Cpu::new(&commands);

    while !cpu.stall {
        cpu.step();
    }

    println!("Part 1: {}", cpu.last_frequency);

    let mut counter = 0;
    let mut cpu0 = Cpu::new(&commands);
    let mut cpu1 = Cpu::new(&commands);
    cpu1.registers.insert("p".to_string(), 1);

    loop {
        let v0 = cpu0.step2();
        let v1 = cpu1.step2();

        if let Some(v0) = v0 {
            cpu1.queue.push_back(v0);
        }

        if let Some(v1) = v1 {
            cpu0.queue.push_back(v1);
            counter += 1;
        }

        if cpu1.stall && cpu0.stall {
            break;
        }
    }

    println!("Part 2: {}", counter);
}
