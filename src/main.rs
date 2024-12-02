pub mod advent;
mod day01;

use advent::*;


fn main() {
    let advent = parse_args_or_panic();
    match advent.day {
        1 => day01::run(advent),
        _ => eprintln!("Day {} not implemented", advent.day),
    }
}