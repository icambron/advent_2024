use std::collections::BTreeMap;

use crate::advent::Solver;

pub struct Day25;

impl Solver for Day25 {
    type Input = LocksAndKeys;

    fn parse(&self, input: &str, _is_sample: bool) -> Self::Input {
        let mut locks = Vec::new();
        let mut keys = Vec::new();
        for key_or_lock in input.split("\n\n") {
            let mut is_lock = false;
            let mut pins = [0u8; 5];
            for (i, row) in key_or_lock.lines().enumerate() {
                if i == 0 {
                    is_lock = row.starts_with("#");
                } else if !is_lock && i == 6 {
                    // if it's a key, we don't count the last row
                    continue;
                } else {
                    for (j, c) in row.chars().enumerate() {
                        if let '#' = c {
                            pins[j] += 1;
                        }
                    }
                }
            }
            if is_lock {
                locks.push(pins);
            } else {
                keys.push(pins);
            }
        }

        LocksAndKeys { locks, keys }
    }

    fn part_1(&self, input: &mut Self::Input) -> String {
        let mut root = TrieNode { children: BTreeMap::new() };
        for key in &input.locks {
            let mut node = &mut root;
            for height in key {
                node = node.children.entry(*height).or_insert(TrieNode { children: BTreeMap::new() });
            }
        }

        // println!("Trie {:#?}", root);

        let mut stack = input.keys.iter().map(|k| (k, &root, 0)).collect::<Vec<_>>();

        let mut total = 0;
        while let Some((key, node, depth)) = stack.pop() {
            if depth == 5 {
                total += 1;
            } else {
                let key_height = key[depth];
                for (_, child_node) in node.children.iter().filter(|(height, _)| 5 - key_height >= **height) {
                    stack.push((key, child_node, depth + 1));
                }
            }
        }

        total.to_string()
    }

    fn part_2(&self, _input: &mut Self::Input) -> String {
        "dne".to_string()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("3619", "dne")
    }

    fn name(&self) -> &'static str {
        "Code Chronicle"
    }
}

#[derive(Debug)]
pub struct LocksAndKeys {
    locks: Vec<[u8; 5]>,
    keys: Vec<[u8; 5]>,
}

#[derive(Debug)]
struct TrieNode {
    children: BTreeMap<u8, TrieNode>,
}
