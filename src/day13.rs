use crate::advent::Solver;

pub struct Day13;

impl Solver for Day13 {
    type Input = Vec<Machine>;

    fn parse(&self, input: &str, _: bool) -> Self::Input {
        let mut machines = Vec::new();
        let mut lines = input.lines().filter(|line| !line.trim().is_empty());

        while let Some(button_a_line) = lines.next() {
            let button_b_line = lines.next().unwrap();
            let prize_line = lines.next().unwrap();

            let button_a = parse_coordinates(&button_a_line[10..], "X+", " Y+");
            let button_b = parse_coordinates(&button_b_line[10..], "X+", " Y+");
            let (prize_x, prize_y) = parse_coordinates(&prize_line[7..], "X=", " Y=");

            machines.push(Machine {
                button_a: Button {
                    x: button_a.0,
                    y: button_a.1,
                },
                button_b: Button {
                    x: button_b.0,
                    y: button_b.1,
                },
                prize_x,
                prize_y,
            });
        }

        machines
    }

    fn part_1(&self, input: &mut Self::Input) -> String {
        solve(input, |prize| prize).to_string()
    }

    fn part_2(&self, input: &mut Self::Input) -> String {
        solve(input, |prize| prize + 10000000000000).to_string()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("29023", "96787395375634")
    }

    fn name(&self) -> &'static str {
        "Claw Contraption"
    }
}

fn solve<F: Fn(i64) -> i64>(parsed: &[Machine], f: F) -> u64 {
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
            b_total += (prize_x - machine.button_a.x * a_count) / machine.button_b.x;
        }
    }

    (a_total * 3 + b_total) as u64
}

fn parse_coordinates(line: &str, x_prefix: &str, y_prefix: &str) -> (i64, i64) {
    let parts: Vec<_> = line.split(',').collect();
    let x = parts[0].trim_start_matches(x_prefix).parse::<i64>().unwrap();
    let y = parts[1].trim_start_matches(y_prefix).parse::<i64>().unwrap();
    (x, y)
}

#[derive(Debug)]
pub struct Machine {
    button_a: Button,
    button_b: Button,
    prize_x: i64,
    prize_y: i64,
}

#[derive(Debug)]
pub struct Button {
    x: i64,
    y: i64,
}
