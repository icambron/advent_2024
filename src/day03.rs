use crate::advent::Advent;
use std::fs::File;
use std::io;
use std::io::BufRead;

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
    ops.iter()
        .fold((true, 0), |(doing, sum), m| {
            match m {
                Op::Mul(op1, op2) => if doing || !allow_disable { (doing, sum + op1 * op2) } else { (doing, sum) },
                Op::Do => (true, sum),
                Op::Dont => (false, sum)
            }
        }).1
}

fn parse_file(file: &str) -> Result<Vec<Op>, anyhow::Error> {
    let file = File::open(file)?;
    let lines = io::BufReader::new(file).lines();
    let ops = lines
        .map_while(|l| l.ok())
        .flat_map(|line| parse_line(&line))
        .collect();
    Ok(ops)
}

fn parse_line(line: &str) -> Vec<Op> {
    use regex::Regex;
    let re = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))").expect("invalid regex");
    re.captures_iter(line)
        .map(|cap| {
            if let Some(m) = cap.get(1) {
                let s = m.as_str();
                if s.starts_with("mul") {
                    let op1 = cap.get(2).unwrap().as_str().parse().expect("invalid state");
                    let op2 = cap.get(3).unwrap().as_str().parse().expect("invalid state");
                    Op::Mul(op1, op2)
                } else if s == "do()" {
                    Op::Do
                } else if s == "don't()" {
                    Op::Dont
                } else {
                    panic!("invalid state")
                }
            } else {
                panic!("invalid state")
            }
        })
        .collect()
}

#[derive(Debug)]
enum Op {
    Mul(i64, i64),
    Do,
    Dont,
}
