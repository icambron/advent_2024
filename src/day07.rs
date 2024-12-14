use crate::advent::Solver;

pub struct Day07;
impl Solver for Day07 {
    type Input = Vec<Equation>;

    fn parse(&self, input: &str) -> Self::Input {
        input
            .lines()
            .map(|line| {
                let colon_split = line.split(": ").collect::<Vec<&str>>();
                let result = colon_split[0].parse::<u64>().unwrap();
                let args = colon_split[1].split(" ").map(|arg| arg.parse::<u64>().unwrap()).collect();
                Equation { result, args }
            })
            .collect()
    }

    fn part_1(&self, input: &mut Self::Input) -> u64 {
        try_combos(input, &[Op::Add, Op::Mul])
    }

    fn part_2(&self, input: &mut Self::Input) -> u64 {
        try_combos(input, &[Op::Add, Op::Mul, Op::Concat])
    }

    fn expected(&self) -> (u64, u64) {
        (21572148763543, 581941094529163)
    }

    fn name(&self) -> &'static str {
        "Bridge Repair (equations)"
    }
}

fn try_combos(equations: &[Equation], ops: &[Op]) -> u64 {
    let mut sum_ok = 0;

    for eq in equations {
        let mut stack: Vec<Entry> = Vec::new();

        for op in ops {
            stack.push(Entry {
                op,
                depth: 0,
                partial_sum: eq.result,
            });
        }

        let first_arg = eq.args[0];
        let arg_length = eq.args.len() - 1;
        let args_reverse: Vec<u64> = eq.args.iter().rev().cloned().collect();

        while let Some(entry) = stack.pop() {
            if let Some(arg) = args_reverse.get(entry.depth) {
                let partial_sum_maybe = match entry.op {
                    Op::Add => entry.partial_sum.checked_sub(*arg),
                    Op::Mul => {
                        if entry.partial_sum % *arg != 0 {
                            None
                        } else {
                            Some(entry.partial_sum / *arg)
                        }
                    }
                    Op::Concat => unconcat(entry.partial_sum, *arg),
                };

                let depth = entry.depth + 1;

                if let Some(partial_sum) = partial_sum_maybe {
                    if depth == arg_length && partial_sum == first_arg {
                        sum_ok += eq.result;
                        break;
                    } else if depth < arg_length {
                        for op in ops {
                            stack.push(Entry { op, depth, partial_sum });
                        }
                    }
                }
            }
        }
    }

    sum_ok
}

fn unconcat(concat_result: u64, concat_input: u64) -> Option<u64> {
    let concat_result = concat_result.to_string();
    let concat_input = concat_input.to_string();

    if concat_result.len() > concat_input.len() && concat_result.ends_with(&concat_input) {
        let unconccated = concat_result.split_at(concat_result.len() - concat_input.len()).0;
        Some(unconccated.parse().unwrap())
    } else {
        None
    }
}

#[derive(Debug)]
pub struct Equation {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unconcat() {
        assert_eq!(unconcat(123, 3), Some(12));
        assert_eq!(unconcat(123, 23), Some(1));
        assert_eq!(unconcat(123, 123), None);
        assert_eq!(unconcat(123, 4), None);
    }
}
