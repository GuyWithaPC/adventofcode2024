#[macro_export]
macro_rules! day {
    ($title:literal => {$p1:ident,$p2:ident}) => {
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
            println!("Part 1: {:#?}", $p1(input));
            println!("Part 2: {:#?}", $p2(input));
        }
    };
    ($title:literal => {$p1: ident}) => {
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
            println!("Part 1: {:#?}", $p1(input));
        }
    };
    ($title:literal => {}) => {
        pub fn main(_input: &str) {
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
        }
    };
}
