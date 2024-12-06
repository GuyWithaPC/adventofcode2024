use std::collections::{HashMap, HashSet};

crate::day!("Guard Gallivant" => {
    part_1
});

fn part_1(data: &str) -> usize {
    let map = parse_input(data);
    let ((mut col, mut row), _) = map.iter().filter(|((col, row), &token)| match token {
        Token::Guard => true,
        _ => false
    }).next().unwrap();
    let (mut dir_r, mut dir_c) = (-1, 0);
    let mut positions: HashSet<(isize, isize)> = HashSet::new();
    positions.insert((col, row));
    'outer: loop{
        match map.get(&(col + dir_c, row + dir_r)) {
            Some(Token::Obstacle) => (dir_r, dir_c) = rotate((dir_r, dir_c)),
            None => break 'outer,
            _ => {
                col += dir_c;
                row += dir_r;
                positions.insert((col, row));
            }
        }
    }
    positions.len()
}

#[derive(Clone, Copy)]
enum Token {
    Empty,
    Obstacle,
    Guard
}

fn parse_input(input: &str) -> HashMap<(isize, isize), Token> {
    input.lines().enumerate().map(|(row, l)| l.chars().enumerate().map(move |(col, c)| match c {
        '#' => ((col as isize, row as isize), Token::Obstacle),
        '^' => ((col as isize, row as isize), Token::Guard), // guard facing up
        _ => ((col as isize, row as isize), Token::Empty)
    })).flatten().collect()
}

fn rotate(dir: (isize, isize)) -> (isize, isize) {
    match dir {
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        _ => (-1, 0)
    }
}