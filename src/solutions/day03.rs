use regex::Regex;

use crate::input;

fn strtup_to_int(strtup: &str) -> (isize, isize) {
    println!("{strtup}");
    let strnums = strtup.split_once(",").unwrap();
    return (
        isize::from_str_radix(strnums.0, 10).unwrap(),
        isize::from_str_radix(strnums.1, 10).unwrap(),
    );
}

fn part_1(data: &str) {
    let mul_matcher = Regex::new(r"mul\((\d+,\d+)\)").unwrap();
    let mul_instrs: Vec<(isize, isize)> = mul_matcher
        .captures_iter(data)
        .map(|tuple| {
            let (_, [nums]) = tuple.extract();
            return strtup_to_int(nums);
        })
        .collect();
    let result: isize = mul_instrs.iter().map(|(x, y)| x * y).sum();
    println!("added results: {result}");
}

pub fn main(input_dir: &str) {
    println!("--- Day 2: Red-Nosed Reports ---");
    let data = input::load(input_dir, 3, None);
    println!("Part 1:");
    part_1(&data);
}
