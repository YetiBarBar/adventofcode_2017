#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}
fn main() {
    let data = include_str!("../data/day19.data")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let (res, count) = process(&data);
    println!("Part 1: {}", res);
    println!("Part 2: {}", count);
}

fn process(data: &[Vec<char>]) -> (String, i32) {
    let mut x: usize = data[0]
        .iter()
        .enumerate()
        .find(|(_, chr)| chr == &&'|')
        .map(|(pos, _)| pos)
        .unwrap();

    let mut y = 0;
    let mut res: Vec<char> = Vec::new();
    let mut dir = Direction::Down;
    let mut count = 0;
    while let Some(value) = get_x_y(x, y, data) {
        count += 1;

        match dir {
            Direction::Up => match value {
                '|' | '-' => {
                    y -= 1;
                }
                '+' => {
                    if data.get(y).unwrap().get(x.saturating_sub(1)) == Some(&'-') {
                        x -= 1;
                        dir = Direction::Left;
                    } else {
                        x += 1;
                        dir = Direction::Right;
                    }
                }
                ' ' => (),
                _ => {
                    res.push(*value);
                    y -= 1;
                }
            },
            Direction::Down => match value {
                '|' | '-' => {
                    y += 1;
                }
                '+' => {
                    if data.get(y).unwrap().get(x.saturating_sub(1)) == Some(&'-') {
                        x -= 1;
                        dir = Direction::Left;
                    } else {
                        x += 1;
                        dir = Direction::Right;
                    }
                }
                ' ' => (),
                'T' => {
                    res.push('T');
                    break;
                }
                _ => {
                    res.push(*value);
                    y += 1;
                }
            },
            Direction::Right => match value {
                '|' | '-' => {
                    x += 1;
                }
                '+' => {
                    if data.get(y.saturating_sub(1)).unwrap().get(x) == Some(&'|') {
                        y -= 1;
                        dir = Direction::Up;
                    } else {
                        y += 1;
                        dir = Direction::Down;
                    }
                }
                ' ' => (),
                _ => {
                    res.push(*value);
                    x += 1;
                }
            },
            Direction::Left => match value {
                '|' | '-' => {
                    x -= 1;
                }
                '+' => {
                    if data.get(y.saturating_sub(1)).unwrap().get(x) == Some(&'|') {
                        y -= 1;
                        dir = Direction::Up;
                    } else {
                        y += 1;
                        dir = Direction::Down;
                    }
                }
                ' ' => (),
                _ => {
                    res.push(*value);
                    x -= 1;
                }
            },
        }
    }
    (res.iter().collect(), count)
}

fn get_x_y(x: usize, y: usize, data: &[Vec<char>]) -> Option<&char> {
    data.get(y)?.get(x)
}
