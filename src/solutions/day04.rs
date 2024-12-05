use std::collections::HashMap;

fn file_grid_size(data: &str) -> (usize, usize) {
    (data.lines().count(), data.lines().next().unwrap().len())
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
    println!("Number of XMAS: {count}");
}

fn is_xmas(grid: &HashMap<(usize, usize), char>, loc: (isize, isize)) -> bool {
    if let Some(pattern) = [(-1, -1), (-1, 1), (1, -1), (1, 1)]
        .iter()
        .map(|dir| {
            if let Some(c) = grid.get(&((loc.0 + dir.0) as usize, (loc.1 + dir.1) as usize)) {
                match *c {
                    'M' => Some(0),
                    'S' => Some(1),
                    _ => None,
                }
            } else {
                None
            }
        })
        .collect::<Option<Vec<usize>>>()
    {
        if pattern.len() != 4 || pattern.iter().fold(0usize, |sum, x| sum + *x) != 2usize {
            return false;
        }
        match (pattern[0], pattern[1], pattern[2], pattern[3]) {
            (0, 1, 1, 0) => false,
            (1, 0, 0, 1) => false,
            _ => true,
        }
    } else {
        false
    }
}

fn part_2(data: &str) {
    let grid = file_to_grid(data);
    let (rows, cols) = file_grid_size(data);
    let mut count = 0;
    for y in 0..rows {
        for x in 0..cols {
            if grid[&(y, x)] == 'A'
                && is_xmas(&grid, (y.try_into().unwrap(), x.try_into().unwrap()))
            {
                count += 1;
            }
        }
    }
    println!("Number of X-MAS: {count}");
}

pub fn main(input: &str) {
    println!("--- Day 4: Ceres Search ---");
    println!("Part 1:");
    part_1(input);
    println!("Part 2:");
    part_2(input);
}
