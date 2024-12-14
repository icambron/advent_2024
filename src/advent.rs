use std::fs;
use std::time::Duration;

pub enum Advent {
    Day(Day, bool),
    All(bool),
}

impl Advent {
    pub fn parse_args() -> Self {
        let mut pargs = pico_args::Arguments::from_env();

        let day = pargs.opt_value_from_str(["-d", "--day"]).unwrap();
        let part = pargs.opt_value_from_str(["-p", "--part"]).unwrap();
        let all = pargs.contains(["-a", "--all"]);
        let check = pargs.contains(["-c", "--check"]);
        let sample = pargs.contains(["-s", "--sample"]);

        if sample && (check || all) {
            panic!("Cannot use -s with -c or -a")
        }

        if all {
            Self::All(check)
        } else if let Some(day) = day {
            Self::Day(
                Day {
                    number: day,
                    part: Part::new(part),
                    input: Input::new(sample),
                },
                check,
            )
        } else {
            panic!("Must provide either -d or -a")
        }
    }
}

pub struct Day {
    pub number: usize,
    pub input: Input,
    pub part: Part,
}

impl Day {
    fn path(&self) -> String {
        let day_str = format!("{:02}", self.number);
        match self.input {
            Input::Sample => format!("files/{}/{}.txt", "samples", day_str),
            Input::Real => format!("files/{}/{}.txt", "inputs", day_str),
        }
    }
}

pub enum Part {
    One,
    Two,
    Both,
}

impl Part {
    fn new(part: Option<u8>) -> Self {
        match part {
            Some(1) => Self::One,
            Some(2) => Self::Two,
            None => Self::Both,
            _ => panic!("Invalid part"),
        }
    }
}

pub enum Input {
    Sample,
    Real,
}

impl Input {
    fn new(is_sample: bool) -> Self {
        if is_sample {
            Self::Sample
        } else {
            Self::Real
        }
    }
}

pub struct Solution {
    pub parse_duration: Duration,
    pub part_1: Option<(u64, Duration)>,
    pub part_2: Option<(u64, Duration)>,
    pub name: &'static str
}

pub trait Solver {
    type Input;
    
    fn parse(&self, input: &str) -> Self::Input;
    fn part_1(&self, input: &mut Self::Input) -> u64;
    fn part_2(&self, input: &mut Self::Input) -> u64;
    fn expected(&self) -> (u64, u64);
    fn name(&self) -> &'static str;

}

pub trait Solvifier {
    fn solve(&self, day: Day, check: bool) -> Solution;
}

impl<S> Solvifier for S where S: Solver {
    fn solve(&self, day: Day, check: bool) -> Solution {
        let input = load_file(&day.path());

        let parse_time = std::time::Instant::now();
        let mut input = self.parse(&input);
        let parse_elapsed = parse_time.elapsed();


        match day.part {
            Part::One => {
                let time = std::time::Instant::now();
                let part_1 = self.part_1(&mut input);
                let elapsed = time.elapsed();
                if check {
                    let expected_1 = self.expected().0;
                    assert_eq!(part_1, expected_1);
                }
                
                Solution {
                    parse_duration: parse_elapsed,
                    part_1: Some((part_1, elapsed)),
                    part_2: None,
                    name: self.name()
                }
            }
            Part::Two => {
                let time = std::time::Instant::now();
                let part_2 = self.part_2(&mut input);
                let elapsed = time.elapsed();
                if check {
                    let expected_2 = self.expected().1;
                    assert_eq!(part_2, expected_2);
                }

                Solution {
                    parse_duration: parse_elapsed,
                    part_1: None,
                    part_2: Some((part_2, elapsed)),
                    name: self.name()
                }
            }
            Part::Both => {
                let time_1 = std::time::Instant::now();
                let part_1 = self.part_1(&mut input);
                let elapsed_1 = time_1.elapsed();

                let time_2 = std::time::Instant::now();
                let part_2 = self.part_2(&mut input);
                let elapsed_2 = time_2.elapsed();

                if check {
                    let (expected_1, expected_2) = self.expected();
                    assert_eq!(part_1, expected_1);
                    assert_eq!(part_2, expected_2);
                }

                Solution {
                    parse_duration: parse_elapsed,
                    part_1: Some((part_1, elapsed_1)),
                    part_2: Some((part_2, elapsed_2)),
                    name: self.name()
                }
            }
        }
    }
}

fn load_file(path: &str) -> String {
    fs::read_to_string(path).expect("Failed to read file")
}
