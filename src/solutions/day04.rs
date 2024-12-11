use crate::util::{grid::Grid, positions::Vec2};

use itertools::Itertools;

crate::day!("Ceres Search" => {
    part_1,
    part_2
});

fn part_1(data: &str) -> usize {
    let grid = file_to_grid(data);
    (0..(grid.width() as isize))
        .cartesian_product(0..(grid.height() as isize))
        .filter(|(x, y)| grid.get(*x, *y) == Some(&'X'))
        .fold(0, |count, (x, y)| {
            count + follow_xmas(&grid, Vec2::new(x, y), Vec2::new(0, 0), 'X')
        })
}

fn part_2(data: &str) -> usize {
    let grid = file_to_grid(data);
    (0..(grid.width() as isize))
        .cartesian_product(0..(grid.height() as isize))
        .filter(|(x, y)| grid.get(*x, *y) == Some(&'A'))
        .filter(|(x, y)| {
            is_xmas(
                &grid,
                Vec2::new((*x).try_into().unwrap(), (*y).try_into().unwrap()),
            )
        })
        .count()
}

fn file_to_grid(data: &str) -> Grid<char> {
    data.lines()
        .map(|l| l.chars().collect::<Vec<char>>())
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

fn follow_xmas(grid: &Grid<char>, loc: Vec2<isize>, dir: Vec2<isize>, last_char: char) -> usize {
    if let Some(check_char) = next_xmas(last_char) {
        if dir == Vec2::zero() {
            // check all directions
            (-1..=1)
                .cartesian_product(-1..=1)
                .filter(|(x, y)| *y != 0 || *x != 0)
                .fold(0, |count, (x, y)| {
                    count + follow_xmas(grid, loc, Vec2::new(x, y), last_char)
                })
        } else {
            // check just 1 direction
            let new_loc = loc + dir;
            if grid.get_coord(new_loc) == Some(&check_char) {
                follow_xmas(grid, new_loc, dir, check_char)
            } else {
                0
            }
        }
    } else {
        1
    }
}

fn is_xmas(grid: &Grid<char>, loc: Vec2<isize>) -> bool {
    if let Some(pattern) = [
        Vec2::new(-1, -1),
        Vec2::new(-1, 1),
        Vec2::new(1, -1),
        Vec2::new(1, 1),
    ]
    .iter()
    .map(|dir| {
        if let Some(c) = grid.get_coord(loc + *dir) {
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
