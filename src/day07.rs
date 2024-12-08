use crate::advent::{Day, Solver};
use std::io::BufRead;

pub struct Day07;
impl Solver for Day07 {
    fn run(&self, day: Day) -> (u64, u64) {
        let equations = parse_file(&day.path());
        let part_1 = try_combos(&equations, &[Op::Add, Op::Mul]);
        let part_2 = try_combos(&equations, &[Op::Add, Op::Mul, Op::Concat]);
        (part_1, part_2)
    }

    fn expected(&self) -> (u64, u64) {
        (21572148763543, 581941094529163)
    }
}

fn try_combos(equations: &[Equation], ops: &[Op]) -> u64 {
    let mut sum_ok = 0;

    for eq in equations {
        let mut stack: Vec<Entry> = Vec::new();

        for op in ops {
            stack.push(Entry {
                op,
                depth: 1,
                partial_sum: eq.args[0],
            });
        }

        let arg_length = eq.args.len();

        while let Some(entry) = stack.pop() {
            if let Some(arg) = eq.args.get(entry.depth) {
                let partial_sum = match entry.op {
                    Op::Add => entry.partial_sum + *arg,
                    Op::Mul => entry.partial_sum * *arg,
                    Op::Concat => concat_numbers(entry.partial_sum, *arg),
                };

                let depth = entry.depth + 1;

                if depth == arg_length && partial_sum == eq.result {
                    sum_ok += eq.result;
                    break;
                } else if depth < arg_length {
                    for op in ops {
                        stack.push(Entry {
                            op,
                            depth,
                            partial_sum,
                        });
                    }
                }
            }
        }
    }

    sum_ok
}

// this turns out to be much much faster than format + concat + parse, presumably because that allocates a string
fn concat_numbers(a: u64, b: u64) -> u64 {
    let mut multiplier = 1;
    let mut temp = b;

    while temp > 0 {
        multiplier *= 10;
        temp /= 10;
    }

    a * multiplier + b
}

fn parse_file(path: &str) -> Vec<Equation> {
    let file = std::fs::File::open(path).expect("Should be able to open file");
    let reader = std::io::BufReader::new(file);
    reader
        .lines()
        .map_while(|l| l.ok())
        .map(|line| {
            let colon_split = line.split(": ").collect::<Vec<&str>>();
            let result = colon_split[0].parse::<u64>().unwrap();
            let args = colon_split[1]
                .split(" ")
                .map(|arg| arg.parse::<u64>().unwrap())
                .collect();
            Equation { result, args }
        })
        .collect()
}

#[derive(Debug)]
struct Equation {
    result: u64,
    args: Vec<u64>,
}

#[derive(Debug)]
enum Op {
    Add,
    Mul,
    Concat,
}

#[derive(Debug)]
struct Entry<'a> {
    op: &'a Op,
    depth: usize,
    partial_sum: u64,
}
