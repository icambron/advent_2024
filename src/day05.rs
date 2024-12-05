use crate::advent::Advent;
use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn run(advent: Advent) {
    let parsed = parse_file(&advent.path());
    part_1(&parsed);
    part_2(parsed);
}

fn part_1(parsed: &Update) {
    let result = parsed
        .pages
        .iter()
        .filter(|page_set| is_correct(page_set, &parsed.rules))
        .map(|page_set| middle_page(page_set))
        .sum::<usize>();

    println!("Part 1: {}", result);
}

fn part_2(parsed: Update) {
    let sum = parsed
        .pages
        .into_iter()
        .filter(|page_set| !is_correct(page_set, &parsed.rules))
        .map(|mut page_set| {
            page_set.sort_by(|a, b| {
                if parsed.rules.contains(&(*a, *b)) {
                    std::cmp::Ordering::Less
                } else if parsed.rules.contains(&(*b, *a)) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });

            middle_page(&page_set)
        })
        .sum::<usize>();

    println!("Part 2: {}", sum);
}

fn is_correct(page_set: &[usize], rules: &HashSet<(usize, usize)>) -> bool {
    let page_map = page_set
        .iter()
        .enumerate()
        .map(|(i, p)| (p, i))
        .collect::<BTreeMap<_, _>>();
    for rule in rules {
        if let Some(first) = page_map.get(&rule.0) {
            if let Some(second) = page_map.get(&rule.1) {
                if first > second {
                    return false;
                }
            }
        }
    }
    true
}

fn middle_page(page_set: &[usize]) -> usize {
    page_set[(page_set.len() - 1) / 2]
}

fn parse_file(file: &str) -> Update {
    let file = File::open(file).expect("Should be able to open file");

    let lines = io::BufReader::new(file).lines();
    let mut rules = HashSet::new();
    let mut pages = Vec::new();
    for line in lines.map_while(|l| l.ok()) {
        if line.contains(",") {
            let parts = line.split(",");
            let lil_pages: Vec<usize> = parts.map(|p| p.parse().unwrap()).collect();
            pages.push(lil_pages);
        } else if line.contains("|") {
            let mut parts = line.split("|");
            let first = parts.next().unwrap().parse().unwrap();
            let second = parts.next().unwrap().parse().unwrap();
            rules.insert((first, second));
        }
    }

    Update { rules, pages }
}

#[derive(Debug)]
struct Update {
    rules: HashSet<(usize, usize)>,
    pages: Vec<Vec<usize>>,
}
