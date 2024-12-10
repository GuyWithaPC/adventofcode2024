use regex::Regex;

crate::day!("Mull It Over" => {
    part_1,
    part_2
});

fn part_1(data: &str) -> usize {
    let mul_matcher = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    mul_matcher.captures_iter(data).fold(0, |sum, capture| {
        let (_, [a, b]) = capture.extract();
        sum + a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap()
    })
}

fn part_2(data: &str) -> usize {
    let instr_matcher = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();
    instr_matcher
        .captures_iter(data)
        .scan(true, |enabled, capture| {
            Some(match capture[1].split_once("(").unwrap().0 {
                "mul" => {
                    if *enabled {
                        capture[2].parse::<usize>().unwrap() * capture[3].parse::<usize>().unwrap()
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

crate::test_day!(
"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
{
    part_1 => 161,
    part_2 => 48
}
);
