use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug)]
pub struct Programs {
    children: HashSet<String>,
    name: String,
    weight: usize,
}

impl Programs {
    fn weight(&self, prog_col: &HashMap<String, Programs>) -> usize {
        if self.children.is_empty() {
            self.weight
        } else {
            self.weight
                + self
                    .children
                    .iter()
                    .map(|child| prog_col.get(child).unwrap().weight(prog_col))
                    .sum::<usize>()
        }
    }

    fn is_balanced(&self, prog_col: &HashMap<String, Programs>) -> bool {
        if self.children.is_empty() {
            true
        } else {
            let ref_name = self.children.iter().next().unwrap();
            let ref_weight = prog_col.get(ref_name).unwrap().weight(prog_col);
            self.children
                .iter()
                .any(|child_name| prog_col.get(child_name).unwrap().weight(prog_col) != ref_weight)
        }
    }

    fn is_candidate(&self, prog_col: &HashMap<String, Programs>) -> bool {
        if self.children.is_empty() {
            false
        } else {
            self.is_balanced(prog_col)
                && self
                    .children
                    .iter()
                    .any(|child| !prog_col.get(child).unwrap().is_balanced(prog_col))
        }
    }
}

impl FromStr for Programs {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split("->").collect::<Vec<_>>();
        let mut children: HashSet<String> = parts
            .get(1)
            .unwrap_or(&"")
            .split(',')
            .map(str::trim)
            .map(ToString::to_string)
            .collect();

        let (name, rest) = parts[0].split_once(' ').unwrap();
        let name = name.trim().to_string();
        let number: String = rest.chars().filter(char::is_ascii_digit).collect();
        let number: usize = number.parse().unwrap();

        if children.contains("") {
            children.remove("");
        }
        Ok(Self {
            children,
            name,
            weight: number,
        })
    }
}

fn main() {
    let input = include_str!("../data/day7.data");

    let prgms: HashMap<String, Programs> = input
        .lines()
        .map(str::parse::<Programs>)
        .map(Result::unwrap)
        .map(|item| (item.name.clone(), item))
        .collect();

    let mut res_name: String = String::new();
    for (ref_name, refr) in &prgms {
        if prgms
            .iter()
            .all(|(_, prgm)| !prgm.children.contains(ref_name))
        {
            res_name = refr.name.clone();
            break;
        }
    }
    println!("Part 1: {}", res_name);

    // We start from root prgm and find first unbalanced node!
    let candidates = prgms
        .iter()
        .filter(|(_, prgm)| prgm.is_candidate(&prgms))
        .collect::<Vec<_>>();

    // let candidate = candidates.iter().find(|(prgm_name, _| candidates.iter().any(|(_, prgm)| prgm.children.con)));
    for prgm in candidates {
        println!("{}", prgm.0);
        println!("---------------");
        println!();
        for child in &prgm.1.children {
            println!("{} : {}", child, prgms.get(child).unwrap().weight(&prgms));
        }
        println!();
        println!();
    }

    /* println!(
        "First node balanced? : {}",
        prgms.get(&res_name).unwrap().is_balanced(&prgms)
    ); */
    // Recursive search of first unbalnced node

    // Deepest unbalanced node
}
