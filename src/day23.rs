use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

use crate::advent::Solver;

pub struct Day23;

impl Solver for Day23 {
    type Input = HashSet<(String, String)>;

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

    fn part_2(&self, input: &mut Self::Input) -> String {
        let mut conn_map: HashMap<String, imbl::HashSet<String>> = HashMap::new();
        for (first, second) in input.iter() {
            conn_map.entry(first.clone()).or_insert(imbl::HashSet::new()).insert(second.clone());
            conn_map.entry(second.clone()).or_insert(imbl::HashSet::new()).insert(first.clone());
        }

        let mut max_found: Option<imbl::HashSet<String>> = None;
        let mut max_len = 0;

        let empty = imbl::HashSet::new();
        let mut stack = conn_map
            .iter()
            .map(|(node, neighbors)| Candidate {
                next: node.clone(),
                path: empty.update(node.clone()),
                pool: neighbors.update(node.clone()),
            })
            .collect::<Vec<_>>();

        let mut visited = HashSet::new();

        while let Some(can) = stack.pop() {
            if visited.contains(&can.next) {
                continue;
            }

            visited.insert(can.next.clone());

            if can.pool.len() < max_len {
                continue;
            }

            let neighbors = conn_map.get(&can.next).expect("Node not found in connection map");

            let intersection = neighbors.clone().intersection(can.pool).update(can.next.clone());
            if intersection.len() <= max_len {
                continue;
            }

            let diff = intersection.clone().relative_complement(can.path.clone());
            if diff.is_empty() {
                if can.path.len() > max_len {
                    max_len = can.path.len();
                    max_found = Some(can.path.clone());
                }
            } else {
                stack.extend(diff.into_iter().map(|node| Candidate {
                    next: node.clone(),
                    path: can.path.update(node),
                    pool: intersection.clone(),
                }));
            }
        }
        max_found.unwrap().iter().sorted().join(",")
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("1476", "co,ef,eu,ff,fh,mt,qi,th,ti,vi,xe,yw")
    }

    fn name(&self) -> &'static str {
        "LAN Party"
    }
}

#[derive(Debug)]
struct Candidate {
    // todo represent as numbers?
    next: String,
    path: imbl::HashSet<String>,
    pool: imbl::HashSet<String>,
}
