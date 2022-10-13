fn main() {
    println!("Part 1: {}", compute_position(289_326));
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

fn step2(n: usize) -> isize {
    // TODO
    // Add a HashMap? Ineffective but fast to implement!
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
