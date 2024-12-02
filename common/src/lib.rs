pub struct Advent {
    pub input: Input,
}

impl Advent {
    pub fn path(&self, prefix: &str) -> String {
        let dir = format!("{}/{}", prefix, "resources");
        match self.input {
            Input::Sample => format!("{}/{}", dir, "sample.txt"),
            Input::Real => format!("{}/{}", dir, "input.txt"),
        }
    }
}

pub enum Input {
    Sample,
    Real,
}

impl Input {
    pub fn new(is_sample: bool) -> Self {
        if is_sample {
            Self::Sample
        } else {
            Self::Real
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
    Ok(Advent {
        input: Input::new(pargs.contains(["-s", "--sample"])),
    })
}
