pub enum Advent {
    Day(Day, bool),
    All(bool),
}

impl Advent {
    pub fn parse_args() -> Self {
        let mut pargs = pico_args::Arguments::from_env();

        let day = pargs.opt_value_from_str(["-d", "--day"]).unwrap();
        let all = pargs.contains(["-a", "--all"]);
        let check = pargs.contains(["-c", "--check"]);

        if all {
            Self::All(check)
        } else if let Some(day) = day {
            Self::Day(
                Day {
                    number: day,
                    input: Input::new(pargs.contains(["-s", "--sample"])),
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
}

impl Day {
    pub fn path(&self) -> String {
        let day_str = format!("{:02}", self.number);
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

pub trait Solver {
    fn run(&self, input: &str) -> (u64, u64);
    
    fn expected(&self) -> (u64, u64);
    
    fn input(&self) -> &'static str;
    
    fn sample(&self) -> &'static str;
    
    fn solve(&self, day: Day, check: bool) -> (u64, u64) {
        let input = match day.input {
            Input::Sample => self.sample(),
            Input::Real => self.input(),
        };
        let (part_1, part_2) = self.run(input);
        
        if check {
            let (expected_1, expected_2) = self.expected();
            assert_eq!(part_1, expected_1);
            assert_eq!(part_2, expected_2);
        }
        
        (part_1, part_2)
    }
}

#[macro_export]
macro_rules! input {
    ($day:expr) => {
        include_str!(concat!("../files/inputs/", stringify!($day), ".txt"))
    };
}

#[macro_export]
macro_rules! sample {
    ($day:expr) => {
        include_str!(concat!("../files/samples/", stringify!($day), ".txt"))
    };
}