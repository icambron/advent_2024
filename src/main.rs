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
mod day14;

use crate::advent::{Advent, Solution, Solvifier};
use advent::Day;
use prettytable::{format, row, Table};
use std::collections::BTreeMap;
use std::time::Duration;

fn main() {
    match Advent::parse_args() {
        Advent::Day(day, check) => run_one(day, check),
        Advent::All(check) => run_all(check),
    }
}

fn run_all(check: bool) {
    let mut times: BTreeMap<usize, Solution> = BTreeMap::new();
    for (i, solver) in days().iter().enumerate() {
        let number = i + 1;

        let day = Day {
            number,
            part: advent::Part::Both,
            input: advent::Input::Real,
        };

        let solution = solver.solve(day, check);
        times.insert(number, solution);
    }

    let total: Duration = times
        .values()
        .map(|sol| sol.parse_duration + sol.part_1.unwrap().1 + sol.part_2.unwrap().1)
        .sum();

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row!["Day", "Name", "Parse (µs)", "Part 1 (µs)", "Part 2 (µs)", "Total (µs)"]);

    for (day, sol) in times {
        table.add_row(row![
            r -> day,
            l -> sol.name,
            r -> sol.parse_duration.as_micros(),
            r -> sol.part_1.unwrap().1.as_micros(),
            r -> sol.part_2.unwrap().1.as_micros(),
            r -> sol.parse_duration.as_micros() + sol.part_1.unwrap().1.as_micros() + sol.part_2.unwrap().1.as_micros()
        ]);
    }

    table.printstd();
    println!("Total: {:?}", total);
}

fn run_one(day: Day, check: bool) {
    let days = days();
    let solver = days.get(day.number - 1).expect("Day not found");

    let sol = solver.solve(day, check);

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    table.add_row(row!["Parse", "", format!("{:?}", sol.parse_duration)]);
    table.set_titles(row!["Part", "Result", "Time"]);

    if let Some((part_1, elapsed)) = sol.part_1 {
        table.add_row(row![r -> "1", part_1, format!("{:?}", elapsed)]);
    }

    if let Some((part_2, elapsed)) = sol.part_2 {
        table.add_row(row![r -> "2", part_2, format!("{:?}", elapsed)]);
    }

    table.printstd();
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
        &day14::Day14,
    ]
}
