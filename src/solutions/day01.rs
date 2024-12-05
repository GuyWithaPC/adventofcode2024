use std::collections::HashSet;

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

fn part_1(data: &str) {
    let (mut list_a, mut list_b) = split_lists(data);
    list_a.sort();
    list_b.sort();
    let mut total_distance = 0;
    for i in 0..list_a.len() {
        total_distance += (list_a[i] - list_b[i]).abs();
    }
    println!("Total difference: {total_distance}")
}

fn part_2(data: &str) {
    let (left, right) = split_lists(data);
    let mut left_values: HashSet<isize> = HashSet::new();
    for n in &left {
        left_values.insert(*n);
    }
    let result: isize = right.into_iter().filter(|x| left_values.contains(x)).sum();
    println!("Similarity score: {result}");
}

pub fn main(input: &str) {
    println!("--- Day 1: Historian Hysteria ---");
    println!("Part 1:");
    part_1(input);
    println!("Part 2:");
    part_2(input);
}
