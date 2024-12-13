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
mod day10;
mod day11;
mod day12;
mod day13;

use crate::advent::{Advent, Solvifier};
use advent::Day;
use std::collections::BTreeMap;
use std::time::Duration;
use prettytable::{format, row, Table};

fn main() {
    match Advent::parse_args() {
        Advent::Day(day, check) => run_one(day, check),
        Advent::All(check) => run_all(check),
    }
}

fn run_all(check: bool) {
    let mut times: BTreeMap<usize, (Duration, Duration, Duration)> = BTreeMap::new();
    for (i, solver) in days().iter().enumerate() {
        let number = i + 1;

        let day = Day {
            number,
            part: advent::Part::Both,
            input: advent::Input::Real,
        };

        let (parse, one, two) = solver.solve(day, check);
        times.insert(number, (parse, one.unwrap().1, two.unwrap().1));
    }

    let total: Duration = times.values().map(|(parse, one, two)| *parse + *one + *two).sum();
    
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row!["Day", "Parse (µs)", "Part 1 (µs)", "Part 2 (µs)"]);

    for (day, (parse, time_1, time_2)) in times {
        table.add_row(row![day, parse.as_micros(), time_1.as_micros(), time_2.as_micros()]);
    }
    
    table.printstd();
    println!("Total: {:?}", total);
}

fn run_one(day: Day, check: bool) {
    let days = days();
    let solver = days.get(day.number - 1).expect("Day not found");

    let (parse, part_1, part_2) = solver.solve(day, check);
    
    println!("Parse time: {:?}", parse);
    
    if let Some((part_1, elapsed)) = part_1 {
        println!("Part 1: {}\t{:?}", part_1, elapsed);
    }

    if let Some((part_2, elapsed)) = part_2 {
        println!("Part 2: {}\t{:?}", part_2, elapsed);
    }
}

fn days() -> Vec<&'static dyn Solvifier> {
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
        &day10::Day10,
        &day11::Day11,
        &day12::Day12,
        &day13::Day13,
    ]
}
