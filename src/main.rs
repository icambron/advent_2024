pub mod advent;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

use advent::Advent;

fn main() {
    let advent = Advent::parse_args();
    let time = std::time::Instant::now();
    let f = match advent.day {
        1 => day01::run,
        2 => day02::run,
        3 => day03::run,
        4 => day04::run,
        5 => day05::run,
        6 => day06::run,
        7 => day07::run,
        _ => panic!("Day {} not implemented", advent.day),
    };

    f(advent);
    println!("Time: {:?}", time.elapsed());
}
