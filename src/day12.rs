use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../data/day12.data");
    let mut hmap: HashMap<usize, HashSet<usize>> = HashMap::new();
    for line in input.lines() {
        let (left, right) = line.split_once("<->").unwrap();
        let left: usize = left.trim().parse().expect("left");
        let right: Vec<usize> = right
            .split(',')
            .map(str::trim)
            .map(str::parse)
            .map(Result::unwrap)
            .collect();
        hmap.entry(left).or_default().extend(right.iter());
        for v in right {
            hmap.entry(v).or_default().insert(left);
        }
    }

    let group_0 = find_group(&hmap, 0);
    println!("Part 1: {}", group_0.len());

    // TODO: Find why this is unstable and sometimes of by one...
    let mut groups = 0;
    let mut founds: HashSet<usize> = HashSet::new();
    loop {
        let val = hmap.keys().find(|key| founds.contains(key)).unwrap();

        let group = find_group(&hmap, *val);
        groups += 1;

        founds.extend(group.iter());
        if founds.len() == hmap.len() {
            break;
        }
    }
    println!("Part 2: {}", groups);
}

fn find_group(hmap: &HashMap<usize, HashSet<usize>>, value: usize) -> HashSet<usize> {
    let mut res = HashSet::new();

    let mut to_insert = vec![value];
    loop {
        let mut new_items = vec![];
        let mut to_remove = vec![];
        for item in &to_insert {
            for &c in hmap.get(item).unwrap() {
                if res.contains(&c) {
                    to_remove.push(c);
                } else {
                    res.insert(c);
                    new_items.push(c);
                }
            }
            to_remove.push(*item);
        }
        to_insert.extend(new_items.iter());
        to_insert.retain(|v| !to_remove.contains(v));
        if to_insert.is_empty() {
            break;
        }
    }
    res
}
