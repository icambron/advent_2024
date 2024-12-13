use crate::advent::Solver;
use regex::Regex;

pub struct Day03;

impl Solver for Day03 {
    fn part_1(&self, input: &str) -> u64 {
        sum_all(&parse(input), false)
    }

    fn part_2(&self, input: &str) -> u64 {
        sum_all(&parse(input), true)
    }

    fn expected(&self) -> (u64, u64) {
        (189600467, 107069718)
    }
}

fn sum_all(ops: &[Op], can_disable: bool) -> u64 {
    let mut sum: u64 = 0;
    let mut doing = true;
    for op in ops {
        match op {
            Op::Mul(op1, op2) => {
                let more = op1 * op2;
                if doing || !can_disable {
                    sum += more;
                }
            }
            Op::Do => doing = true,
            Op::Dont => doing = false,
        }
    }

    sum
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
    Mul(u64, u64),
    Do,
    Dont,
}
