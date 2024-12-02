use aoc_2024::{run_all, run_day};
use clap::Parser;

/// Advent of Code 2024 CLI
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    #[arg(default_value_t = String::from("inputs"), help="Folder with puzzle input files.")]
    input_dir: String,
    #[arg(
        short,
        long,
        help = "Which day to execute (runs all if not specified)."
    )]
    day: Option<u32>,
}

fn main() {
    let args = Args::parse();
    let result = match args.day {
        Some(day) => run_day(args.input_dir.as_str(), day),
        None => {
            run_all(args.input_dir.as_str());
            Ok(())
        }
    };
    if let Err(e) = result {
        println!("{:?}", e);
    }
}
