pub mod advent;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

use crate::advent::{Advent, Solver};
use advent::Day;
use std::collections::BTreeMap;

fn main() {
     match Advent::parse_args() {
         Advent::Day(day, check) => run_one(day, check),
         Advent::All(check) => run_all(check),
     }
}

fn run_all(check: bool) {
    let mut times: BTreeMap<usize, std::time::Duration> = BTreeMap::new();
    for (i, solver) in days().iter().enumerate() {
        let number = i + 1;

        let day = Day {
            number,
            input: advent::Input::Real,
        };

        let time = std::time::Instant::now();

        solver.solve(day, check);
        times.insert(number, time.elapsed());
    }

    let total: std::time::Duration = times.values().sum();

    println!("{0: <4}| {1: <10}", "Day", "Time");
    for (day, time) in times {
        println!(" {0:02} | {1: <10?}", day, time);
    }
    
    println!("Total: {:?}", total);
}

fn run_one(day: Day, check: bool) {
    let days = days();
    let solver = days.get(day.number - 1).expect("Day not found");
    let time = std::time::Instant::now();
    let (part_1, part_2) = solver.solve(day, check);
    let elapsed = time.elapsed();

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
    println!("Time: {:?}", elapsed);
}

fn days() -> Vec<&'static dyn Solver> {
    vec![
        &day01::Day01,
        &day02::Day02,
        &day03::Day03,
        &day04::Day04,
        &day05::Day05,
        &day06::Day06,
        &day07::Day07,
        &day08::Day08,
        &day09::Day09,
    ]
}
