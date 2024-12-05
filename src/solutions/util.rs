#[macro_export]
macro_rules! day {
    ($title:literal) => {
        struct Day;
        trait Part1 {
            type Output;
            fn run(data: &str) -> Self::Output;
        }
        trait Part2 {
            type Output;
            fn run(data: &str) -> Self::Output;
        }
        pub fn main(input: &str) {
            use regex::Regex;
            use std::file;
            let day_no = Regex::new(r"day(\d+).rs")
                .unwrap()
                .captures(file!())
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();
            println!(
                "--- Day {}: {} ---",
                day_no.parse::<usize>().unwrap(),
                $title
            );
            println!("Part 1: {:#?}", <Day as Part1>::run(input));
            println!("Part 2: {:#?}", <Day as Part2>::run(input));
        }
    };
}
