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

    pub fn parse_args() -> Self {
        let mut pargs = pico_args::Arguments::from_env();
        Advent {
            day: pargs.opt_value_from_str(["-d", "--day"]).expect("Should provide a --day argument").unwrap_or(1),
            input: Input::new(pargs.contains(["-s", "--sample"])),
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
