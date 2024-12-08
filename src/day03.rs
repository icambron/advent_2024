use crate::advent::Solver;
use regex::Regex;

pub struct Day03;

impl Solver for Day03 {
    fn run(&self, input: &str) -> (u64, u64) {
        let file = parse(input);
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

fn parse(input: &str) -> Vec<Op> {
    let re = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();
    re.captures_iter(input)
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
