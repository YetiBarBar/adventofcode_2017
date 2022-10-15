use adventofcode_2017::knothasher::KnotHasher;

fn main() {
    let base = "hfdlxzhv";
    let knot_hashes: Vec<u128> = (0..128)
        .map(|idx| KnotHasher::hash(&format!("{}-{}", base, idx)))
        .map(|s| u128::from_str_radix(&s, 16).unwrap())
        .collect();

    let ones: u32 = knot_hashes.iter().map(|val| val.count_ones()).sum();

    println!("Part 1: {}", ones)
}
