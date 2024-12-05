crate::day!("Historian Hysteria");

use itertools::Itertools;

impl Part1 for Day {
    type Output = isize;
    fn run(data: &str) -> isize {
        let (mut list_a, mut list_b) = split_lists(data);
        list_a.sort();
        list_b.sort();
        list_a
            .iter()
            .zip(list_b.iter())
            .fold(0isize, |acc, (&a, &b)| acc + (a - b).abs())
    }
}

impl Part2 for Day {
    type Output = isize;
    fn run(data: &str) -> isize {
        let (left, right) = split_lists(data);
        left.iter().unique().fold(0, |acc, current| {
            acc + (right.iter().filter(|&x| x.eq(current)).count() as isize) * *current
        })
    }
}

fn split_lists(data: &str) -> (Vec<isize>, Vec<isize>) {
    let mut list_a = Vec::new();
    let mut list_b = Vec::new();
    for row in data.lines() {
        let (new_a, new_b) = row
            .split_once("   ")
            .unwrap_or_else(|| panic!("Malformed input file."));
        list_a.push(
            isize::from_str_radix(new_a, 10).unwrap_or_else(|_| panic!("Malformed input file.")),
        );
        list_b.push(
            isize::from_str_radix(new_b, 10).unwrap_or_else(|_| panic!("Malformed input file.")),
        );
    }
    return (list_a, list_b);
}
