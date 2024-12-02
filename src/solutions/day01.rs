
use crate::input;

fn split_lists(data: String) -> (Vec<isize>, Vec<isize>) {
    let mut list_a = Vec::new();
    let mut list_b = Vec::new();
    for row in data.lines() {
        let (new_a, new_b) = row.split_once("   ").unwrap_or_else(|| panic!("Malformed input file."));
        list_a.push(isize::from_str_radix(new_a, 10).unwrap_or_else(|_| panic!("Malformed input file.")));
        list_b.push(isize::from_str_radix(new_b, 10).unwrap_or_else(|_| panic!("Malformed input file.")));
    }
    return (list_a, list_b)
}

pub fn main(input_dir: &str) {
    println!("Day 1: Historian Hysteria");
    let data = input::load(input_dir, 1, None);
    let (mut list_a, mut list_b) = split_lists(data);
    list_a.sort();
    list_b.sort();
    let mut total_distance = 0;
    for i in 0..list_a.len() {
        total_distance += (list_a[i] - list_b[i]).abs();
    }
    println!("Total difference (part 1): {total_distance}")
}