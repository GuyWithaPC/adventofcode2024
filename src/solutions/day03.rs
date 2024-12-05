crate::day!("Mull It Over");

use regex::Regex;

impl Part1 for Day {
    type Output = usize;
    fn run(data: &str) -> usize {
        let mul_matcher = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        mul_matcher.captures_iter(data).fold(0, |sum, capture| {
            let (_, [a, b]) = capture.extract();
            sum + a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap()
        })
    }
}

impl Part2 for Day {
    type Output = usize;
    fn run(data: &str) -> usize {
        let instr_matcher = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();
        instr_matcher
            .captures_iter(data)
            .scan(true, |enabled, capture| {
                Some(match capture[1].split_once("(").unwrap().0 {
                    "mul" => {
                        if *enabled {
                            capture[2].parse::<usize>().unwrap()
                                * capture[3].parse::<usize>().unwrap()
                        } else {
                            0
                        }
                    }
                    "do" => {
                        *enabled = true;
                        0
                    }
                    _ => {
                        *enabled = false;
                        0
                    }
                })
            })
            .sum()
    }
}
