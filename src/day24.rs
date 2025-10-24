use std::fmt::Display;

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
            .map(|capture| (Wire::new(&capture[1]), &capture[2] == "1"))
            .collect();

        let connection_regex = Regex::new(r"(\w+) (\w+) (\w+) -> (\w+)").unwrap();
        let connections = connections
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| connection_regex.captures(line).unwrap())
            .map(|capture| Gate {
                left_wire: Wire::new(&capture[1]),
                right_wire: Wire::new(&capture[3]),
                op: match &capture[2] {
                    "AND" => GateType::And,
                    "OR" => GateType::Or,
                    "XOR" => GateType::Xor,
                    _ => panic!("Invalid operation: {}", &capture[1]),
                },
                output_wire: Wire::new(&capture[4]),
                left_value: None,
                right_value: None,
            })
            .collect();

        Circuit { state, gates: connections }
    }

    fn part_1(&self, input: &mut Self::Input) -> String {
        let mut connections: HashMap<Wire, Vec<(usize, GateSide)>> = HashMap::new();
        for (idx, gate) in input.gates.iter_mut().enumerate() {
            gate.left_value = None;
            gate.right_value = None;

            connections.entry(gate.left_wire.clone()).or_default().push((idx, GateSide::Left));
            connections.entry(gate.right_wire.clone()).or_default().push((idx, GateSide::Right));
        }

        let mut stack: Vec<(Wire, bool)> = input.state.clone().into_iter().collect::<Vec<_>>();
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

                        if let Wire::Output(z_val) = &gate.output_wire {
                            result.insert(*z_val, value);
                        } else {
                            stack.push((gate.output_wire.clone(), value));
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
        let mut connections: HashMap<Wire, Vec<&Gate>> = HashMap::new();
        for gate in &input.gates {
            connections.entry(gate.left_wire.clone()).or_default().push(gate);
            connections.entry(gate.right_wire.clone()).or_default().push(gate);
        }

        // assume blithly that there's no issue in the first bit, but we still need the carry wire
        let and = find_gate(&connections, &Wire::Input('x', 0), GateType::And).unwrap();
        let mut prev_carry_wire = and.output_wire.clone();

        let mut bad_wires = Vec::new();

        for i in 1..45 {
            let mut bad_this_iteration = None;
            let x_wire = Wire::Input('x', i);

            // we're going to assume xn and yn are never swapped with anything
            let xor1 = find_gate(&connections, &x_wire, GateType::Xor).unwrap();
            let and1 = find_gate(&connections, &x_wire, GateType::And).unwrap();

            // also assume the incoming carry isn't swapped, because if it is, we'll find it the round before and provide the swapped one
            let xor2 = find_gate(&connections, &prev_carry_wire, GateType::Xor).unwrap();
            let and2 = find_gate(&connections, &prev_carry_wire, GateType::And).unwrap();

            // whatever xor1's output is, it had better be hooked up to xor2
            if xor2.left_wire == prev_carry_wire && xor2.right_wire != xor1.output_wire
                || xor2.right_wire == prev_carry_wire && xor2.left_wire != xor1.output_wire
            {
                bad_wires.push(xor1.output_wire.clone());
                bad_this_iteration = Some(xor1.output_wire.clone());
            }

            // xor2 produces the output value, so if it's not an output, it's wrong
            if !xor2.output_wire.is_output() {
                bad_wires.push(xor2.output_wire.clone());
                bad_this_iteration = Some(xor2.output_wire.clone());
            }

            // the last one doesn't have a carry
            if i == 44 {
                continue;
            }

            let or = match find_gate(&connections, &and2.output_wire, GateType::Or) {
                None => {
                    bad_wires.push(and2.output_wire.clone());
                    bad_this_iteration = Some(and2.output_wire.clone());

                    // if we couldn't find the or from and2, then we need to find it from and1's output; they can't both be wrong
                    find_gate(&connections, &and1.output_wire, GateType::Or).unwrap()
                }
                Some(or) => {
                    // make sure and1 is the other side of it
                    if or.left_wire == and2.output_wire && or.right_wire != and1.output_wire
                        || or.right_wire == and2.output_wire && or.left_wire != and1.output_wire
                    {
                        bad_wires.push(and1.output_wire.clone());
                        bad_this_iteration = Some(and1.output_wire.clone());
                    }
                    or
                }
            };

            if or.output_wire.is_output() {
                bad_wires.push(or.output_wire.clone());
                prev_carry_wire = bad_this_iteration.unwrap();
            } else {
                prev_carry_wire = or.output_wire.clone();
            }
        }

        bad_wires.into_iter().map(|x| x.to_string()).sorted().join(",")
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("42410633905894", "cqm,mps,vcv,vjv,vwp,z13,z19,z25")
    }

    fn name(&self) -> &'static str {
        "Crossed Wires"
    }
}

fn find_gate<'a>(connections: &'a HashMap<Wire, Vec<&'a Gate>>, one_side: &Wire, t: GateType) -> Option<&'a Gate> {
    connections
        .get(one_side)
        .into_iter()
        .flatten()
        .find(|&gate| (gate.left_wire == *one_side || gate.right_wire == *one_side) && gate.op == t)
        .copied()
}

#[derive(Debug, Clone)]
pub struct Circuit {
    pub state: Vec<(Wire, bool)>,
    pub gates: Vec<Gate>,
}

#[derive(Debug, Clone)]
pub struct Gate {
    left_wire: Wire,
    right_wire: Wire,
    output_wire: Wire,
    op: GateType,
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

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Wire {
    Input(char, u32),
    Output(u32),
    Intermediate(heapless::String<3>),
}

impl Wire {
    pub fn new(s: &str) -> Self {
        let first_char = s.chars().next().unwrap();

        match first_char {
            'x' | 'y' => Wire::Input(first_char, s[1..].parse().unwrap()),
            'z' => Wire::Output(s[1..].parse().unwrap()),
            _ => Wire::Intermediate(s.parse().unwrap()),
        }
    }

    pub fn is_output(&self) -> bool {
        match self {
            Wire::Input(_, _) => false,
            Wire::Output(_) => true,
            Wire::Intermediate(_) => false,
        }
    }
}

impl Display for Wire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Wire::Input(c, n) => write!(f, "{}{}", c, n),
            Wire::Output(n) => write!(f, "z{}", n),
            Wire::Intermediate(s) => write!(f, "{}", s),
        }
    }
}
