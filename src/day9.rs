fn clean_garbage(input: &str) -> (String, usize) {
    let mut ignore_next = false;
    let mut open_garbage = false;
    let mut v = Vec::new();
    let mut count = 0;

    for chr in input.chars() {
        if ignore_next {
            ignore_next = false;
        } else if chr == '!' {
            ignore_next = true;
        } else if open_garbage {
            if chr == '>' {
                open_garbage = false;
            } else {
                count += 1;
            }
        } else if chr == '<' {
            open_garbage = true;
        } else {
            v.push(chr);
        }
    }
    (v.iter().collect(), count)
}

fn evaluate(input: &str, level: usize) -> usize {
    let mut last_open = None;
    let mut depth = 0;
    let mut top_level = Vec::new();
    for (position, chr) in input.chars().enumerate() {
        if chr == '{' {
            if depth == 0 {
                last_open = Some(position);
            }
            depth += 1;
        }
        if chr == '}' {
            if depth == 1 {
                if let Some(lp) = last_open {
                    top_level.push((lp, position));
                    last_open = None;
                    depth = 0;
                } else {
                    panic!("unbalanced!");
                }
            } else {
                depth -= 1;
            }
        }
    }

    level
        + top_level
            .iter()
            .map(|&(a, b)| evaluate(&input[a + 1..b], level + 1))
            .sum::<usize>()
}

fn main() {
    let input = include_str!("../data/day9.data");
    let (cleaned, garbage_count) = clean_garbage(input);
    println!("Part 1: {}", evaluate(&cleaned, 0));
    println!("Part 2: {}", garbage_count);
}
