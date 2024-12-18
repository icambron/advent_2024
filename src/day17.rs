use std::cmp::Ordering;
use std::collections::BinaryHeap;
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
        let mut output: Vec<u64> = vec![];
        while let Some(o) = computer.run_until_output() {
            output.push(o);
        }
        output.iter().join(",")
    }

    fn part_2(&self, computer: &mut Self::Input) -> String {
        let max_index = computer.instructions_raw.len() - 1;
        let mut solutions: BinaryHeap<Sol> = BinaryHeap::from([Sol { a: 0, iteration: 0 }]);

        while let Some(sol) = solutions.pop() {
            let goal = computer.instructions_raw[max_index - sol.iteration];
            let val = sol.a.shl(3);
            for a in val..val + 8 {
                computer.reset(a);
                let output = computer.run_until_output();

                if output == Some(goal) {
                    if sol.iteration == max_index {
                        return a.to_string();
                    }

                    solutions.push(Sol { a, iteration: sol.iteration + 1 });
                }
            }
        }
        "No solution found".to_string()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("1,5,0,3,7,3,0,3,1", "105981155568026")
    }

    fn name(&self) -> &'static str {
        "Chronospacial Computer"
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Sol {
    a: u64,
    iteration: usize
}

impl PartialOrd<Self> for Sol {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Sol {
    fn cmp(&self, other: &Self) -> Ordering {
        other.a.cmp(&self.a)
    }
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
    instruction_pointer: usize,
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
            instruction_pointer: 0,
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
    
    fn reset(&mut self, reg_a: u64) {
        self.instruction_pointer = 0;
        self.reg_a = reg_a;
    }

    fn run_until_output(&mut self) -> Option<u64> {
        while let Some(i) = self.instructions.get(self.instruction_pointer) {
            match i {
                Instruction::Bxl(arg) => self.reg_b ^= *arg as u64,

                Instruction::Jnz(arg) => {
                    if self.reg_a != 0 {
                        self.instruction_pointer = (*arg / 2) as usize;
                        continue
                    }
                }

                Instruction::Bst(combo) => self.reg_b = self.arg(combo) & 0x7,
                Instruction::Bxc => self.reg_b ^= self.reg_c,
                Instruction::Out(combo) => {
                    self.instruction_pointer += 1;
                    return Some(self.arg(combo) & 0x7)
                },
                Instruction::Adv(combo) => self.reg_a.shr_assign(self.arg(combo)),
                Instruction::Bdv(combo) => self.reg_b = self.reg_a.shr(self.arg(combo)),
                Instruction::Cdv(combo) => self.reg_c = self.reg_a.shr(self.arg(combo)),
            }

            self.instruction_pointer += 1;
        }

        None
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