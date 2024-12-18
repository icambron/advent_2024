use crate::advent::Solver;
use std::collections::BTreeMap;

pub struct Day01;

impl Solver for Day01 {
    type Input = (Vec<i32>, Vec<i32>);

    fn parse(&self, input: &str, _: bool) -> Self::Input {
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

    fn part_1(&self, (list_a, list_b): &mut Self::Input) -> String {
        let r = list_a.iter().zip(list_b.iter()).fold(0, |sum, (a, b)| sum + (b - a).unsigned_abs());
        r.to_string()
    }

    fn part_2(&self, (list_a, list_b): &mut Self::Input) -> String {
        let mut hash = BTreeMap::new();
        for n in list_b.iter() {
            hash.entry(n).and_modify(|e| *e += 1).or_insert(1);
        }

        let sum_similarity = list_a.iter().fold(0, |sum, val| {
            let similarity = hash.get(val).unwrap_or(&0);
            sum + similarity * val
        });

        sum_similarity.to_string()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("2756096", "23117829")
    }

    fn name(&self) -> &'static str {
        "Historian Hysteria (list similarity)"
    }
}
