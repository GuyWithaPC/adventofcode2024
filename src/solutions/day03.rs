use regex::Regex;

use crate::input;

fn strtup_to_int(strtup: &str) -> (isize, isize) {
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

fn part_2(data: &str) {
    let instr_matcher = Regex::new(r"(mul|don't|do)\((\d+,\d+|)\)").unwrap();
    let instrs: Vec<(isize, isize)> = instr_matcher
        .captures_iter(data)
        .scan(true, |enabled, instr| {
            let (_, [func, nums]) = instr.extract();
            match func {
                "mul" => {
                    if *enabled {
                        Some(strtup_to_int(nums))
                    } else {
                        Some((0, 0))
                    }
                }
                "do" => {
                    *enabled = true;
                    Some((0, 0))
                }
                _ => {
                    // this is the "don't" branch
                    *enabled = false;
                    Some((0, 0))
                }
            }
        })
        .collect();
    let result: isize = instrs.iter().map(|(x, y)| x * y).sum();
    println!("added results: {result}");
}

pub fn main(input_dir: &str) {
    println!("--- Day 3: Mull It Over ---");
    let data = input::load(input_dir, 3, None);
    println!("Part 1:");
    part_1(&data);
    println!("Part 2:");
    part_2(&data);
}
