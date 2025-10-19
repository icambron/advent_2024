use hashbrown::HashMap;

use crate::advent::Solver;

pub struct Day22;

impl Solver for Day22 {
    type Input = Vec<u32>;

    fn parse(&self, input: &str, _is_sample: bool) -> Self::Input {
        input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.parse().unwrap())
            .collect()
    }

    fn part_1(&self, input: &mut Self::Input) -> String {
        let mut total: u64 = 0;
        for secret in input {
            let mut accum = *secret;
            for _ in 0..2000 {
                accum = next(accum);
            }

            total += accum as u64;
        }

        total.to_string()
    }

    fn part_2(&self, input: &mut Self::Input) -> String {
        let mut map: HashMap<[i32; 4], (usize, u64)> = HashMap::new();

        for (secret_index, secret_start) in input.iter_mut().enumerate() {
            let mut three: i32 = 0;
            let mut two: i32 = 0;
            let mut one: i32 = 0;
            let mut previous = 0;
            let mut next_secret = *secret_start;
            // one extra loop because we need the digit from the seed
            for iteration in 0..2001 {
                let last_digit = (next_secret % 10) as i32;
                let delta = last_digit - previous;
                let last_four = [three, two, one, delta];

                if iteration > 3 {
                    map.entry(last_four)
                        .and_modify(|(index, total)| {
                            if *index != secret_index {
                                *index = secret_index;
                                *total += last_digit as u64;
                            }
                        })
                        .or_insert((secret_index, last_digit as u64));
                }

                previous = last_digit;
                three = two;
                two = one;
                one = delta;
                next_secret = next(next_secret);
            }
        }

        let (_, (_, total)) = map.iter().max_by(|(_, (_, total1)), (_, (_, total2))| total1.cmp(total2)).unwrap();

        total.to_string()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("13185239446", "?")
    }

    fn name(&self) -> &'static str {
        "Monkey Market"
    }
}

const MASK: u32 = 0x00ffffff;

fn next(secret: u32) -> u32 {
    let secret = ((secret << 6) ^ secret) & MASK;
    let secret = ((secret >> 5) ^ secret) & MASK;
    ((secret << 11) ^ secret) & MASK
}
