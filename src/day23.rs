use hashbrown::{HashMap, HashSet};

use crate::advent::Solver;

pub struct Day23;

impl Solver for Day23 {
    type Input = Vec<(String, String)>;

    fn parse(&self, input: &str, _is_sample: bool) -> Self::Input {
        input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|s| {
                let mut split = s.split("-");
                let first = split.next().unwrap().trim().to_string();
                let second = split.next().unwrap().trim().to_string();
                (first, second)
            })
            .collect()
    }

    fn part_1(&self, input: &mut Self::Input) -> String {
        let mut conn_map: HashMap<String, HashSet<String>> = HashMap::new();
        for (first, second) in input.iter() {
            conn_map.entry(first.clone()).or_insert(HashSet::new()).insert(second.clone());
            conn_map.entry(second.clone()).or_insert(HashSet::new()).insert(first.clone());
        }

        let mut triplets = HashSet::new();
        for (first, second) in input.iter() {
            let first_matches = conn_map.get(first).unwrap();
            let second_matches = conn_map.get(second).unwrap();

            for third in first_matches.intersection(second_matches) {
                if first.starts_with('t') || third.starts_with('t') || second.starts_with('t') {
                    let mut thing = vec![first.clone(), second.clone(), third.clone()];
                    thing.sort();
                    triplets.insert(thing);
                }
            }
        }

        triplets.len().to_string()
    }

    fn part_2(&self, _input: &mut Self::Input) -> String {
        todo!()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        todo!()
    }

    fn name(&self) -> &'static str {
        "LAN Party"
    }
}
