use common::parse_args_or_panic;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let path = parse_args_or_panic().path("day_1");
    let parsed = parse_file(&path).expect("Failed to parse file");
    part_1(parsed.clone());
    part_2(parsed);
}

fn part_1(parsed: (Vec<i32>, Vec<i32>))  {
    let (mut list_a, mut list_b) = parsed;

    list_a.sort();
    list_b.sort();

    let sum = list_a.iter()
        .zip(list_b.iter())
        .fold(0, |sum, (a, b)| {
            sum + (b - a).abs()
        });

    println!("Part 1: {}", sum);
}

fn part_2(parsed: (Vec<i32>, Vec<i32>)) {
    let (list_a, list_b) = parsed;

    let mut hash = BTreeMap::new();
    for n in list_b.iter() {
        hash.entry(n)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    let sum_similarity = list_a.iter().fold(0, |sum, val| {
        let similarity = hash.get(val).unwrap_or(&0);
        sum + similarity * val
    });

    println!("Part 2: {}", sum_similarity);
}

fn parse_file(file: &str) -> Result<(Vec<i32>, Vec<i32>), Error> {
    let file = File::open(file)?;
    let lines = io::BufReader::new(file).lines();
    let mut list_a = Vec::with_capacity(1000);
    let mut list_b = Vec::with_capacity(1000);

    for line in lines.map_while(|l| l.ok()) {
        let numbers: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse::<i32>())
            .collect::<Result<_, _>>()
            .map_err(|_| Error::BadLine(line.clone()))?;
        if numbers.len() == 2 {
            list_a.push(numbers[0]);
            list_b.push(numbers[1]);
        } else {
            return Err(Error::BadLine(line));
        }
    }

    Ok((list_a, list_b))
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("Bad line: {0}")]
    BadLine(String),
}
