pub struct Advent {
    pub day: u8,
    pub input: Input,
}

impl Advent {
    pub fn path(&self) -> String {
        let day_str = format!("{:02}", self.day);
        match self.input {
            Input::Sample => format!("files/{}/{}.txt", "samples", day_str),
            Input::Real => format!("files/{}/{}.txt", "inputs", day_str),
        }
    }

    pub fn parse_args() -> Result<Self, anyhow::Error> {
        let mut pargs = pico_args::Arguments::from_env();
        Ok(Advent {
            day: pargs.opt_value_from_str(["-d", "--day"])?.unwrap_or(1),
            input: Input::new(pargs.contains(["-s", "--sample"])),
        })
    }

    pub fn parse_args_or_panic() -> Self {
        Self::parse_args().unwrap_or_else(|e| {
            panic!("{}", e);
        })
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