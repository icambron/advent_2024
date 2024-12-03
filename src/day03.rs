use crate::advent::Advent;
use std::fs::File;
use std::io::read_to_string;
use regex::Regex;

pub fn run(advent: Advent) {
    let file = parse_file(&advent.path()).expect("Failed to open file");
    part_1(&file);
    part_2(&file);
}

fn part_1(ops: &[Op]) {
    let sum = sum_all(ops, false);
    println!("Part 1: {}", sum);
}

fn part_2(ops: &[Op]) {
    let sum = sum_all(ops, true);
    println!("Part 2: {}", sum);
}

fn sum_all(ops: &[Op], allow_disable: bool) -> i64 {
    let mut sum = 0;
    let mut doing = true;
    for op in ops {
        match op {
            Op::Mul(op1, op2) => if doing || !allow_disable {
                sum += op1 * op2
            },
            Op::Do => doing = true,
            Op::Dont => doing = false,
        }
    }
    sum
}

fn parse_file(file: &str) -> Result<Vec<Op>, anyhow::Error> {
    let file = File::open(file)?;
    let s = read_to_string(file)?;
    let re = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))")?;
    Ok(re
        .captures_iter(&s)
        .map(|cap| {
            match cap.get(1) {
                Some(m) => {
                    match m.as_str() {
                        s if s.starts_with("mul") => {
                            let op1 = cap.get(2).unwrap().as_str().parse().unwrap();
                            let op2 = cap.get(3).unwrap().as_str().parse().unwrap();
                            Op::Mul(op1, op2)
                        }
                        "do()" => Op::Do,
                        "don't()" => Op::Dont,
                        _ => panic!("invalid state"),
                    }
                }
                None => panic!("invalid state"),
            }
        })
        .collect())
}

#[derive(Debug)]
enum Op {
    Mul(i64, i64),
    Do,
    Dont,
}
