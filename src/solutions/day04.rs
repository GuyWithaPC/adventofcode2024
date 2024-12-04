use std::collections::HashMap;

use crate::input;

fn file_grid_size(data: &str) -> (usize, usize) {
    (
        data.lines().map(|_| 1).sum(),
        data.lines().fold(0, |acc, x| x.len()),
    )
}

fn file_to_grid(data: &str) -> HashMap<(usize, usize), char> {
    data.lines()
        .scan(0, |row_no, row| {
            *row_no += 1;
            Some(row.chars().scan((*row_no - 1, 0), |(row_no, col_no), c| {
                *col_no += 1;
                Some(((*row_no, *col_no - 1), c))
            }))
        })
        .flatten()
        .collect()
}

fn next_xmas(c: char) -> Option<char> {
    match c {
        'X' => Some('M'),
        'M' => Some('A'),
        'A' => Some('S'),
        _ => None,
    }
}

fn follow_xmas(
    grid: &HashMap<(usize, usize), char>,
    loc: (isize, isize),
    dir: (isize, isize),
    last_char: char,
) -> usize {
    if let Some(check_char) = next_xmas(last_char) {
        if dir == (0, 0) {
            // check all directions
            let mut count = 0;
            for x in -1..=1 {
                for y in -1..=1 {
                    if x == y && y == 0 {
                        continue;
                    }
                    count += follow_xmas(grid, loc, (y, x), last_char);
                }
            }
            count
        } else {
            // check just 1 direction
            let (y, x) = (loc.0 + dir.0, loc.1 + dir.1);
            if grid.get(&(y as usize, x as usize)) == Some(&check_char) {
                follow_xmas(grid, (y, x), dir, check_char)
            } else {
                0
            }
        }
    } else {
        1
    }
}

fn part_1(data: &str) {
    let grid = file_to_grid(data);
    let (rows, cols) = file_grid_size(data);
    let mut count = 0;
    for y in 0..rows {
        for x in 0..cols {
            if grid[&(y, x)] == 'X' {
                count += follow_xmas(
                    &grid,
                    (y.try_into().unwrap(), x.try_into().unwrap()),
                    (0, 0),
                    'X',
                );
            }
        }
    }
    println!("number of XMAS: {count}");
}

fn part_2(data: &str) {}

pub fn main(input_dir: &str) {
    println!("--- Day 4: Ceres Search ---");
    let data = input::load(input_dir, 4, None);
    println!("Part 1:");
    part_1(&data);
    println!("Part 2:");
    part_2(&data);
}
