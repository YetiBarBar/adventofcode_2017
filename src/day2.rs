fn main() {
    let input = include_str!("../data/day2.data").trim();

    let input: Vec<Vec<usize>> = input.lines().map(parse_line).collect();
    let res = input
        .iter()
        .map(|v| v.iter().max().unwrap() - v.iter().min().unwrap())
        .sum::<usize>();
    println!("Part 1: {}", res);

    let res = input.iter().map(|v| evenly_division(v)).sum::<usize>();
    println!("Part 1: {}", res);
}

fn parse_line(line: &str) -> Vec<usize> {
    line.split_ascii_whitespace()
        .filter_map(|value| value.parse::<usize>().ok())
        .collect()
}

fn evenly_division(v: &[usize]) -> usize {
    for value1 in v {
        for value2 in v {
            if value1 != value2 && value1 % value2 == 0 {
                return value1 / value2;
            }
        }
    }
    0
}
