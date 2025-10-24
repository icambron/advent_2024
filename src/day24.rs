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

    fn part_2(&self, input: &mut Self::Input) -> String {
        let mut connections: HashMap<u32, Vec<&Gate>> = HashMap::new();
        for gate in &input.gates {
            connections.entry(gate.left_wire).or_default().push(gate);
            connections.entry(gate.right_wire).or_default().push(gate);
        }

        // println!("Connections: {:?}", connections);

        // assume blithly that there's no issue in the first bit, but we still need the carry wire
        let and = find_gate(&connections, encode("x00"), GateType::And).unwrap();
        let mut prev_carry_wire = and.to;

        let mut bad_wires = Vec::new();

        for i in 1..45 {
            let mut bad_this_iteration = None;
            println!("i: {}", format!("x{:02}", i));
            let x_wire = encode(&format!("x{:02}", i));

            // we're going to assume xn and yn are never swapped with anything
            let xor1 = find_gate(&connections, x_wire, GateType::Xor).unwrap();
            let and1 = find_gate(&connections, x_wire, GateType::And).unwrap();

            // also assume the incoming carry isn't swapped, because if it is, we'll find it the round before and provide the swapped one
            println!("Carry wire: {}", decode(prev_carry_wire));
            let xor2 = find_gate(&connections, prev_carry_wire, GateType::Xor).unwrap();
            let and2 = find_gate(&connections, prev_carry_wire, GateType::And).unwrap();

            // whatever xor1's output is, it had better be hooked up to xor2
            if xor2.left_wire == prev_carry_wire && xor2.right_wire != xor1.to
                || xor2.right_wire == prev_carry_wire && xor2.left_wire != xor1.to
            {
                bad_wires.push(xor1.to);
                bad_this_iteration = Some(xor1.to);
            }

            // xor2 produces the output value, so if it's not an output, it's wrong
            if is_z(xor2.to).is_none() {
                bad_wires.push(xor2.to);
                bad_this_iteration = Some(xor2.to);
            }

            // the last one doesn't have a carry
            if i == 44 {
                continue;
            }

            let or = match find_gate(&connections, and2.to, GateType::Or) {
                None => {
                    bad_wires.push(and2.to);
                    bad_this_iteration = Some(and2.to);

                    // if we couldn't find the or from and2, then we need to find it from and1's output; they can't both be wrong
                    find_gate(&connections, and1.to, GateType::Or).unwrap()
                }
                Some(or) => {
                    // make sure and1 is the other side of it
                    if or.left_wire == and2.to && or.right_wire != and1.to || or.right_wire == and2.to && or.left_wire != and1.to {
                        bad_wires.push(and1.to);
                        bad_this_iteration = Some(and1.to);
                    }
                    or
                }
            };

            if is_z(or.to).is_some() {
                bad_wires.push(or.to);
                prev_carry_wire = bad_this_iteration.unwrap();
            } else {
                prev_carry_wire = or.to;
            }
        }

        bad_wires.into_iter().map(decode).sorted().join(",")
    }

    fn expected(&self) -> (&'static str, &'static str) {
        todo!()
    }

    fn name(&self) -> &'static str {
        "Crossed Wires"
    }
}

fn find_gate<'a>(connections: &'a HashMap<u32, Vec<&'a Gate>>, one_side: u32, t: GateType) -> Option<&'a Gate> {
    connections
        .get(&one_side)
        .into_iter()
        .flatten()
        .find(|&gate| (gate.left_wire == one_side || gate.right_wire == one_side) && gate.op == t)
        .copied()
}

fn encode(input: &str) -> u32 {
    let bytes = input.as_bytes();
    let a = bytes[0] as u32;
    let b = bytes[1] as u32;
    let c = bytes[2] as u32;
    a << 16 | b << 8 | c
}

fn decode(input: u32) -> String {
    let a = ((input >> 16) & 0xff) as u8 as char;
    let b = ((input >> 8) & 0xff) as u8 as char;
    let c = (input & 0xff) as u8 as char;
    format!("{}{}{}", a, b, c)
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GateType {
    Or,
    Xor,
    And,
}
