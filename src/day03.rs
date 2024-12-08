use crate::advent::{Day, Solver};
use regex::Regex;
use std::fs::File;
use std::io::read_to_string;

pub struct Day03;

impl Solver for Day03 {
    fn run(&self, day: Day) -> (u64, u64) {
        let file = parse_file(&day.path());
        let part_1 = sum_all(&file, false);
        let part_2 = sum_all(&file, true);
        (part_1, part_2)
    }

    fn expected(&self) -> (u64, u64) {
        (189600467, 107069718)
    }
}

fn sum_all(ops: &[Op], allow_disable: bool) -> u64 {
    let mut sum = 0;
    let mut doing = true;
    for op in ops {
        match op {
            Op::Mul(op1, op2) => {
                if doing || !allow_disable {
                    sum += op1 * op2
                }
            }
            Op::Do => doing = true,
            Op::Dont => doing = false,
        }
    }
    sum as u64
}

fn parse_file(file: &str) -> Vec<Op> {
    let file = File::open(file).expect("Should be able to open file");
    let s = read_to_string(file).expect("Should be able to read the file as a string");
    let re = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();
    re.captures_iter(&s)
        .map(|cap| match cap.get(1) {
            Some(m) => match m.as_str() {
                s if s.starts_with("mul") => {
                    let op1 = cap.get(2).unwrap().as_str().parse().unwrap();
                    let op2 = cap.get(3).unwrap().as_str().parse().unwrap();
                    Op::Mul(op1, op2)
                }
                "do()" => Op::Do,
                "don't()" => Op::Dont,
                _ => panic!("invalid state"),
            },
            None => panic!("invalid state"),
        })
        .collect()
}

#[derive(Debug)]
enum Op {
    Mul(i64, i64),
    Do,
    Dont,
}
