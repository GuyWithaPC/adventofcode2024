
#[macro_export]
macro_rules! day {
    ($title:literal) => {
        pub fn main(input: &str) {
            use std::file;
            use regex::Regex;
            let day_no = Regex::new(r"day(\d+).rs").unwrap().captures(file!()).unwrap().get(1).unwrap().as_str();
            println!("--- Day {}: {} ---", day_no.parse::<usize>().unwrap(), $title);
            println!("Part 1: {}", part_1(input));
            println!("Part 2: {}", part_2(input));
        }
    }
}