use std::fs::File;
use std::io;
use std::io::BufRead;
use crate::advent::Advent;

pub fn run(advent: Advent) {
    let parsed = parse_file(&advent.path()).expect("Failed to parse file");
    part_1(&parsed);
    part_2(&parsed);
}

fn part_1(parsed: &Vec<Vec<i8>>) {
    let count_safe = parsed.iter()
        .fold(0, |acc, report| {
            if report_safe(report, None) {
                acc + 1
            } else {
                acc
            }
        });
        
    println!("Part 1: {}", count_safe);
}

fn part_2(parsed: &Vec<Vec<i8>>) {
    let count_safe = parsed.iter().fold(0, |acc, report| {
        for (i, _) in report.iter().enumerate() {
            if report_safe(report, Some(i)) {
                return acc + 1;
            }
        }
        acc
    });
    println!("Part 2: {}", count_safe);
}

fn report_safe(report: &[i8], exclude_index: Option<usize>) -> bool {
    let mut direction  = Dir::None;
    
    let mut prev = -1;
    for (i, level) in report.iter().enumerate() {
        if Some(i) == exclude_index {
            continue;
        }
        
        let (dir, is_safe) = direction.next(prev, *level);
        direction = dir;
        prev = *level;

        if !is_safe {
            return false
        }
    }
    true
}


#[derive(PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    None,
}

impl Dir {
    fn next(self, prev: i8, level: i8) -> (Self, bool) {
        if prev == -1 {
            return (Dir::None, true);
        }
        if (prev - level).abs() > 3 {
            return (Dir::None, false);
        }
        match self {
            Dir::Up => (Dir::Up, prev < level),
            Dir::Down => (Dir::Down, prev > level),
            Dir::None => match prev.cmp(&level) {
                std::cmp::Ordering::Less => (Dir::Up, true),
                std::cmp::Ordering::Greater => (Dir::Down, true),
                std::cmp::Ordering::Equal => (Dir::None, false),
            },
        }
    }
}

fn parse_file(file: &str) -> Result<Vec<Vec<i8>>, Error> {
    let file = File::open(file)?;
    let lines = io::BufReader::new(file).lines();
    let mut reports = Vec::with_capacity(1000);
    for line in lines.map_while(|l| l.ok()) {
        let levels: Vec<i8> = line.split_whitespace()
            .map(|s| s.parse::<i8>())
            .collect::<Result<_, _>>()
            .map_err(|_| Error::BadLine(line.clone()))?;
        reports.push(levels);
        
    }
    Ok(reports)
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("Bad line: {0}")]
    BadLine(String),
}