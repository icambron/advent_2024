use crate::advent::{Day, Solver};
use std::fs::File;
use std::io;
use std::io::BufRead;

pub struct Day02;

impl Solver for Day02 {
    fn run(&self, day: Day) -> (u64, u64) {
        let parsed = parse_file(&day.path());
        (part_1(&parsed), part_2(&parsed))
    }

    fn expected(&self) -> (u64, u64) {
        (332, 398)
    }
}

fn part_1(parsed: &[Vec<i8>]) -> u64 {
    parsed
        .iter()
        .filter(|report| is_report_safe(report, None))
        .count() as u64
}

fn part_2(parsed: &[Vec<i8>]) -> u64 {
    parsed
        .iter()
        .filter(|report| {
            report
                .iter()
                .enumerate()
                .any(|(i, _)| is_report_safe(report, Some(i)))
        })
        .count() as u64
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

fn parse_file(file: &str) -> Vec<Vec<i8>> {
    let file = File::open(file).expect("Should be able to open file");
    let lines = io::BufReader::new(file).lines();
    let mut reports = Vec::with_capacity(1000);
    for line in lines.map_while(|l| l.ok()) {
        let levels: Vec<i8> = line
            .split_whitespace()
            .map(|s| s.parse::<i8>().expect("Text should be a number"))
            .collect();
        reports.push(levels);
    }
    reports
}
