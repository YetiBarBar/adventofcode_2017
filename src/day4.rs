fn main() {
    let input = include_str!("../data/day4.data");

    let res = input
        .lines()
        .filter(|line| {
            line.split_ascii_whitespace().count()
                == line
                    .split_ascii_whitespace()
                    .collect::<std::collections::HashSet<_>>()
                    .len()
        })
        .count();
    println!("Part 1: {}", res);

    let res = input
        .lines()
        .filter(|line| {
            line.split_ascii_whitespace().count()
                == line
                    .split_ascii_whitespace()
                    .map(|word| {
                        let mut v = word.chars().collect::<Vec<_>>();
                        v.sort_unstable();
                        v
                    })
                    .collect::<std::collections::HashSet<_>>()
                    .len()
        })
        .count();

    println!("Part 2: {}", res);
}
