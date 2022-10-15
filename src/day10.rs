use std::collections::VecDeque;

struct KnotHasher {
    buffer: [u8; 256],
    skip_size: usize,
    current_position: usize,
}

impl KnotHasher {
    fn new() -> Self {
        let mut buffer = [0; 256];
        for idx in 0_u8..=255 {
            buffer[idx as usize] = idx;
        }
        Self {
            buffer,
            skip_size: 0,
            current_position: 0,
        }
    }

    fn reverse(&mut self, length: usize) {
        let mut reversed = (0..length).fold(Vec::new(), |mut acc, position| {
            acc.push(self.buffer[(self.current_position + position) % 256]);
            acc
        });

        for position in 0..length {
            self.buffer[(self.current_position + position) % 256] = reversed.pop().unwrap();
        }
    }

    fn advance_position(&mut self, length: usize) {
        self.current_position += length + self.skip_size;
        self.current_position %= 256;
        self.skip_size += 1;
    }

    fn apply_length(&mut self, length: usize) {
        self.reverse(length);
        self.advance_position(length);
    }

    fn encode_u8_sequence(&mut self, seq: &[u8]) {
        for length in seq {
            self.apply_length(*length as usize);
        }
    }

    fn encode_str(&mut self, data: &str) -> String {
        let mut data = Vec::from_iter(data.as_bytes().iter().copied());
        data.extend([17, 31, 73, 47, 23].iter());
        for _ in 0..64 {
            self.encode_u8_sequence(&data);
        }

        let dense_hash: Vec<u8> = self
            .buffer
            .chunks(16)
            .map(|chunk| chunk.iter().copied().reduce(|a, b| a ^ b).unwrap())
            .collect();
        println!("{:?}", dense_hash);
        dense_hash.iter().fold(String::new(), |mut acc, value| {
            acc.push_str(&format!("{:02x}", value));
            acc
        })
    }

    fn hash(input: &str) -> String {
        let mut knot = Self::new();
        knot.encode_str(input)
    }
}

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

    println!("Part 2: {}", KnotHasher::hash(input))
}
