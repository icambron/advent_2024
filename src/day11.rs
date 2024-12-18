use crate::advent::Solver;
use hashbrown::HashMap;

pub struct Day11;

impl Solver for Day11 {
    type Input = Vec<u64>;

    fn parse(&self, input: &str, _: bool) -> Self::Input {
        input.split_whitespace().map(|s| s.parse().unwrap()).collect()
    }

    fn part_1(&self, input: &mut Self::Input) -> String {
        let mut lookup = HashMap::with_capacity(140_000);
        count_stones(input, 25, &mut lookup).to_string()
    }

    fn part_2(&self, input: &mut Self::Input) -> String {
        let mut lookup = HashMap::with_capacity(140_000);
        count_stones(input, 75, &mut lookup).to_string()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("188902", "223894720281135")
    }

    fn name(&self) -> &'static str {
        "Plutonian Pebbles"
    }
}

fn count_stones(stones: &[u64], max_gens: u8, lookup: &mut HashMap<(u64, u8), u64>) -> u64 {
    stones.iter().fold(0, |acc, stone| acc + count_stone(*stone, max_gens, lookup))
}

fn count_stone(stone: u64, gens_left: u8, lookup: &mut HashMap<(u64, u8), u64>) -> u64 {
    if gens_left == 0 {
        return 1;
    }

    let key = (stone, gens_left);

    if let Some(count) = lookup.get(&key) {
        return *count;
    }

    let (first, second) = next_stone(stone);
    let next_gen = gens_left - 1;

    let mut count = count_stone(first, next_gen, lookup);
    if let Some(second) = second {
        count += count_stone(second, next_gen, lookup);
    }

    lookup.insert(key, count);

    count
}

fn next_stone(stone: u64) -> (u64, Option<u64>) {
    if stone == 0 {
        return (1, None);
    }

    if let Some((first, second)) = split_in_two(stone) {
        return (first, Some(second));
    }

    (stone * 2024, None)
}

fn split_in_two(num: u64) -> Option<(u64, u64)> {
    let num_digits = ((num as f64).log10() + 1.0) as u32;

    if num_digits % 2 != 0 {
        return None;
    }

    let split_position = num_digits / 2;

    let divisor = 10u64.pow(split_position);

    let high = num / divisor;
    let low = num % divisor;

    Some((high, low))
}
