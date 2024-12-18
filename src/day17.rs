use itertools::Itertools;
use crate::advent::{Solver};

pub struct Day17;

impl Solver for Day17 {
    type Input = Computer;

    fn parse(&self, input: &str) -> Self::Input {
        let split: Vec<&str> = input.split("\n\n").collect();
        let [reg_text, ins_txt] = split[0..2] else { panic!("Can't parse") };

        let mut reg_split = reg_text.lines();
        let reg_a: i32 = reg_split.next().unwrap()[12..].parse().unwrap();
        let reg_b: i32 = reg_split.next().unwrap()[12..].parse().unwrap();
        let reg_c: i32 = reg_split.next().unwrap()[12..].parse().unwrap();
        
        Computer::parse(reg_a, reg_b, reg_c, ins_txt)
    }

    fn part_1(&self, computer: &mut Self::Input) -> String {
        let output = run(computer);
        output.iter().join(",")
    }

    fn part_2(&self, _input: &mut Self::Input) -> String {
        "".to_string()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("", "")
    }

    fn name(&self) -> &'static str {
        "Chronospacial Computer"
    }
}

fn run(computer: &mut Computer) -> Vec<i32> {

    let mut instruction_pointer: u8 = 0;
    let mut output: Vec<i32> = vec![];

    while let Some(i) = computer.instructions.get(instruction_pointer as usize) {

        // println!("instruction pointer: {}", instruction_pointer);
        // println!("instruction: {:?}", i);
        // println!("{:?}", computer);

        match i {
            Instruction::Bxl(combo) => computer.reg_b ^= *combo as i32,

            Instruction::Jnz(arg) => {
                if computer.reg_a != 0 {
                    instruction_pointer = *arg / 2;
                    continue
                }
            }

            Instruction::Bst(combo) => computer.reg_b = computer.arg(combo) % 8,

            Instruction::Bxc => computer.reg_b ^= computer.reg_c,

            Instruction::Out(combo) => output.push(computer.arg(combo) % 8),

            Instruction::Adv(combo) => computer.reg_a /= 2_i32.pow(computer.arg(combo) as u32),

            Instruction::Bdv(combo) => computer.reg_b = computer.reg_a / 2_i32.pow(computer.arg(combo) as u32),

            Instruction::Cdv(combo) => computer.reg_c = computer.reg_a / 2_i32.pow(computer.arg(combo) as u32),
        }

        instruction_pointer += 1;
    }
    
    output
}

fn parse_instructions(txt: &str) -> Vec<Instruction> {

    let mut ins = txt.chars()
        .skip("Program: ".len())
        .filter(|c| *c != ',');


    let mut instructions = vec![];
    while let Some(i) = ins.next() {
        let arg: u8 = ins.next().unwrap() as u8 - b'0';

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
    
    instructions
}

#[derive(Debug)]
pub struct Computer {
    reg_a: i32,
    reg_b: i32,
    reg_c: i32,
    instructions: Vec<Instruction>
}

impl Computer {
    
    fn parse(reg_a: i32, reg_b: i32, reg_c: i32, instructions_str: &str) -> Self {
        let instructions = parse_instructions(instructions_str);
        Computer {
            reg_a,
            reg_b,
            reg_c,
            instructions
        }
    }
    
    fn arg(&self, arg: &Combo) -> i32 {
        match arg {
            Combo::Literal(v) => *v as i32,
            Combo::RegA => self.reg_a,
            Combo::RegB => self.reg_b,
            Combo::RegC => self.reg_c,
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
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
        let mut computer = Computer::parse(0, 0, 9, "Program: 2,6");
        run(&mut computer);
        assert_eq!(computer.reg_b, 1);
        
    }

    #[test]
    fn test_2() {
        let mut computer = Computer::parse(10, 0, 0, "Program: 5,0,5,1,5,4");
        let output = run(&mut computer);
        assert_eq!(output, vec![0, 1, 2]);
    }

    #[test]
    fn test_3() {
        let mut computer = Computer::parse(2024, 0, 0, "Program: 0,1,5,4,3,0");
        let output = run(&mut computer);
        assert_eq!(output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(computer.reg_a, 0);
    }

    #[test]
    fn test_4() {
        let mut computer = Computer::parse(0, 29, 0, "Program: 1,7");
        run(&mut computer);
        assert_eq!(computer.reg_b, 26);
    }

    #[test]
    fn test_5() {
        let mut computer = Computer::parse(0, 2024, 43690, "Program: 4,0");
        run(&mut computer);
        assert_eq!(computer.reg_b, 44354);
    }
}