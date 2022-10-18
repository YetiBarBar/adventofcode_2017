use std::iter::repeat;

fn spin(mut values: Vec<usize>, takes: usize, position: usize, next: usize) -> (Vec<usize>, usize) {
    let insert_position = (position + takes) % values.len() + 1;
    values.insert(insert_position, next);
    (values, insert_position)
}

fn main() {
    let v = vec![0];

    let ((v, _), _) = repeat(369)
        .take(2017)
        .fold(((v, 0), 1), |((acc, position), next), item| {
            // println!("{:?}", acc);
            (spin(acc, item, position, next), next + 1)
        });

    println!(
        "Part 1: {}",
        v.iter().skip_while(|item| item != &&2017).nth(1).unwrap()
    );

    // As 0 is always the first element, we only have to keep track of the last item inserted in position 1
    let mut position = 0;
    let mut next_val = 0;
    for idx in 1..=50_000_000 {
        position = (position + 369) % idx + 1;
        if position == 1 {
            next_val = idx;
        }
    }

    println!("Part 2: {}", next_val);
}
