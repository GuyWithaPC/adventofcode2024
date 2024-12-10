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
            let result = $p1(input);
            println!("Part 1: {:#?}", result);
            let result = $p2(input);
            println!("Part 2: {:#?}", result);
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
            let result = $p1(input);
            println!("Part 1: {:#?}", result);
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
    ($title:literal + bars => {$p1:ident,$p2:ident}) => {
        pub fn main(input: &str) {
            use regex::Regex;
            use std::file;
            use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
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
            let bar_space = format!("--- Day {}: {} ---", day_no.parse::<usize>().unwrap(), $title).len() - "Part 1: []".len();
            let bar_style = ProgressStyle::with_template(format!("{{prefix}}[{{bar:{bar_space}.cyan}}] [{{elapsed_precise}} / {{duration_precise}}]").as_str()).unwrap()
                .progress_chars("#>-");
            let bar = ProgressBar::with_draw_target(None, ProgressDrawTarget::stdout());
            bar.set_style(bar_style.clone());
            bar.set_prefix("Part 1: ");
            let result = $p1(input, &bar);
            bar.finish_and_clear();
            println!("Part 1: {:#?}", result);
            let bar = ProgressBar::with_draw_target(None, ProgressDrawTarget::stdout());
            bar.set_style(bar_style);
            bar.set_prefix("Part 2: ");
            let result = $p2(input, &bar);
            bar.finish_and_clear();
            println!("Part 2: {:#?}", result);
        }
    };
    ($title:literal + bars => {$p1: ident}) => {
        pub fn main(input: &str) {
            use regex::Regex;
            use std::file;
            use indicatif::{ProgressBar, ProgressDrawTarget}
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
            let bar_space = format!("--- Day {}: {} ---", day_no.parse::<usize>().unwrap(), $title).len() - "Part 1: []".len();
            let bar_style = ProgressStyle::with_template(format!("{{prefix}}[{{bar:{bar_space}.cyan}}] [{{elapsed_precise}} / {{duration_precise}}]").as_str()).unwrap()
                .progress_chars("#>-");
            let bar = ProgressBar::with_draw_target(None, ProgressDrawTarget::stdout());
            bar.set_style(bar_style);
            bar.set_prefix("Part 1: ");
            let result = $p1(input, &bar);
            bar.finish();
            println!("Part 1: {:#?}", result);
        }
    };
}

#[macro_export]
macro_rules! test_day {
    ($test_string:literal, {}) => {};
    ($test_string:literal, {$p1:ident => $p1_answer:literal$(,)?}) => {
        #[cfg(test)]
        mod test {
            use super::*;
            use indoc::indoc;
            const TEST: &str = indoc! {$test_string};
            #[test]
            fn p1_test() {
                assert_eq!($p1(TEST), $p1_answer);
            }
        }
    };
    ($test_string:literal, {$p1:ident => $p1_answer:literal, $p2:ident => $p2_answer:literal}) => {
        #[cfg(test)]
        mod test {
            use super::*;
            use indoc::indoc;
            const TEST: &str = indoc! {$test_string};
            #[test]
            fn p1_test() {
                assert_eq!($p1(TEST), $p1_answer);
            }
            #[test]
            fn p2_test() {
                assert_eq!($p2(TEST), $p2_answer);
            }
        }
    };
    ($test_string:literal + bars, {$p1:ident => $p1_answer:literal$(,)?}) => {
        #[cfg(test)]
        mod test {
            use super::*;
            use indicatif::ProgressBar;
            use indoc::indoc;
            const TEST: &str = indoc! {$test_string};
            #[test]
            fn p1_test() {
                assert_eq!($p1(TEST, &ProgressBar::hidden()), $p1_answer);
            }
        }
    };
    ($test_string:literal + bars, {$p1:ident => $p1_answer:literal, $p2:ident => $p2_answer:literal}) => {
        #[cfg(test)]
        mod test {
            use super::*;
            use indicatif::ProgressBar;
            use indoc::indoc;
            const TEST: &str = indoc! {$test_string};
            #[test]
            fn p1_test() {
                assert_eq!($p1(TEST, &ProgressBar::hidden()), $p1_answer);
            }
            #[test]
            fn p2_test() {
                assert_eq!($p2(TEST, &ProgressBar::hidden()), $p2_answer);
            }
        }
    };
}
