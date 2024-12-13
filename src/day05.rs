use crate::advent::Solver;
use hashbrown::{HashMap, HashSet};

pub struct Day05;
impl Solver for Day05 {
    fn part_1(&self, input: &str) -> u64 {
        part_1(&parse(input))
    }

    fn part_2(&self, input: &str) -> u64 {
        part_2(parse(input))
    }

    fn expected(&self) -> (u64, u64) {
        (5948, 3062)
    }
}

fn part_1(parsed: &Update) -> u64 {
    parsed
        .pages
        .iter()
        .filter(|page_set| is_correct(page_set, &parsed.rules))
        .map(|page_set| middle_page(page_set))
        .sum::<usize>() as u64
}

fn part_2(parsed: Update) -> u64 {
    parsed
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
        .sum::<usize>() as u64
}

fn is_correct(page_set: &[usize], rules: &HashSet<(usize, usize)>) -> bool {
    let page_map = page_set.iter().enumerate().map(|(i, p)| (p, i)).collect::<HashMap<_, _>>();
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

fn parse(input: &str) -> Update {
    let mut rules = HashSet::new();
    let mut pages = Vec::new();
    for line in input.lines() {
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
