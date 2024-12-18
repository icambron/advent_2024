use crate::advent::Solver;
use hashbrown::{HashMap, HashSet};

pub struct Day05;
impl Solver for Day05 {
    type Input = Update;

    fn parse(&self, input: &str) -> Self::Input {
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

    fn part_1(&self, input: &mut Self::Input) -> String {
        input
            .pages
            .iter()
            .filter(|page_set| is_correct(page_set, &input.rules))
            .map(|page_set| middle_page(page_set))
            .sum::<usize>().to_string()
    }

    fn part_2(&self, input: &mut Self::Input) -> String {
        input
            .pages
            .iter_mut()
            .filter(|page_set| !is_correct(page_set, &input.rules))
            .map(|page_set| {
                page_set.sort_by(|a, b| {
                    if input.rules.contains(&(*a, *b)) {
                        std::cmp::Ordering::Less
                    } else if input.rules.contains(&(*b, *a)) {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Equal
                    }
                });

                middle_page(&page_set)
            })
            .sum::<usize>().to_string()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("5948", "3062")
    }

    fn name(&self) -> &'static str {
        "Print Queue"
    }
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

#[derive(Debug)]
pub struct Update {
    rules: HashSet<(usize, usize)>,
    pages: Vec<Vec<usize>>,
}
