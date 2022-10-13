fn main() {
    let input = include_str!("../data/day1.data").trim();
    // let input = "91212129";
    // let input = "1122";
    let res = sum_digits_ahead(input, 1);
    println!("Part 1: {}", res);

    let res = sum_digits_ahead(input, input.chars().count() / 2);
    println!("Part 2: {}", res);
}

fn sum_digits_ahead(input: &str, skip: usize) -> u32 {
    let res = input
        .chars()
        .zip(input.chars().cycle().skip(skip))
        .filter(|(chr, next)| chr == next)
        .map(|chr| chr.0.to_digit(10).unwrap())
        .sum::<u32>();
    res
}
