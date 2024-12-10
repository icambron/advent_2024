use std::collections::{BTreeMap, BTreeSet};
use crate::advent::Solver;

pub struct Day10;

impl Solver for Day10 {
    fn run(&self, input: &str) -> (u64, u64) {
        let map = parse(input);
        // println!("{:?}", map);
        let p1 = part_1(&map);
        
        (p1, 0)
    }

    fn expected(&self) -> (u64, u64) {
        (617, 0)
    }
}

fn part_1(map: &Map) -> u64 {
    let mut progress : Tracker = map.chars.get(&9).unwrap().iter()
        .map(|coord| (*coord, BTreeSet::from([*coord])))
        .collect();
    
    for i in (0..=8).rev() {
        let next_set = map.chars.get(&i).unwrap();
        
        let mut next_progress = Tracker::new();
        
        for (coord, nines) in progress.into_iter() {
            let up = coord - map.width;
            let down = coord + map.width;
            let left = if coord % map.width == 0 {usize::MAX} else { coord - 1 };
            let right = if coord % map.width == map.width - 1 { usize::MAX} else { coord + 1};
            
            for dir in [up, down, left, right].iter() {
                if *dir >= map.size {
                    continue;
                }
                
                if next_set.contains(dir) {
                    next_progress.entry(*dir)
                        .and_modify(|e| { e.extend(nines.clone()); })
                        .or_insert_with(|| nines.clone());
                }
            }
        }
        
        progress = next_progress;
    }
    
    progress.values().map(|nines| nines.len() as u64).sum()

}

fn parse(input: &str) -> Map {
    let mut map = Tracker::new();
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
                .and_modify(|e| { e.insert(size); })
                .or_insert_with(|| { BTreeSet::from([size]) });
            
            size += 1;
        } else {
            first_row = false;
        }
    }
    
    Map { chars: map, width, size }
}

type Tracker = BTreeMap<usize, BTreeSet<usize>>;

#[derive(Debug)]
struct Map {
    chars: Tracker,
    width: usize,
    size: usize,
}
