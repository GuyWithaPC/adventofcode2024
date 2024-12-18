use crate::util::iterators::TruthChecks;

crate::day!("Red-Nosed Reports" => {
    part_1,
    part_2
});

fn part_1(data: &str) -> usize {
    data.lines()
        .filter(|&r| is_safe_simple(&parse_report(r)))
        .count()
}

fn part_2(data: &str) -> usize {
    data.lines()
        .filter(|&r| is_safe_dampener(&parse_report(r)))
        .count()
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

fn is_safe_simple(report: &Vec<isize>) -> bool {
    match is_safe(report) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn is_safe(report: &Vec<isize>) -> Result<(), usize> {
    let mut report_iter = report.iter();
    let first = *report_iter.by_ref().next().unwrap();
    let correct = report_iter
        .scan((first, 0), |(prev, total_diff), &current| {
            if !check_diff(*prev, current, *total_diff) {
                return None;
            }
            *total_diff += current - *prev;
            *prev = current;
            return Some(());
        })
        .count();
    if correct == report.len() - 1 {
        Ok(())
    } else {
        Err(correct)
    }
}

fn is_safe_dampener(report: &Vec<isize>) -> bool {
    match is_safe(report) {
        Ok(_) => true,
        Err(idx) => ((idx.checked_sub(1).unwrap_or(0))..(std::cmp::min(idx + 2, report.len())))
            .into_iter()
            .map(|i| {
                let mut mod_report = report.clone();
                mod_report.remove(i);
                is_safe_simple(&mod_report)
            })
            .any_true(),
    }
}

crate::test_day!(
"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
",
{
    part_1 => 2,
    part_2 => 4
}
);
