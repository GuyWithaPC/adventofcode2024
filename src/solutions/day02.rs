
crate::day!("Red-Nosed Reports");

fn part_1(data: &str) -> usize {
    data.lines().filter(|&r| is_safe(&parse_report(r))).count()
}

fn part_2(data: &str) -> usize {
    data.lines().filter(|&r| is_safe_dampener(&parse_report(r))).count()
}

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