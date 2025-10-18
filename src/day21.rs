use crate::advent::Solver;
use std::collections::{BTreeMap, HashMap, HashSet};

pub struct Day21;

impl Solver for Day21 {
    type Input = Puzzle;

    fn parse(&self, input: &str, _is_sample: bool) -> Self::Input {
        let seqs = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

        let numeric_mapping = compute_numeric_mapping();
        let direction_mapping = compute_direction_mapping();

        Puzzle {
            seqs,
            numeric_mapping,
            direction_mapping,
        }
    }

    fn part_1(&self, input: &mut Self::Input) -> String {
        solve(input, 2).to_string()
    }

    fn part_2(&self, input: &mut Self::Input) -> String {
        solve(input, 25).to_string()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("238078", "293919502998014")
    }

    fn name(&self) -> &'static str {
        "Keypad Conundrum"
    }
}

pub struct Puzzle {
    seqs: Vec<Vec<char>>,
    numeric_mapping: HashMap<(char, char), Vec<char>>,
    direction_mapping: HashMap<(char, char), Vec<char>>,
}

fn solve(puzzle: &Puzzle, direction_layers: u16) -> u64 {
    let mut total = 0;
    let mut cache = HashMap::new();

    for seq in &puzzle.seqs {
        let numeric = next_layer(seq.clone(), &puzzle.numeric_mapping);
        let shortest_length = solve_seq(numeric, direction_layers, &puzzle.direction_mapping, &mut cache);

        let numeric = seq.iter().filter(|&c| c.is_numeric()).collect::<String>().parse::<u64>().unwrap();
        let subtotal = numeric * shortest_length;

        total += subtotal;
    }
    total
}

fn pairs(input: Vec<char>) -> Vec<(char, char)> {
    let left_iter = std::iter::once('A').chain(input.iter().copied());
    left_iter.zip(input.iter().copied()).collect()
}

fn solve_seq(
    motion: Vec<char>,
    layer: u16,
    mapping: &HashMap<(char, char), Vec<char>>,
    memo: &mut HashMap<((char, char), u16), u64>,
) -> u64 {
    if layer == 0 {
        motion.len() as u64
    } else {
        let to_add = pairs(motion);
        let mut accum = 0;
        for next_pair in to_add {
            let val = if let Some(cached) = memo.get(&(next_pair, layer)) {
                *cached
            } else {
                let next_motion = mapping.get(&next_pair).expect("Missing mapping");
                let val = solve_seq(next_motion.clone(), layer - 1, mapping, memo);
                memo.insert((next_pair, layer), val);
                val
            };

            accum += val;
        }

        accum
    }
}

fn next_layer(input: Vec<char>, mapping: &HashMap<(char, char), Vec<char>>) -> Vec<char> {
    let mut current = 'A';

    let mut result = vec![];
    for item in input {
        if current != item {
            let motion = mapping.get(&(current, item)).expect("missing mapping");
            result.extend(motion);
            current = item;
        }
    }

    result.into_iter().collect()
}

fn compute_numeric_mapping() -> HashMap<(char, char), Vec<char>> {
    let locations = BTreeMap::from([
        ('7', (0, 0)),
        ('8', (1, 0)),
        ('9', (2, 0)),
        ('4', (0, 1)),
        ('5', (1, 1)),
        ('6', (2, 1)),
        ('1', (0, 2)),
        ('2', (1, 2)),
        ('3', (2, 2)),
        ('0', (1, 3)),
        ('A', (2, 3)),
    ]);

    compute_mapping(locations)
}

fn compute_direction_mapping() -> HashMap<(char, char), Vec<char>> {
    let locations = BTreeMap::from([('^', (1, 0)), ('A', (2, 0)), ('<', (0, 1)), ('v', (1, 1)), ('>', (2, 1))]);
    compute_mapping(locations)
}

fn is_valid_path(start: &(usize, usize), path: Vec<char>, valid_coords: &HashSet<&(usize, usize)>) -> bool {
    let mut loc = *start;

    for action in path {
        match action {
            '<' => loc.0 -= 1,
            '>' => loc.0 += 1,
            '^' => loc.1 -= 1,
            'v' => loc.1 += 1,
            'A' => {}
            _ => unreachable!(),
        }

        if !valid_coords.contains(&&loc) {
            return false;
        }
    }

    true
}

// Represent the preferred ordering of motions as an enum instead of string literals.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum MotionOrder {
    LeftRight,
    UpDown,
}

fn decide_best_motion(preferred_order: &[MotionOrder], x_motion: &[char], y_motion: &[char]) -> Vec<char> {
    let mut result = Vec::new();

    for &item in preferred_order.iter() {
        match item {
            MotionOrder::LeftRight => result.extend_from_slice(x_motion),
            MotionOrder::UpDown => result.extend_from_slice(y_motion),
        }
    }

    result
}

fn compute_mapping(locations: BTreeMap<char, (usize, usize)>) -> HashMap<(char, char), Vec<char>> {
    let mut map = HashMap::new();

    let valid_coords = locations.values().collect::<HashSet<_>>();

    for (start_char, start_loc) in locations.iter() {
        for (end_char, end_loc) in locations.iter() {
            if start_char == end_char {
                map.insert((*start_char, *end_char), vec!['A']);
            } else {
                let x_diff = end_loc.0 as isize - start_loc.0 as isize;
                let y_diff = end_loc.1 as isize - start_loc.1 as isize;

                let x_count = x_diff.unsigned_abs();
                let y_count = y_diff.unsigned_abs();

                let x_motion = if x_count == 0 {
                    Vec::new()
                } else {
                    let action = if x_diff < 0 { '<' } else { '>' };
                    vec![action; x_count]
                };

                let y_motion = if y_count == 0 {
                    Vec::new()
                } else {
                    let action = if y_diff < 0 { '^' } else { 'v' };
                    vec![action; y_count]
                };

                let mut preferred_order = if x_diff < 0 {
                    vec![MotionOrder::LeftRight, MotionOrder::UpDown]
                } else {
                    vec![MotionOrder::UpDown, MotionOrder::LeftRight]
                };

                let mut best_motion = decide_best_motion(&preferred_order, &x_motion, &y_motion);

                if !is_valid_path(start_loc, best_motion.clone(), &valid_coords) {
                    preferred_order.reverse();

                    best_motion = decide_best_motion(&preferred_order, &x_motion, &y_motion);

                    if !is_valid_path(start_loc, best_motion.clone(), &valid_coords) {
                        panic!("Invalid path");
                    }
                }

                best_motion.push('A');
                map.insert((*start_char, *end_char), best_motion);
            }
        }
    }

    map
}
