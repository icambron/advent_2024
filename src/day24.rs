use hashbrown::HashMap;
use itertools::Itertools;
use regex::Regex;

use crate::advent::Solver;

pub struct Day24;

impl Solver for Day24 {
    type Input = Circuit;

    fn parse(&self, input: &str, _is_sample: bool) -> Self::Input {
        let (state, connections) = input.split("\n\n").collect_tuple().unwrap();

        let state_regex = Regex::new(r"(\w+): (\d)").unwrap();
        let state = state
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| state_regex.captures(line).unwrap())
            .map(|capture| (encode(&capture[1]), &capture[2] == "1"))
            .collect();

        let connection_regex = Regex::new(r"(\w+) (\w+) (\w+) -> (\w+)").unwrap();
        let connections = connections
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| connection_regex.captures(line).unwrap())
            .map(|capture| Gate {
                left_wire: encode(&capture[1]),
                right_wire: encode(&capture[3]),
                op: match &capture[2] {
                    "AND" => GateType::And,
                    "OR" => GateType::Or,
                    "XOR" => GateType::Xor,
                    _ => panic!("Invalid operation: {}", &capture[1]),
                },
                to: encode(&capture[4]),
                left_value: None,
                right_value: None,
            })
            .collect();

        Circuit { state, gates: connections }
    }

    fn part_1(&self, input: &mut Self::Input) -> String {
        let mut connections: HashMap<u32, Vec<(usize, GateSide)>> = HashMap::new();
        for (idx, gate) in input.gates.iter_mut().enumerate() {
            gate.left_value = None;
            gate.right_value = None;

            connections.entry(gate.left_wire).or_default().push((idx, GateSide::Left));
            connections.entry(gate.right_wire).or_default().push((idx, GateSide::Right));
        }

        let mut stack: Vec<(u32, bool)> = input.state.clone().into_iter().collect::<Vec<_>>();
        let mut result = HashMap::<u32, bool>::new();

        while let Some((wire, val)) = stack.pop() {
            if let Some(conns) = connections.get(&wire) {
                for &(idx, side) in conns {
                    let gate = &mut input.gates[idx];
                    match side {
                        GateSide::Left => gate.left_value = Some(val),
                        GateSide::Right => gate.right_value = Some(val),
                    }

                    if let (Some(left), Some(right)) = (gate.left_value, gate.right_value) {
                        let value = match gate.op {
                            GateType::And => left && right,
                            GateType::Or => left || right,
                            GateType::Xor => left ^ right,
                        };

                        if let Some(z_val) = is_z(gate.to) {
                            result.insert(z_val, value);
                        } else {
                            stack.push((gate.to, value));
                        }
                    }
                }
            }
        }

        let mut final_number: u64 = 0;
        for (idx, value) in result {
            if value {
                let shifted = 1 << idx;
                final_number |= shifted;
            }
        }

        final_number.to_string()
    }

    fn part_2(&self, _input: &mut Self::Input) -> String {
        todo!()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        todo!()
    }

    fn name(&self) -> &'static str {
        "Crossed Wires"
    }
}

fn encode(input: &str) -> u32 {
    let bytes = input.as_bytes();
    let a = bytes[0] as u32;
    let b = bytes[1] as u32;
    let c = bytes[2] as u32;
    a << 16 | b << 8 | c
}

fn is_z(input: u32) -> Option<u32> {
    if input >> 16 == 0x7a {
        let first = (((input & 0xff00) >> 8) - 0x30) * 10;
        let second = (input & 0xff) - 0x30;
        Some(first + second)
    } else {
        None
    }
}

#[derive(Debug, Clone)]
pub struct Circuit {
    pub state: Vec<(u32, bool)>,
    pub gates: Vec<Gate>,
}

#[derive(Debug, Clone)]
pub struct Gate {
    left_wire: u32,
    right_wire: u32,
    op: GateType,
    to: u32,
    left_value: Option<bool>,
    right_value: Option<bool>,
}

#[derive(Debug, Clone, Copy)]
enum GateSide {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub enum GateType {
    Or,
    Xor,
    And,
}
