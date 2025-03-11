use crate::advent::Solver;
use itertools::Itertools;

pub struct Day19;

impl Solver for Day19 {
    type Input = Parsed;

    fn parse(&self, input: &str, _: bool) -> Self::Input {
        let mut split = input.split("\n\n");
        let towels: Vec<String> = split.next().unwrap().split(", ").map(|s| s.to_string()).collect();
        let patterns: Vec<String> = split.next().unwrap().lines().map(|s| s.to_string()).collect();

        Parsed { towels, patterns }
    }

    fn part_1(&self, input: &mut Self::Input) -> String {
        let regex = format!(
            "^(:?{})+$",
            input
                .towels
                .iter()
                .map(|t| if t.len() == 1 { t.to_string() } else { format!("(:?{})", t) })
                .join("|")
        );
        let re = regex::Regex::new(&regex).unwrap();
        input.patterns.iter().filter(|p| re.is_match(p)).count().to_string()
    }

    fn part_2(&self, input: &mut Self::Input) -> String {
        "foo".to_string()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("", "")
    }

    fn name(&self) -> &'static str {
        "Linen Layout"
    }
}

pub struct Parsed {
    towels: Vec<String>,
    patterns: Vec<String>,
}
