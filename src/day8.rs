use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
enum Condition {
    NotEq,
    Eq,
    GreaterThan,
    GreaterEq,
    LowerThan,
    LowerEq,
}

impl FromStr for Condition {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "!=" => Ok(Self::NotEq),
            "==" => Ok(Self::Eq),
            ">=" => Ok(Self::GreaterEq),
            ">" => Ok(Self::GreaterThan),
            "<=" => Ok(Self::LowerEq),
            "<" => Ok(Self::LowerThan),
            x => Err(x.to_string()),
        }
    }
}

#[derive(Debug)]
enum Operation {
    Inc,
    Dec,
}

#[derive(Debug)]
struct Line {
    register: String,
    operation: Operation,
    value: isize,
    reg_cond: String,
    cond: Condition,
    reg_value: isize,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_ascii_whitespace().collect();
        Ok(Self {
            register: parts[0].trim().to_string(),
            operation: if parts[1].trim() == "inc" {
                Operation::Inc
            } else if parts[1].trim() == "dec" {
                Operation::Dec
            } else {
                panic!()
            },
            value: parts[2].trim().parse().unwrap(),
            reg_cond: parts[4].trim().to_string(),
            cond: parts[5].parse().unwrap(),
            reg_value: parts[6].trim().parse().unwrap(),
        })
    }
}

fn main() {
    let input = include_str!("../data/day8.data");
    // let input = r#"b inc 5 if a > 1
    // a inc 1 if b < 5
    // c dec -10 if a >= 1
    // c inc -20 if c == 10"#;
    let input_data: Vec<Line> = input.lines().map(str::parse).map(Result::unwrap).collect();
    let mut hmap = HashMap::new();
    let mut max = 0;
    for op in input_data {
        if verify_condition(&mut hmap, op.reg_cond, &op.cond, op.reg_value) {
            match op.operation {
                Operation::Inc => *hmap.entry(op.register.clone()).or_default() += op.value,
                Operation::Dec => *hmap.entry(op.register.clone()).or_default() -= op.value,
            }
            let c = hmap.get(&op.register).unwrap();
            if c > &max {
                max = *c;
            }
        }
    }
    println!("Part 1: {}", hmap.iter().map(|(_, val)| val).max().unwrap());
    println!("Part 2: {}", max);
}

fn verify_condition(
    hmap: &mut HashMap<String, isize>,
    register: String,
    condition: &Condition,
    value: isize,
) -> bool {
    let cur_value = hmap.entry(register).or_default();
    match condition {
        Condition::NotEq => *cur_value != value,
        Condition::Eq => *cur_value == value,
        Condition::GreaterThan => *cur_value > value,
        Condition::GreaterEq => *cur_value >= value,
        Condition::LowerThan => *cur_value < value,
        Condition::LowerEq => *cur_value <= value,
    }
}
