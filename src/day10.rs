use crate::advent::Solver;
use std::collections::{BTreeMap, BTreeSet};

pub struct Day10;

impl Solver for Day10 {
    type Input = Map;

    fn parse(&self, input: &str) -> Self::Input {
        let mut map = CharMap::new();
        let mut width = 0;
        let mut first_row = true;
        let mut size = 0;
        for c in input.chars() {
            if c.is_ascii_digit() {
                if first_row {
                    width += 1;
                }

                let num = c.to_digit(10).unwrap() as usize;
                map.entry(num)
                    .and_modify(|e| {
                        e.insert(size);
                    })
                    .or_insert_with(|| BTreeSet::from([size]));

                size += 1;
            } else {
                first_row = false;
            }
        }

        Map { chars: map, width, size }
    }

    fn part_1(&self, input: &mut Self::Input) -> u64 {
        let progress = compute(input);
        progress.values().fold(0, |nines, agg| nines + agg.zeros.len() as u64)
    }

    fn part_2(&self, input: &mut Self::Input) -> u64 {
        let progress = compute(input);
        progress.values().fold(0, |distinct, agg| distinct + agg.perm)
    }

    fn expected(&self) -> (u64, u64) {
        (617, 1477)
    }

    fn name(&self) -> &'static str {
        "Hoof It (topographic map)"
    }
}

fn compute(map: &Map) -> BTreeMap<usize, TrailAgg> {
    let mut progress: BTreeMap<usize, TrailAgg> = map
        .chars
        .get(&0)
        .unwrap()
        .iter()
        .map(|coord| (*coord, TrailAgg::new(*coord)))
        .collect();

    for i in 1..=9 {
        let next_set = map.chars.get(&i).unwrap();

        let mut next_progress: BTreeMap<usize, TrailAgg> = BTreeMap::new();

        for (coord, agg) in progress.into_iter() {
            let up = coord.overflowing_sub(map.width).0;
            let down = coord + map.width;
            let left = if coord % map.width == 0 { usize::MAX } else { coord - 1 };
            let right = if coord % map.width == map.width - 1 {
                usize::MAX
            } else {
                coord + 1
            };

            for dir in [up, down, left, right].iter() {
                if *dir >= map.size {
                    continue;
                }

                if next_set.contains(dir) {
                    next_progress
                        .entry(*dir)
                        .and_modify(|e| {
                            e.extend(&agg);
                        })
                        .or_insert_with(|| agg.clone());
                }
            }
        }

        progress = next_progress;
    }

    progress
}

type CharMap = BTreeMap<usize, BTreeSet<usize>>;

#[derive(Debug)]
pub struct Map {
    chars: CharMap,
    width: usize,
    size: usize,
}

#[derive(Debug, Clone)]
struct TrailAgg {
    perm: u64,
    zeros: BTreeSet<usize>,
}

impl TrailAgg {
    fn new(seed: usize) -> Self {
        TrailAgg {
            perm: 1,
            zeros: BTreeSet::from([seed]),
        }
    }

    fn extend(&mut self, other: &TrailAgg) {
        self.perm += other.perm;
        self.zeros.extend(&other.zeros);
    }
}
