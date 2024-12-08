use std::collections::HashMap;

use itertools::Itertools;

crate::day!("Ceres Search" => {
    part_1,
    part_2
});

fn part_1(data: &str) -> usize {
    let grid = file_to_grid(data);
    let (rows, cols) = file_grid_size(data);
    (0..rows)
        .cartesian_product(0..cols)
        .filter(|(y, x)| grid[&(*y, *x)] == 'X')
        .fold(0, |count, (y, x)| {
            count
                + follow_xmas(
                    &grid,
                    (y.try_into().unwrap(), x.try_into().unwrap()),
                    (0, 0),
                    'X',
                )
        })
}

fn part_2(data: &str) -> usize {
    let grid = file_to_grid(data);
    let (rows, cols) = file_grid_size(data);
    (0..rows)
        .cartesian_product(0..cols)
        .filter(|(y, x)| grid[&(*y, *x)] == 'A')
        .filter(|(y, x)| is_xmas(&grid, ((*y).try_into().unwrap(), (*x).try_into().unwrap())))
        .count()
}

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
            (-1..=1)
                .cartesian_product(-1..=1)
                .filter(|(x, y)| *y != 0 || *x != 0)
                .fold(0, |count, (x, y)| {
                    count + follow_xmas(grid, loc, (y, x), last_char)
                })
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

crate::test_day!(
"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
",
{
    part_1 => 18,
    part_2 => 9
}
);