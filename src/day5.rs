fn main() {
    let input = include_str!("../data/day5.data");
    let mut input_data: Vec<isize> = input
        .lines()
        .map(str::trim)
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let mut count = 0;
    let mut cur_pos: isize = 0;
    while let Some(value) = input_data.get_mut(usize::try_from(cur_pos).unwrap()) {
        count += 1;
        cur_pos += *value;
        *value += 1;
    }
    println!("Part 1: {}", count);

    let mut input_data: Vec<isize> = input
        .lines()
        .map(str::trim)
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let mut count = 0;
    let mut cur_pos: isize = 0;
    while let Some(value) = input_data.get_mut(usize::try_from(cur_pos).unwrap()) {
        count += 1;
        cur_pos += *value;
        *value += if *value >= 3 { -1 } else { 1 };
    }
    println!("Part 2: {}", count);
}
