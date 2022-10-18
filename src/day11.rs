fn main() {
    let input: Vec<&str> = include_str!("../data/day11.data").split(',').collect();
    // let input: Vec<_> = "ne,ne,sw,sw".split(',').collect();

    let (x, y, dist) = input
        .iter()
        .fold((0_isize, 0_isize, 0_isize), |(x, y, dist), word| {
            let (x, y) = match word.trim() {
                "ne" => (x + 1, y),
                "nw" => (x - 1, y + 1),
                "se" => (x + 1, y - 1),
                "sw" => (x - 1, y),
                "n" => (x, y + 1),
                "s" => (x, y - 1),
                _ => panic!(),
            };
            (x, y, dist.max((x.abs() + y.abs() + (x + y).abs()) / 2))
        });

    println!("Part 1: {:?}", (x.abs() + y.abs() + (x + y).abs()) / 2);
    println!("Part 2: {:?}", dist);
}
