use crate::input;

fn parse_report(report: &str) -> Vec<isize> {
    report
        .split(" ")
        .map(|s| isize::from_str_radix(s, 10).unwrap_or_else(|_| panic!("Malformed input file.")))
        .collect()
}

fn check_diff(prev: isize, current: isize, total_diff: isize) -> bool {
    let diff = current - prev;
    return !(diff.abs() < 1
        || diff.abs() > 3
        || (total_diff.signum() != 0 && diff.signum() != total_diff.signum()));
}

fn is_safe(report: &Vec<isize>) -> bool {
    let mut prev: Option<isize> = None;
    let mut total_diff: isize = 0;
    for n in report {
        if let Some(m) = prev {
            if !check_diff(m, *n, total_diff) {
                return false;
            }
            prev = Some(*n);
            total_diff += n - m;
        } else {
            prev = Some(*n);
        }
    }
    return true;
}

fn is_safe_dampener(report: &Vec<isize>) -> bool {
    if is_safe(report) {
        return true;
    }
    for n in 0..report.len() {
        let mut mod_report = report.clone();
        mod_report.remove(n);
        if is_safe(&mod_report) {
            return true;
        }
    }
    return false;
}

fn part_1(data: &str) {
    let mut safe = 0;
    for report in data.lines() {
        let parsed_report = parse_report(report);
        if is_safe(&parsed_report) {
            safe += 1
        }
    }
    println!("Safe reports: {safe}");
}

fn part_2(data: &str) {
    let mut safe = 0;
    let mut idx = 0;
    for report in data.lines() {
        let parsed_report = parse_report(report);
        if is_safe_dampener(&parsed_report) {
            safe += 1
        }
        idx += 1;
    }
    println!("Safe reports: {safe}");
}

pub fn main(input_dir: &str) {
    println!("--- Day 2: Red-Nosed Reports ---");
    let data = input::load(input_dir, 2, None);
    println!("Part 1:");
    part_1(&data);
    println!("Part 2:");
    part_2(&data);
}
