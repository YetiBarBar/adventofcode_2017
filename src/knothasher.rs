pub struct KnotHasher {
    pub buffer: [u8; 256],
    skip_size: usize,
    current_position: usize,
}

impl KnotHasher {
    pub fn new() -> Self {
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

    pub fn encode_u8_sequence(&mut self, seq: &[u8]) {
        for length in seq {
            self.apply_length(*length as usize);
        }
    }

    fn encode_str(&mut self, data: &str) -> String {
        let mut data = data.as_bytes().to_vec();
        data.extend([17, 31, 73, 47, 23].iter());
        for _ in 0..64 {
            self.encode_u8_sequence(&data);
        }

        let dense_hash: Vec<u8> = self
            .buffer
            .chunks(16)
            .map(|chunk| chunk.iter().copied().reduce(|a, b| a ^ b).unwrap())
            .collect();

        dense_hash.iter().fold(String::new(), |mut acc, value| {
            acc.push_str(&format!("{:02x}", value));
            acc
        })
    }

    pub fn hash(input: &str) -> String {
        let mut knot = Self::new();
        knot.encode_str(input)
    }
}
