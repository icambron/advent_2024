use crate::advent::Solver;
use std::collections::BTreeMap;

pub struct Day01;

impl Solver for Day01 {
    fn part_1(&self, input: &str) -> u64 {
        let (list_a, list_b) = parse(input);
        part_1(&list_a, &list_b)
    }

    fn part_2(&self, input: &str) -> u64 {
        let (list_a, list_b) = parse(input);
        part_2(&list_a, &list_b)
    }

    fn expected(&self) -> (u64, u64) {
        (2756096, 23117829)
    }
}

fn part_1(list_a: &[i32], list_b: &[i32]) -> u64 {
    list_a.iter().zip(list_b.iter()).fold(0, |sum, (a, b)| sum + (b - a).unsigned_abs()) as u64
}

fn part_2(list_a: &[i32], list_b: &[i32]) -> u64 {
    let mut hash = BTreeMap::new();
    for n in list_b.iter() {
        hash.entry(n).and_modify(|e| *e += 1).or_insert(1);
    }

    let sum_similarity = list_a.iter().fold(0, |sum, val| {
        let similarity = hash.get(val).unwrap_or(&0);
        sum + similarity * val
    });

    sum_similarity as u64
}

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut list_a = Vec::with_capacity(1000);
    let mut list_b = Vec::with_capacity(1000);

    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Text should be a number"))
            .collect();
        if numbers.len() == 2 {
            list_a.push(numbers[0]);
            list_b.push(numbers[1]);
        } else {
            panic!("Invalid input");
        }
    }

    list_a.sort_unstable();
    list_b.sort_unstable();

    (list_a, list_b)
}
