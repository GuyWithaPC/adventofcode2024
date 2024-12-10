mod input;
pub mod solutions;
pub mod util;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Day not implemented: {}.", .0)]
    DayNotImplemented(u32),
}

macro_rules! do_day {
    ($day:ident, $data:expr) => {{
        solutions::$day::main($data);
        Ok(())
    }};
}

pub fn run_day(input_dir: &str, day: u32) -> Result<(), Error> {
    match day {
        1 => do_day!(day01, &input::load(input_dir, 1)),
        2 => do_day!(day02, &input::load(input_dir, 2)),
        3 => do_day!(day03, &input::load(input_dir, 3)),
        4 => do_day!(day04, &input::load(input_dir, 4)),
        5 => do_day!(day05, &input::load(input_dir, 5)),
        6 => do_day!(day06, &input::load(input_dir, 6)),
        7 => do_day!(day07, &input::load(input_dir, 7)),
        8 => do_day!(day08, &input::load(input_dir, 8)),
        9 => do_day!(day09, &input::load(input_dir, 9)),
        10 => do_day!(day10, &input::load(input_dir, 10)),
        _ => Err(Error::DayNotImplemented(day)),
    }
}

pub fn run_all(input_dir: &str) {
    for day in 1..=25 {
        match run_day(input_dir, day) {
            Ok(()) => continue,
            Err(Error::DayNotImplemented(_)) => break,
        }
    }
}
