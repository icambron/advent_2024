use crate::advent::Advent;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead};

pub fn run(advent: Advent) {
    let (list_a, list_b) = parse_file(&advent.path());
    part_1(&list_a, &list_b);
    part_2(&list_a, &list_b);
}

fn part_1(list_a: &[i32], list_b: &[i32]) {
    let sum = list_a
        .iter()
        .zip(list_b.iter())
        .fold(0, |sum, (a, b)| sum + (b - a).abs());
    
    println!("Part 1: {}", sum);
}

fn part_2(list_a: &[i32], list_b: &[i32]) {
    let mut hash = BTreeMap::new();
    for n in list_b.iter() {
        hash.entry(n).and_modify(|e| *e += 1).or_insert(1);
    }

    let sum_similarity = list_a.iter().fold(0, |sum, val| {
        let similarity = hash.get(val).unwrap_or(&0);
        sum + similarity * val
    });

    println!("Part 2: {}", sum_similarity);
}

fn parse_file(file: &str) -> (Vec<i32>, Vec<i32>) {
    let file = File::open(file).expect("Should be able to open file");
    let lines = io::BufReader::new(file).lines();
    let mut list_a = Vec::with_capacity(1000);
    let mut list_b = Vec::with_capacity(1000);

    for line in lines.map_while(|l| l.ok()) {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Text should be a number"))
            .collect();
        if numbers.len() == 2 {
            list_a.push(numbers[0]);
            list_b.push(numbers[1]);
        } else {
            panic!("Invalid input");
        }
    }

    list_a.sort_unstable();
    list_b.sort_unstable();

    (list_a, list_b)
}
