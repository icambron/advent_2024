use std::str::FromStr;

pub struct Advent {
    pub part: Part,
    pub input: Input,
}

impl Advent {
    pub fn run(&self, prefix: &str, part1: fn(&str), part2: fn(&str)) {
        let full_path = format!("{}/{}", prefix, "resources");
        if self.part == Part::Part1 {
            part1(&self.input.path(&full_path));
        } else {
            part2(&self.input.path(&full_path));
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Part {
    Part1,
    Part2,
}

impl FromStr for Part {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Part::Part1),
            "2" => Ok(Part::Part2),
            _ => Err(ParseError::InvalidPart(s.to_string())),
        }
    }
}

pub enum Input {
    Sample,
    Real,
}

impl Input {
    pub fn path(&self, dir: &str) -> String {
        match self {
            Input::Sample => format!("{}/{}", dir, "sample.txt"),
            Input::Real => format!("{}/{}", dir, "input.txt"),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid part: {0}")]
    InvalidPart(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

pub fn parse_args_or_panic() -> Advent {
    parse_args().unwrap_or_else(|e| {
        panic!("{}", e);
    })
}

pub fn parse_args() -> Result<Advent, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    let sample = pargs.contains(["-s", "--sample"]);

    Ok(Advent {
        part: pargs.opt_value_from_str("--part")?.unwrap_or(Part::Part1),
        input: if sample { Input::Sample } else { Input::Real },
    })
}
