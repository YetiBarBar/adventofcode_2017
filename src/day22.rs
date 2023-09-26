use std::collections::HashMap;

struct Virus {
    x: isize,
    y: isize,
    dir: Direction,
}

impl Virus {
    pub fn move_part_1(&mut self, infected: &mut HashMap<(isize, isize), Status>) -> bool {
        let res = if infected.get(&(self.x, self.y)).is_some() {
            self.rotate_right();
            infected.remove(&(self.x, self.y));
            false
        } else {
            self.rotate_left();
            infected.insert((self.x, self.y), Status::Infected);
            true
        };
        match self.dir {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        };
        res
    }

    pub fn move_part_2(&mut self, infected: &mut HashMap<(isize, isize), Status>) -> bool {
        let node_status = infected.entry((self.x, self.y)).or_insert(Status::Clean);
        *node_status = match node_status {
            Status::Infected => {
                self.rotate_right();
                Status::Flagged
            }
            Status::Clean => {
                self.rotate_left();
                Status::Weakened
            }
            Status::Flagged => {
                self.reverse_direction();
                Status::Clean
            }
            Status::Weakened => Status::Infected,
        };

        match self.dir {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        };

        *node_status == Status::Infected
    }

    fn reverse_direction(&mut self) {
        self.dir = match self.dir {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn rotate_left(&mut self) {
        self.dir = match self.dir {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn rotate_right(&mut self) {
        self.dir = match self.dir {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Eq, PartialEq, Clone, Copy)]
enum Status {
    Infected,
    Clean,
    Flagged,
    Weakened,
}

pub fn main() {
    let input = include_str!("../data/day22.data");

    let test_input = r#"..#
#..
..."#;

    let mut infected = input
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| line.chars().enumerate().map(move |(x, chr)| (x, y, chr)))
        .fold(std::collections::HashMap::new(), |mut acc, (x, y, chr)| {
            if chr == '#' {
                acc.insert((x as isize, y as isize), Status::Infected);
            };
            acc
        });

    let mut test_infected = test_input
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| line.chars().enumerate().map(move |(x, chr)| (x, y, chr)))
        .fold(std::collections::HashMap::new(), |mut acc, (x, y, chr)| {
            if chr == '#' {
                acc.insert((x as isize, y as isize), Status::Infected);
            };
            acc
        });

    let mut virus = Virus {
        x: 12,
        y: 12,
        dir: Direction::Up,
    };

    let mut test_virus = Virus {
        x: 1,
        y: 1,
        dir: Direction::Up,
    };

    let mut test_count = 0;
    let mut count = 0;
    let mut test_infected_clone = test_infected.clone();
    let mut infected_clone = infected.clone();
    for _ in 0..10_000 {
        if test_virus.move_part_1(&mut test_infected_clone) {
            test_count += 1;
        }

        if virus.move_part_1(&mut infected_clone) {
            count += 1;
        }
    }

    println!("Test part 1: {test_count}");
    println!("Part 1: {count}");

    let mut virus = Virus {
        x: 12,
        y: 12,
        dir: Direction::Up,
    };

    let mut test_virus = Virus {
        x: 1,
        y: 1,
        dir: Direction::Up,
    };

    let mut test_count = 0;
    let mut count = 0;
    for _ in 0..10_000_000 {
        if test_virus.move_part_2(&mut test_infected) {
            test_count += 1;
        }

        if virus.move_part_2(&mut infected) {
            count += 1;
        }
    }

    println!("Test part 2: {test_count}");
    println!("Part 2: {count}");
}
