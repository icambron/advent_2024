use regex::Regex;
use crate::advent::Solver;

pub struct Day13;

impl Solver for Day13 {
    fn run(&self, input: &str) -> (u64, u64) {
        let parsed = parse(input);
        let p1 = solve(&parsed, |prize| prize);
        let p2 = solve(&parsed, |prize| prize + 10000000000000);
        
        (p1, p2)
    }

    fn expected(&self) -> (u64, u64) {
        (29023, 96787395375634)
    }
}

fn solve<F: Fn(i64) -> i64>(parsed: &[Machine], f: F) -> u64  {

    let mut a_total = 0;
    let mut b_total = 0;

    for machine in parsed {
        
        let prize_x = f(machine.prize_x);
        let prize_y = f(machine.prize_y);
        
        let denom = (machine.button_a.y * machine.button_b.x) - (machine.button_a.x * machine.button_b.y);
        
        assert_ne!(denom, 0, "we only handle linearly independent buttons");
        
        let num = (prize_y * machine.button_b.x) - (prize_x * machine.button_b.y);
        let a_rem = num % denom;
        
        if a_rem == 0 {
            let a_count = num / denom;
            a_total += a_count;
            b_total += (prize_x - machine.button_a.x * a_count)/machine.button_b.x;
        }
    }

    (a_total * 3 + b_total) as u64
}

fn parse(input: &str) -> Vec<Machine> {
    let button_re = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
    let prize_re = Regex::new(r"X=(\d+), Y=(\d+)").unwrap();
    input.split("\n\n")
        .map(|group| {
            let mut lines = group.lines();
            let button_a = button_re.captures(lines.next().unwrap()).unwrap();
            let button_b = button_re.captures(lines.next().unwrap()).unwrap();
            let prize = prize_re.captures(lines.next().unwrap()).unwrap();
            
            Machine {
                button_a: Button { x: button_a[1].parse().unwrap(), y: button_a[2].parse().unwrap() },
                button_b: Button { x: button_b[1].parse().unwrap(), y: button_b[2].parse().unwrap() },
                prize_x: prize[1].parse().unwrap(),
                prize_y: prize[2].parse().unwrap()
            }
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Machine {
    button_a: Button,
    button_b: Button,
    prize_x: i64,
    prize_y: i64
}

#[derive(Debug, Clone)]
struct Button {
    x: i64,
    y: i64
}