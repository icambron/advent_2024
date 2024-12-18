use std::ops::{Shl, Shr, ShrAssign};
use itertools::Itertools;
use crate::advent::{Solver};

pub struct Day17;

impl Solver for Day17 {
    type Input = Computer;

    fn parse(&self, input: &str) -> Self::Input {
        let split: Vec<&str> = input.split("\n\n").collect();
        let [reg_text, ins_txt] = split[0..2] else { panic!("Can't parse") };

        let mut reg_split = reg_text.lines();
        let reg_a: u64 = reg_split.next().unwrap()[12..].parse().unwrap();
        let reg_b: u64 = reg_split.next().unwrap()[12..].parse().unwrap();
        let reg_c: u64 = reg_split.next().unwrap()[12..].parse().unwrap();

        Computer::parse(reg_a, reg_b, reg_c, &ins_txt[9..])
    }

    fn part_1(&self, computer: &mut Self::Input) -> String {
        let output = run(computer, false);
        output.iter().join(",")
    }

    fn part_2(&self, computer: &mut Self::Input) -> String {

        let len = computer.instructions_raw.len();

        let mut solutions: Vec<u64> = vec![0];

        for i in (0..len).rev() {
            let goal = computer.instructions_raw[i];
            let mut next_solutions = vec![];

            for sol in solutions.iter() {
                let sol = sol.shl(3);
                for a in sol..sol + 8 {
                    computer.reg_a = a;
                    let output = run(computer, true);

                    if *output.first().unwrap() == goal {
                        next_solutions.push(a);
                    }
                }
            }

            solutions = next_solutions;
        }

        solutions.sort();
        solutions[0].to_string()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("1,5,0,3,7,3,0,3,1", "105981155568026")
    }

    fn name(&self) -> &'static str {
        "Chronospacial Computer"
    }
}

fn run(computer: &mut Computer, halt_at_first_output: bool) -> Vec<u64> {

    let mut instruction_pointer: u8 = 0;
    let mut output: Vec<u64> = vec![];

    while let Some(i) = computer.instructions.get(instruction_pointer as usize) {
        match i {
            Instruction::Bxl(arg) => computer.reg_b ^= *arg as u64,

            Instruction::Jnz(arg) => {
                if computer.reg_a != 0 {
                    instruction_pointer = *arg / 2;
                    continue
                }
            }

            Instruction::Bst(combo) => computer.reg_b = computer.arg(combo) & 0x7,
            Instruction::Bxc => computer.reg_b ^= computer.reg_c,
            Instruction::Out(combo) => {
                output.push(computer.arg(combo) & 0x7);
                if halt_at_first_output {
                    return output;
                }
            },
            Instruction::Adv(combo) => computer.reg_a.shr_assign(computer.arg(combo)),
            Instruction::Bdv(combo) => computer.reg_b = computer.reg_a.shr(computer.arg(combo)),
            Instruction::Cdv(combo) => computer.reg_c = computer.reg_a.shr(computer.arg(combo)),
        }

        instruction_pointer += 1;
    }

    output
}

fn parse_instructions(txt: &str) -> (Vec<Instruction>, Vec<u64>) {
    let mut ins = txt.chars()
        .filter(|c| *c != ',');


    let mut instructions = vec![];
    let mut instructions_raw = vec![];
    while let Some(i) = ins.next() {
        let arg: u8 = ins.next().unwrap() as u8 - b'0';

        instructions_raw.push((i as u8 - b'0') as u64);
        instructions_raw.push(arg as u64);

        instructions.push(match i {
            '0' => Instruction::Adv(Combo::from_u8(arg)),
            '1' => Instruction::Bxl(arg),
            '2' => Instruction::Bst(Combo::from_u8(arg)),
            '3' => Instruction::Jnz(arg),
            '4' => Instruction::Bxc,
            '5' => Instruction::Out(Combo::from_u8(arg)),
            '6' => Instruction::Bdv(Combo::from_u8(arg)),
            '7' => Instruction::Cdv(Combo::from_u8(arg)),
            _ => panic!("Unknown instruction")
        });
    }

    (instructions, instructions_raw)
}

#[derive(Debug, Clone)]
pub struct Computer {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    instructions: Vec<Instruction>,
    instructions_raw : Vec<u64>,
}

impl Computer {

    fn parse(reg_a: u64, reg_b: u64, reg_c: u64, instructions_str: &str) -> Self {
        let (instructions, instructions_raw) = parse_instructions(instructions_str);
        Computer {
            reg_a,
            reg_b,
            reg_c,
            instructions,
            instructions_raw,
        }
    }

    fn arg(&self, arg: &Combo) -> u64 {
        match arg {
            Combo::Literal(v) => *v as u64,
            Combo::RegA => self.reg_a,
            Combo::RegB => self.reg_b,
            Combo::RegC => self.reg_c,
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Adv(Combo),
    Bxl(u8),
    Bst(Combo),
    Jnz(u8),
    Bxc,
    Out(Combo),
    Bdv(Combo),
    Cdv(Combo),
}

#[derive(Debug, Clone)]
enum Combo {
    Literal(u8),
    RegA,
    RegB,
    RegC,
}

impl Combo {
    fn from_u8(v: u8) -> Self {
        match v {
            0..=3 => Self::Literal(v),
            4 => Self::RegA,
            5 => Self::RegB,
            6 => Self::RegC,
            _ => panic!("Unknown combo")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut computer = Computer::parse(0, 0, 9, "2,6");
        run(&mut computer, false);
        assert_eq!(computer.reg_b, 1);
    }

    #[test]
    fn test_2() {
        let mut computer = Computer::parse(10, 0, 0, "5,0,5,1,5,4");
        let output = run(&mut computer, false);
        assert_eq!(output, vec![0, 1, 2]);
    }

    #[test]
    fn test_3() {
        let mut computer = Computer::parse(2024, 0, 0, "0,1,5,4,3,0");
        let output = run(&mut computer, false);
        assert_eq!(output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(computer.reg_a, 0);
    }

    #[test]
    fn test_4() {
        let mut computer = Computer::parse(0, 29, 0, "1,7");
        run(&mut computer, false);
        assert_eq!(computer.reg_b, 26);
    }

    #[test]
    fn test_5() {
        let mut computer = Computer::parse(0, 2024, 43690, "4,0");
        run(&mut computer, false);
        assert_eq!(computer.reg_b, 44354);
    }
}