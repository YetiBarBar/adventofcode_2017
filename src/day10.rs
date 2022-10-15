use adventofcode_2017::knothasher::KnotHasher;

fn main() {
    let input = include_str!("../data/day10.data");
    let lenghts = input
        .trim()
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect::<Vec<u8>>();
    let mut knot = KnotHasher::new();
    knot.encode_u8_sequence(&lenghts);

    println!("Part 1: {}", knot.buffer[0] * knot.buffer[1]);

    let input = include_str!("../data/day10.data").trim();

    println!("Part 2: {}", KnotHasher::hash(input));
}
