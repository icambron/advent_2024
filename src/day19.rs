use hashbrown::{HashMap, HashSet};

use crate::advent::Solver;

pub struct Day19;

impl Solver for Day19 {
    type Input = Towels;

    fn parse(&self, input: &str, _: bool) -> Self::Input {
        let split = input.split("\n\n").collect::<Vec<&str>>();

        let pattern_line = split[0].lines().next().unwrap();
        let patterns = pattern_line.split(",").map(|e| e.trim().to_string()).collect();
        let designs = split[1].lines().map(|line| line.to_string()).collect();
        Towels { patterns, designs }
    }

    fn part_1(&self, input: &mut Self::Input) -> String {
        let mut count_possible = 0;
        for design in &input.designs {
            if is_solveable(design, &input.patterns) {
                count_possible += 1;
            }
        }
        count_possible.to_string()
    }

    fn part_2(&self, input: &mut Self::Input) -> String {
        let mut total = 0;
        let mut memo = HashMap::new();
        for design in &input.designs {
            total += ways_to_solve(design, &input.patterns, &mut memo);
        }
        total.to_string()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        todo!()
    }

    fn name(&self) -> &'static str {
        "Linen Layout"
    }
}

fn is_solveable(design: &str, patterns: &Vec<String>) -> bool {
    let mut visited = HashSet::new();
    let mut possible_matches = vec![design];
    while let Some(design) = possible_matches.pop() {
        if visited.contains(&design) {
            continue;
        }
        visited.insert(design);
        for pattern in patterns {
            if design.starts_with(pattern) {
                let substr = &design[pattern.len()..];
                if substr.is_empty() {
                    return true;
                }
                possible_matches.push(substr)
            }
        }
    }

    false
}

fn ways_to_solve<'a>(design: &'a str, patterns: &Vec<String>, memo: &mut HashMap<&'a str, u64>) -> u64 {
    if design.is_empty() {
        return 1;
    }

    if let Some(&ways) = memo.get(&design) {
        return ways;
    }

    let result = patterns.iter().fold(0, |acc, pattern| {
        if design.starts_with(pattern) {
            acc + ways_to_solve(&design[pattern.len()..], patterns, memo)
        } else {
            acc
        }
    });

    memo.insert(design, result);
    result
}

#[derive(Debug)]
pub struct Towels {
    patterns: Vec<String>,
    designs: Vec<String>,
}
