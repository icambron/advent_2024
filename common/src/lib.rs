pub struct Advent {
    pub input: Input,
}

impl Advent {
    pub fn path(&self, day: u8) -> String {
        let day_str = format!("{:02}", day);
        match self.input {
            Input::Sample => format!("files/{}/{}.txt", "samples", day_str),
            Input::Real => format!("files/{}/{}.txt", "inputs", day_str),
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
