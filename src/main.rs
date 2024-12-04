pub mod advent;
mod day01;
mod day02;
mod day03;
mod day04;

use advent::Advent;

fn main() {
    let advent = Advent::parse_args_or_panic();
    let f = match advent.day {
        1 => day01::run,
        2 => day02::run,
        3 => day03::run,
        4 => day04::run,
        _ => panic!("Day {} not implemented", advent.day),
    };

    f(advent);
}
