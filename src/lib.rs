
pub mod solutions;
mod input;

use thiserror::Error;

#[derive(Error,Debug)]
pub enum Error {
    #[error("Day not yet implemented: {}.", .0)]
    DayNotImplemented(u32)
}

pub fn run_day(input_dir: &str, day: u32) -> Result<(), Error> {
    match day {
        1 => {
            solutions::day01::main(input_dir);
            Ok(())
        },
        _ => Err(Error::DayNotImplemented(day))
    }
}

pub fn run_all(input_dir: &str) {
    for day in 1..=25 {
        match run_day(input_dir, day) {
            Ok(()) => continue,
            Err(Error::DayNotImplemented(_)) => break
        }
    }
}