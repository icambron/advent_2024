use crate::advent::Solver;

pub struct Day02;

impl Solver for Day02 {
    type Input = Vec<Vec<i8>>;

    fn parse(&self, input: &str) -> Self::Input {
        let mut reports = Vec::with_capacity(1000);
        for line in input.lines() {
            let levels: Vec<i8> = line
                .split_whitespace()
                .map(|s| s.parse::<i8>().expect("Text should be a number"))
                .collect();
            reports.push(levels);
        }
        reports
    }

    fn part_1(&self, input: &mut Self::Input) -> String {
        input.iter().filter(|report| is_report_safe(report, None)).count().to_string()
    }

    fn part_2(&self, input: &mut Self::Input) -> String {
        input
            .iter()
            .filter(|report| report.iter().enumerate().any(|(i, _)| is_report_safe(report, Some(i))))
            .count()
            .to_string()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("332", "398")
    }

    fn name(&self) -> &'static str {
        "Red-Nosed Reports"
    }
}

fn is_report_safe(report: &[i8], exclude_index: Option<usize>) -> bool {
    let mut state = ReportState::Start;

    for (i, level) in report.iter().cloned().enumerate() {
        if Some(i) == exclude_index {
            continue;
        }

        state = match state {
            ReportState::Start => ReportState::First(level),
            ReportState::Up(prev) => {
                if ok_higher(prev, level) {
                    ReportState::Up(level)
                } else {
                    return false;
                }
            }
            ReportState::Down(prev) => {
                if ok_lower(prev, level) {
                    ReportState::Down(level)
                } else {
                    return false;
                }
            }
            ReportState::First(prev) => {
                if ok_higher(prev, level) {
                    ReportState::Up(level)
                } else if ok_lower(prev, level) {
                    ReportState::Down(level)
                } else {
                    return false;
                }
            }
        };
    }
    true
}

fn ok_higher(prev: i8, level: i8) -> bool {
    level > prev && level <= prev + 3
}

fn ok_lower(prev: i8, level: i8) -> bool {
    level < prev && level >= prev - 3
}

enum ReportState {
    Start,
    First(i8),
    Up(i8),
    Down(i8),
}
