struct Generator {
    factor: u128,
    last_value: u128,
}

impl Generator {
    fn new(start_value: u128, factor: u128) -> Self {
        Self {
            factor,
            last_value: start_value,
        }
    }
}

impl Iterator for Generator {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        self.last_value = self.last_value * self.factor % 2_147_483_647;
        Some(self.last_value)
    }
}

#[derive(Eq, PartialEq)]
enum GenType {
    TypeA,
    TypeB,
}
struct Generator2 {
    factor: u128,
    last_value: u128,
    typ: GenType,
}

impl Generator2 {
    fn new(start_value: u128, factor: u128, typ: GenType) -> Self {
        Self {
            factor,
            last_value: start_value,
            typ,
        }
    }
}

impl Iterator for Generator2 {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.last_value = self.last_value * self.factor % 2_147_483_647;
            if self.typ == GenType::TypeA && self.last_value % 4 == 0 {
                break;
            }
            if self.typ == GenType::TypeB && self.last_value % 8 == 0 {
                break;
            }
        }
        Some(self.last_value)
    }
}

fn main() {
    // Generator A starts with 591
    // Generator B starts with 393

    let gen_a = Generator::new(591, 16807);
    let gen_b = Generator::new(393, 48271);

    let val = gen_a
        .zip(gen_b)
        .take(40_000_000)
        .filter(|(a, b)| {
            let (a, b) = (format!("{:16x}", a), format!("{:16x}", b));
            a[a.len() - 4..] == b[b.len() - 4..]
        })
        .count();
    println!("Part 1: {}", val);

    let gen_a = Generator2::new(591, 16807, GenType::TypeA);
    let gen_b = Generator2::new(393, 48271, GenType::TypeB);

    let val = gen_a
        .zip(gen_b)
        .take(5_000_000)
        .filter(|(a, b)| {
            let (a, b) = (format!("{:16x}", a), format!("{:16x}", b));
            a[a.len() - 4..] == b[b.len() - 4..]
        })
        .count();
    println!("Part 2: {}", val);
}
