use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", compute_position(289_326));
    println!("Part 2: {}", step2(289_326));
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn compute_position(n: usize) -> isize {
    let mut x_max: isize = 0;
    let mut x_min: isize = 0;
    let mut y_min: isize = 0;
    let mut y_max: isize = 0;
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut count = n - 1;
    let mut dir = Direction::Right;
    loop {
        if count == 0 {
            return x.abs() + y.abs();
        }
        count -= 1;
        match dir {
            Direction::Up => {
                y += 1;
                if y > y_max {
                    y_max = x;
                    dir = Direction::Left;
                }
            }
            Direction::Down => {
                y -= 1;
                if y < y_min {
                    y_min = y;
                    dir = Direction::Right;
                }
            }
            Direction::Left => {
                x -= 1;
                if x < x_min {
                    x_min = x;
                    dir = Direction::Down;
                }
            }
            Direction::Right => {
                x += 1;
                if x > x_max {
                    x_max = x;
                    dir = Direction::Up;
                }
            }
        }
    }
}

fn step2(n: usize) -> usize {
    let mut hmap = HashMap::new();
    hmap.insert((0, 0), 1);
    // TODO
    // Add a HashMap? Ineffective but fast to implement!
    let mut x_max: isize = 0;
    let mut x_min: isize = 0;
    let mut y_min: isize = 0;
    let mut y_max: isize = 0;
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut dir = Direction::Right;

    loop {
        match dir {
            Direction::Up => {
                y += 1;
                if y > y_max {
                    y_max = x;
                    dir = Direction::Left;
                }
            }
            Direction::Down => {
                y -= 1;
                if y < y_min {
                    y_min = y;
                    dir = Direction::Right;
                }
            }
            Direction::Left => {
                x -= 1;
                if x < x_min {
                    x_min = x;
                    dir = Direction::Down;
                }
            }
            Direction::Right => {
                x += 1;
                if x > x_max {
                    x_max = x;
                    dir = Direction::Up;
                }
            }
        }
        let value: usize = [
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y - 1),
            (x, y + 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
        ]
        .iter()
        .map(|(abs, ord)| *hmap.get(&(*abs, *ord)).unwrap_or(&0))
        .sum();
        hmap.insert((x, y), value);
        if *hmap.get(&(x, y)).unwrap() >= n {
            return *hmap.get(&(x, y)).unwrap();
        }
    }
}
