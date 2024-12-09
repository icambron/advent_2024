use crate::advent::Solver;
use itertools::Itertools;
use std::collections::HashMap;

pub struct Day08;

impl Solver for Day08 {
    fn run(&self, input: &str) -> (u64, u64) {
        let parsed = parse(input);
        (part_1(&parsed), part_2(&parsed))
    }

    fn expected(&self) -> (u64, u64) {
        (261, 898)
    }
}

fn part_1(parsed: &Parsed) -> u64 {
    let mut antinode_map = vec![false; parsed.width * parsed.height];
    for (_, antennas) in parsed.antennas.iter() {
        for pair in antennas.iter().combinations(2) {
            let (a, b) = (pair[0], pair[1]);

            let option_1 = Pos::new(2 * a.pos.x - b.pos.x, 2 * a.pos.y - b.pos.y);
            let option_2 = Pos::new(2 * b.pos.x - a.pos.x, 2 * b.pos.y - a.pos.y);

            for option in &[option_1, option_2] {
                if option.x >= 0 && option.y >= 0 && option.x < parsed.width as i64 && option.y < parsed.height as i64 {
                    if let Some(slot) = antinode_map.get_mut(option.y as usize * parsed.width + option.x as usize) {
                        *slot = true;
                    }
                }
            }
        }
    }

    antinode_map.iter().filter(|&a| *a).count() as u64
}

fn part_2(parsed: &Parsed) -> u64 {
    let formulas: Vec<Resonation> = parsed.antennas.iter().flat_map(|(_, antennas)| {
        antennas.iter().combinations(2).map(|pair| {
            let (a, b) = (pair[0], pair[1]);
            Resonation {
                delta_x: a.pos.x - b.pos.x,
                delta_y: a.pos.y - b.pos.y,
                anchor: a.pos.clone(),
            }
        })
    }).collect();
    
    let mut count = 0;
    for x in 0..parsed.width as i64 {
        for y in 0..parsed.height as i64 {
            for formula in &formulas {
                let delta_x = x - formula.anchor.x;
                let delta_y = y - formula.anchor.y;
                
                if delta_y * formula.delta_x == delta_x * formula.delta_y {
                    count += 1;
                    break;
                }
            }
        }
    }
    
    count
}

fn parse(input: &str) -> Parsed {
    let mut height = 0;
    let mut width = 0;
    let mut antennas = Vec::new();

    for (y, line) in input.lines().enumerate() {
        if y == 0 {
            width = line.len();
        }
        height += 1;
        for (x, c) in line.chars().enumerate() {
            if let 'a'..='z' | 'A'..='Z' | '0'..='9' = c {
                antennas.push(Antenna {
                    freq: c,
                    pos: Pos::new(x as i64, y as i64)
                });
            }
        }
    }

    let antennas: HashMap<char, Vec<Antenna>> = antennas.iter().fold(HashMap::new(), |mut map, a| {
        map.entry(a.freq).or_default().push(a.clone());
        map
    });

    Parsed { antennas, width, height }
}

#[derive(Debug)]
struct Parsed {
    antennas: HashMap<char, Vec<Antenna>>,
    width: usize,
    height: usize,
}

#[derive(Debug, Clone)]
struct Antenna {
    freq: char,
    pos: Pos
}

#[derive(Debug, Clone)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Resonation {
    delta_x: i64,
    delta_y: i64,
    anchor: Pos,
}